//! Emoji reactions and claps (R1): `post_reactions`, one row per post, profile and emoji.
//!
//! The table arrives with the v8 migration; a row whose count drops to zero is deleted, and a
//! post's rows go with the post.

use super::{collect_rows, db_err, parse_timestamp, Database};
use crate::domain::reaction::Reaction;
use crate::domain::{PostId, ProfileId};
use crate::error::Result;
use rusqlite::{params, Row};

/// v8: the `post_reactions` table exists.
pub(super) const REACTIONS_VERSION: i64 = 8;

const CREATE_REACTIONS: &str = r#"
    CREATE TABLE IF NOT EXISTS post_reactions (
        post_id TEXT NOT NULL,
        profile_id TEXT NOT NULL,
        emoji TEXT NOT NULL,
        count INTEGER NOT NULL DEFAULT 1,
        updated_at TEXT NOT NULL,
        PRIMARY KEY (post_id, profile_id, emoji)
    );
    CREATE INDEX IF NOT EXISTS idx_post_reactions_profile ON post_reactions(profile_id);
"#;
const UPSERT_REACTION: &str = r#"
    INSERT INTO post_reactions (post_id, profile_id, emoji, count, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)
    ON CONFLICT(post_id, profile_id, emoji) DO UPDATE SET count = excluded.count, updated_at = excluded.updated_at
"#;
const DELETE_REACTION: &str = "DELETE FROM post_reactions WHERE post_id = ?1 AND profile_id = ?2 AND emoji = ?3";
/// Every reaction to the post `?1`.
pub(super) const DELETE_REACTIONS_OF: &str = "DELETE FROM post_reactions WHERE post_id = ?1";
const SELECT_REACTIONS: &str =
    "SELECT post_id, profile_id, emoji, count, updated_at FROM post_reactions ORDER BY post_id, updated_at, profile_id, emoji";

impl Database {
    /// v7 → v8: creates `post_reactions`.
    pub(super) fn migrate_reactions_table(&self) -> Result<()> {
        self.conn.execute_batch(CREATE_REACTIONS).map_err(db_err)
    }

    /// Inserts or replaces `reaction`; a count of zero deletes the row instead.
    pub fn save_reaction(&self, reaction: &Reaction) -> Result<()> {
        if reaction.count == 0 {
            return self.delete_reaction(&reaction.post_id, &reaction.profile_id, &reaction.emoji);
        }
        let updated_at = crate::time::rfc3339(reaction.updated_at);
        let values = params![reaction.post_id.as_str(), reaction.profile_id.as_str(), reaction.emoji, reaction.count, updated_at];
        self.conn.execute(UPSERT_REACTION, values).map_err(db_err).map(|_| ())
    }

    /// Deletes one profile's reaction with `emoji` to `post_id`.
    pub fn delete_reaction(&self, post_id: &PostId, profile_id: &ProfileId, emoji: &str) -> Result<()> {
        self.conn.execute(DELETE_REACTION, params![post_id.as_str(), profile_id.as_str(), emoji]).map_err(db_err).map(|_| ())
    }

    /// Every stored reaction, grouped by post, oldest first within a post.
    pub fn load_reactions(&self) -> Result<Vec<Reaction>> {
        let mut stmt = self.conn.prepare(SELECT_REACTIONS).map_err(db_err)?;
        let rows = stmt.query_map([], reaction_from_row).map_err(db_err)?;
        collect_rows(rows)
    }
}

fn reaction_from_row(row: &Row<'_>) -> rusqlite::Result<Reaction> {
    let (post_id, profile_id): (String, String) = (row.get(0)?, row.get(1)?);
    let count: i64 = row.get(3)?;
    Ok(Reaction {
        post_id: PostId::new(post_id),
        profile_id: ProfileId::new(profile_id),
        emoji: row.get(2)?,
        count: u32::try_from(count).unwrap_or(0),
        updated_at: parse_timestamp(&row.get::<_, String>(4)?),
    })
}
