//! Full-text index of the earlier versions of posts (Q6): `revisions_fts` (FTS5), one row
//! per kept post version with text, so a search may include old versions.
//!
//! Triggers on `revisions` keep it in step; the v10 migration indexes the versions kept
//! before it existed. A sealed version (X5: a locked post's, whose `content` is only the
//! placeholder and whose text is sealed in `record_json`) is never indexed.

use super::{collect_rows, db_err, Database};
use crate::domain::PostId;
use crate::error::Result;
use rusqlite::params;

/// v10: every kept post version with open text has a row in `revisions_fts`.
pub(super) const REVISIONS_FTS_VERSION: i64 = 10;

/// The condition a `revisions` row (`r`) meets to be indexed: a post's version with text that
/// is neither the lock placeholder nor sealed.
macro_rules! indexable {
    ($r:literal) => {
        concat!($r, ".subject_kind = 'post' AND ", $r, ".content != '' AND ", $r, ".content != '\u{1F512}' AND instr(", $r, ".record_json, '\"lock\"') = 0")
    };
}

const CREATE_REVISIONS_FTS: &str = concat!(
    "CREATE VIRTUAL TABLE IF NOT EXISTS revisions_fts USING fts5(revision_id UNINDEXED, post_id UNINDEXED, content, tokenize = 'unicode61 remove_diacritics 2');",
    // `INSERT OR REPLACE` fires no delete trigger, so an insert first drops any row of its id.
    "CREATE TRIGGER IF NOT EXISTS revisions_fts_after_insert AFTER INSERT ON revisions BEGIN ",
    "DELETE FROM revisions_fts WHERE revision_id = new.id; ",
    "INSERT INTO revisions_fts (revision_id, post_id, content) SELECT new.id, new.subject_id, new.content WHERE ", indexable!("new"), "; END;",
    "CREATE TRIGGER IF NOT EXISTS revisions_fts_after_update AFTER UPDATE ON revisions BEGIN ",
    "DELETE FROM revisions_fts WHERE revision_id = old.id; ",
    "INSERT INTO revisions_fts (revision_id, post_id, content) SELECT new.id, new.subject_id, new.content WHERE ", indexable!("new"), "; END;",
    "CREATE TRIGGER IF NOT EXISTS revisions_fts_after_delete AFTER DELETE ON revisions BEGIN ",
    "DELETE FROM revisions_fts WHERE revision_id = old.id; END;",
);

const REINDEX_REVISIONS: &str = concat!(
    "DELETE FROM revisions_fts;",
    "INSERT INTO revisions_fts (revision_id, post_id, content) SELECT r.id, r.subject_id, r.content FROM revisions r WHERE ", indexable!("r"), ";",
);

/// Matching versions: the post, the version's text and its bm25 score, best first.
const SCORE_REVISIONS: &str = "SELECT post_id, content, bm25(revisions_fts) FROM revisions_fts WHERE revisions_fts MATCH ?1 \
    ORDER BY bm25(revisions_fts), revision_id LIMIT ?2";

impl Database {
    /// Creates `revisions_fts` and its triggers when missing.
    pub(super) fn create_revisions_index(&self) -> Result<()> {
        self.conn.execute_batch(CREATE_REVISIONS_FTS).map_err(db_err)
    }

    /// v9 → v10: indexes every version kept before the index existed.
    pub(super) fn migrate_revisions_index(&self) -> Result<()> {
        self.conn.execute_batch(REINDEX_REVISIONS).map_err(db_err)
    }

    /// At most `limit` earlier versions of posts matching the FTS5 `expression`, best first:
    /// the post, the version's text and its score (lower is better). A post may come more
    /// than once, one row per matching version.
    pub fn score_revisions(&self, expression: &str, limit: usize) -> Result<Vec<(PostId, String, f64)>> {
        let mut stmt = self.conn.prepare_cached(SCORE_REVISIONS).map_err(db_err)?;
        let rows = stmt
            .query_map(params![expression, limit as i64], |row| Ok((PostId::new(row.get::<_, String>(0)?), row.get::<_, String>(1)?, row.get::<_, f64>(2)?)))
            .map_err(db_err)?;
        collect_rows(rows)
    }
}
