//! Review item and review-log rows (sb C1, items anchored on posts since sb C1b): load the
//! whole deck, upsert or delete items, append an answer. Rows that no longer parse are
//! skipped, never fatal.

use super::{collect_rows, db_err, parse_timestamp, sql_bool, Database};
use crate::domain::review::{Deck, ItemId, ReviewEntry, ReviewItem};
use crate::domain::NetworkId;
use crate::error::Result;
use rusqlite::{params, Connection, Row};

const UPSERT_ITEM: &str = r#"
                INSERT INTO review_items (id, network_id, kind, anchor_key, subject_json, tags_json, created_at, schedule_json, due, suspended, retired)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                ON CONFLICT(id) DO UPDATE SET
                    network_id = excluded.network_id,
                    kind = excluded.kind,
                    anchor_key = excluded.anchor_key,
                    subject_json = excluded.subject_json,
                    tags_json = excluded.tags_json,
                    schedule_json = excluded.schedule_json,
                    due = excluded.due,
                    suspended = excluded.suspended,
                    retired = excluded.retired;
                "#;
const SELECT_ITEMS: &str =
    "SELECT id, network_id, subject_json, tags_json, created_at, schedule_json, suspended, retired FROM review_items ORDER BY created_at, id";
const DELETE_ITEM: &str = "DELETE FROM review_items WHERE id = ?1";
const INSERT_ENTRY: &str = r#"
                INSERT INTO review_log (card_id, network_id, reviewed_at, review_day, grade, mode, entry_json)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);
                "#;
const SELECT_ENTRIES: &str = "SELECT entry_json FROM review_log ORDER BY id";

/// A JSON column as text.
fn json<T: serde::Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(db_err)
}

impl Database {
    /// Every item and logged answer.
    pub fn load_review_deck(&self) -> Result<Deck> {
        let mut stmt = self.conn.prepare(SELECT_ITEMS).map_err(db_err)?;
        let items = collect_rows(stmt.query_map([], item_from_row).map_err(db_err)?)?.into_iter().flatten().collect();
        let mut stmt = self.conn.prepare(SELECT_ENTRIES).map_err(db_err)?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(db_err)?;
        let log = collect_rows(rows)?.into_iter().filter_map(|raw| serde_json::from_str(&raw).ok()).collect();
        Ok(Deck { items, log })
    }

    /// Inserts `items` or replaces them by id, in one transaction.
    pub fn save_review_items(&self, items: &[ReviewItem]) -> Result<()> {
        let tx = self.conn.unchecked_transaction().map_err(db_err)?;
        upsert_review_items(&tx, items)?;
        tx.commit().map_err(db_err)
    }

    pub fn delete_review_item(&self, id: &ItemId) -> Result<()> {
        self.conn.execute(DELETE_ITEM, params![id.as_str()]).map_err(db_err)?;
        Ok(())
    }

    /// Appends one answer to the log.
    pub fn append_review_entry(&self, entry: &ReviewEntry) -> Result<()> {
        let values = params![
            entry.item_id.as_str(), entry.network_id.as_str(), crate::time::rfc3339(entry.reviewed_at), entry.day.to_string(),
            entry.grade.key(), entry.mode.key(), json(entry)?
        ];
        self.conn.execute(INSERT_ENTRY, values).map_err(db_err)?;
        Ok(())
    }
}

/// Inserts `items` or replaces them by id on `conn` (inside the caller's transaction).
pub(super) fn upsert_review_items(conn: &Connection, items: &[ReviewItem]) -> Result<()> {
    let mut stmt = conn.prepare(UPSERT_ITEM).map_err(db_err)?;
    for item in items {
        let anchor_key = item.subject.anchor().map(|anchor| anchor.key()).unwrap_or_default();
        let values = params![
            item.id.as_str(), item.network_id.as_str(), item.subject.key(), anchor_key, json(&item.subject)?, json(&item.tags)?,
            crate::time::rfc3339(item.created_at), json(&item.schedule)?, item.schedule.due.to_string(), sql_bool(item.suspended), sql_bool(item.retired)
        ];
        stmt.execute(values).map_err(db_err)?;
    }
    Ok(())
}

/// One item row (`None` when a JSON column no longer parses).
fn item_from_row(row: &Row<'_>) -> rusqlite::Result<Option<ReviewItem>> {
    let text = |i: usize| row.get::<_, String>(i);
    let (id, network, subject, tags, created, schedule) = (text(0)?, text(1)?, text(2)?, text(3)?, text(4)?, text(5)?);
    let (suspended, retired): (i64, i64) = (row.get(6)?, row.get(7)?);
    let (Ok(subject), Ok(schedule)) = (serde_json::from_str(&subject), serde_json::from_str(&schedule)) else { return Ok(None) };
    Ok(Some(ReviewItem {
        id: ItemId::new(id),
        network_id: NetworkId::new(network),
        subject,
        tags: serde_json::from_str(&tags).unwrap_or_default(),
        created_at: parse_timestamp(&created),
        schedule,
        suspended: suspended != 0,
        retired: retired != 0,
    }))
}
