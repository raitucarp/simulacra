//! v5 (sb C1b): the standalone flashcards of `review_cards` become review items anchored on
//! the post or vault passage they cited (`domain::review::legacy` decides what is kept);
//! free-form cards are discarded and the old table is dropped. The review log stays as it is:
//! carried items keep their ids, so their history still counts.

use super::review::upsert_review_items;
use super::{collect_rows, db_err, parse_timestamp, Database};
use crate::domain::review::legacy::{migrated_items, LegacyRow};
use crate::error::Result;
use rusqlite::Row;

/// v5: review items anchored on posts and passages.
pub(super) const REVIEW_ITEMS_VERSION: i64 = 5;

const LEGACY_TABLE_COUNT: &str = "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'review_cards'";
const SELECT_LEGACY: &str = "SELECT id, network_id, kind_json, source_json, tags_json, created_at, schedule_json, suspended FROM review_cards";
const DROP_LEGACY: &str = "DROP TABLE IF EXISTS review_cards";

fn legacy_row(row: &Row<'_>) -> rusqlite::Result<LegacyRow> {
    let text = |i: usize| row.get::<_, String>(i);
    Ok(LegacyRow {
        id: text(0)?,
        network_id: text(1)?,
        kind_json: text(2)?,
        source_json: text(3)?,
        tags_json: text(4)?,
        created_at: parse_timestamp(&text(5)?),
        schedule_json: text(6)?,
        suspended: row.get::<_, i64>(7)? != 0,
    })
}

impl Database {
    /// Moves every kept flashcard into `review_items` and drops `review_cards`.
    pub(super) fn migrate_review_items(&self) -> Result<()> {
        let tables: i64 = self.conn.query_row(LEGACY_TABLE_COUNT, [], |row| row.get(0)).map_err(db_err)?;
        if tables == 0 {
            return Ok(());
        }
        let mut stmt = self.conn.prepare(SELECT_LEGACY).map_err(db_err)?;
        let rows = collect_rows(stmt.query_map([], legacy_row).map_err(db_err)?)?;
        drop(stmt);
        upsert_review_items(&self.conn, &migrated_items(&rows))?;
        self.conn.execute_batch(DROP_LEGACY).map_err(db_err)
    }
}
