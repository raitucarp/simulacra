//! Versioned data migrations tracked with `PRAGMA user_version`.
//!
//! Column additions stay in `migrate` (idempotent `ALTER TABLE`s); this module holds
//! migrations that rewrite data and must run exactly once per vault. Each unit owns its
//! version number; [`MIGRATIONS`] lists them in order, so new ones are added as new rows.

use super::embeddings_schema::EMBEDDING_MODEL_VERSION;
use super::review_migration::REVIEW_ITEMS_VERSION;
use super::{collect_rows, db_err, Database, EMPTY_AUTHOR_SNAPSHOT};
use crate::domain::{Profile, ProfileId};
use crate::error::Result;
use std::collections::HashMap;

/// v1: posts reference their author by `author_id`; `author_json` only archives deleted authors.
pub(super) const AUTHOR_ID_VERSION: i64 = 1;
/// v2: every document has a catalogue record and a full-text index row.
pub(super) const CATALOGUE_VERSION: i64 = 2;

/// One data migration, run inside a transaction.
type MigrationStep = fn(&Database) -> Result<()>;

/// Every versioned migration, in ascending version order.
const MIGRATIONS: [(i64, MigrationStep); 4] = [
    (AUTHOR_ID_VERSION, Database::migrate_author_ids),
    (CATALOGUE_VERSION, Database::migrate_catalogue_records),
    (EMBEDDING_MODEL_VERSION, Database::migrate_embedding_model),
    (REVIEW_ITEMS_VERSION, Database::migrate_review_items),
];

/// The version a fully migrated vault is at.
pub(super) const LATEST_VERSION: i64 = MIGRATIONS[MIGRATIONS.len() - 1].0;

const READ_VERSION: &str = "PRAGMA user_version";
const WRITE_VERSION: &str = "PRAGMA user_version = ";

/// v0 → v1: take `author_id` from the embedded profile snapshot, then drop the snapshots of
/// authors that still exist as profiles (orphans keep theirs as an archive).
const MIGRATE_AUTHOR_IDS: &str = r#"
    UPDATE posts SET author_id = json_extract(author_json, '$.id')
        WHERE json_valid(author_json) AND json_type(author_json, '$.id') = 'text';
    UPDATE posts SET author_json = '{}' WHERE author_id IN (SELECT id FROM profiles);
"#;

/// Archived snapshots of authors that no longer exist as profiles.
const SELECT_ARCHIVED_AUTHORS: &str = r#"
    SELECT author_id, MAX(author_json) FROM posts
    WHERE author_json != ?1 AND author_id NOT IN (SELECT id FROM profiles)
    GROUP BY author_id
"#;

impl Database {
    /// The vault's `PRAGMA user_version`.
    pub fn schema_version(&self) -> Result<i64> {
        self.conn.query_row(READ_VERSION, [], |row| row.get(0)).map_err(db_err)
    }

    /// Runs every versioned migration newer than the vault, each in its own transaction.
    pub(super) fn migrate_versions(&self) -> Result<()> {
        let current = self.schema_version()?;
        if current >= LATEST_VERSION {
            return Ok(());
        }
        MIGRATIONS
            .iter()
            .filter(|(version, _)| *version > current)
            .try_for_each(|(version, step)| self.run_migration(*step, *version))
    }

    fn run_migration(&self, step: MigrationStep, version: i64) -> Result<()> {
        self.conn.execute_batch("BEGIN;").map_err(db_err)?;
        let outcome = step(self).and_then(|_| self.conn.execute_batch(&format!("{WRITE_VERSION}{version}; COMMIT;")).map_err(db_err));
        if outcome.is_err() {
            let _ = self.conn.execute_batch("ROLLBACK;");
        }
        outcome
    }

    fn migrate_author_ids(&self) -> Result<()> {
        self.conn.execute_batch(MIGRATE_AUTHOR_IDS).map_err(db_err)
    }

    /// Author snapshots archived on posts whose profile was deleted, by author id.
    pub fn load_archived_authors(&self) -> Result<HashMap<ProfileId, Profile>> {
        let mut stmt = self.conn.prepare(SELECT_ARCHIVED_AUTHORS).map_err(db_err)?;
        let rows = stmt
            .query_map([EMPTY_AUTHOR_SNAPSHOT], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))
            .map_err(db_err)?;
        let archived = collect_rows(rows)?
            .into_iter()
            .filter_map(|(id, json)| serde_json::from_str::<Profile>(&json).ok().map(|p| (ProfileId::new(id), p)))
            .collect();
        Ok(archived)
    }
}
