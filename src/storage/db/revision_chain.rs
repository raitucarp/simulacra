//! Version chains of revisions (V1): `revisions.parent_id` and `revisions.version`.
//!
//! A new revision is numbered as the next version of its subject's chain when it is stored;
//! the v6 migration numbers the revisions stored before chains existed. Both follow
//! [`SemVer`]: per subject, oldest first, the revisions are `1.0.0`, `1.1.0`, `1.2.0`…, each
//! the child of the one before it.

use super::{db_err, Database};
use crate::domain::data::history::{Revision, RevisionId, SemVer};
use crate::error::Result;
use rusqlite::{params, OptionalExtension};

/// v6: every revision has a version and a parent.
pub(super) const REVISION_CHAIN_VERSION: i64 = 6;

/// The newest other revision of a subject, with its version and the subject's revision count.
const CHAIN_TAIL: &str = "SELECT id, version, COUNT(*) OVER () FROM revisions \
    WHERE subject_kind = ?1 AND subject_id = ?2 AND id != ?3 ORDER BY saved_at DESC, id DESC LIMIT 1";

/// Numbers every revision lacking a version by its place in its subject's chain (stored order,
/// ties broken by id, as the history lists them) and links it to the revision before it.
const BACKFILL_CHAINS: &str = r#"
    WITH chain AS (
        SELECT id,
               ROW_NUMBER() OVER subject_chain - 1 AS step,
               LAG(id) OVER subject_chain AS parent
        FROM revisions
        WINDOW subject_chain AS (PARTITION BY subject_kind, subject_id ORDER BY saved_at ASC, id ASC)
    )
    UPDATE revisions SET version = '1.' || chain.step || '.0', parent_id = chain.parent
    FROM chain WHERE chain.id = revisions.id AND revisions.version = '';
"#;

impl Database {
    /// `revision` with its place in the chain: as given when it has a version, else the
    /// version after the subject's newest revision, as that revision's child.
    pub(super) fn chained(&self, revision: &Revision) -> Result<Revision> {
        if revision.version.is_some() {
            return Ok(revision.clone());
        }
        let subject = &revision.subject;
        let tail: Option<(String, String, i64)> = self
            .conn
            .query_row(CHAIN_TAIL, params![subject.kind(), subject.id(), revision.id.as_str()], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .optional()
            .map_err(db_err)?;
        let (parent_id, version) = match tail {
            None => (None, SemVer::ORIGINAL),
            Some((id, version, count)) => {
                let after = SemVer::parse(&version).map_or(SemVer::after_edits(count as u32), SemVer::next_edit);
                (Some(RevisionId::new(id)), after)
            }
        };
        Ok(Revision { parent_id, version: Some(version), ..revision.clone() })
    }

    /// v5 → v6: numbers the revisions stored before version chains.
    pub(super) fn migrate_revision_chains(&self) -> Result<()> {
        self.conn.execute_batch(BACKFILL_CHAINS).map_err(db_err)
    }
}
