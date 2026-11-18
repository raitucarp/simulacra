//! Card and review-log rows (sb C1): load the whole deck, upsert or delete a card, append
//! an answer. Rows that no longer parse are skipped, never fatal.

use super::{collect_rows, db_err, parse_timestamp, sql_bool, Database};
use crate::domain::review::{Card, CardId, Deck, ReviewEntry};
use crate::domain::NetworkId;
use crate::error::Result;
use rusqlite::{params, Row};

const UPSERT_CARD: &str = r#"
                INSERT INTO review_cards (id, network_id, kind_json, source_json, tags_json, created_at, schedule_json, due, suspended)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                ON CONFLICT(id) DO UPDATE SET
                    network_id = excluded.network_id,
                    kind_json = excluded.kind_json,
                    source_json = excluded.source_json,
                    tags_json = excluded.tags_json,
                    schedule_json = excluded.schedule_json,
                    due = excluded.due,
                    suspended = excluded.suspended;
                "#;
const SELECT_CARDS: &str =
    "SELECT id, network_id, kind_json, source_json, tags_json, created_at, schedule_json, suspended FROM review_cards ORDER BY created_at, id";
const DELETE_CARD: &str = "DELETE FROM review_cards WHERE id = ?1";
const INSERT_ENTRY: &str = r#"
                INSERT INTO review_log (card_id, network_id, reviewed_at, review_day, grade, mode, entry_json)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);
                "#;
const SELECT_ENTRIES: &str = "SELECT entry_json FROM review_log ORDER BY id";

/// A JSON column as text (the row is skipped when a value cannot be encoded).
fn json<T: serde::Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(db_err)
}

impl Database {
    /// Every card and logged answer.
    pub fn load_review_deck(&self) -> Result<Deck> {
        let mut stmt = self.conn.prepare(SELECT_CARDS).map_err(db_err)?;
        let cards = collect_rows(stmt.query_map([], card_from_row).map_err(db_err)?)?.into_iter().flatten().collect();
        let mut stmt = self.conn.prepare(SELECT_ENTRIES).map_err(db_err)?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(db_err)?;
        let log = collect_rows(rows)?.into_iter().filter_map(|raw| serde_json::from_str(&raw).ok()).collect();
        Ok(Deck { cards, log })
    }

    /// Inserts `cards` or replaces them by id, in one transaction.
    pub fn save_review_cards(&self, cards: &[Card]) -> Result<()> {
        let tx = self.conn.unchecked_transaction().map_err(db_err)?;
        {
            let mut stmt = tx.prepare(UPSERT_CARD).map_err(db_err)?;
            for card in cards {
                let values = params![
                    card.id.as_str(), card.network_id.as_str(), json(&card.kind)?, json(&card.source)?, json(&card.tags)?,
                    card.created_at.to_rfc3339(), json(&card.schedule)?, card.schedule.due.to_string(), sql_bool(card.suspended)
                ];
                stmt.execute(values).map_err(db_err)?;
            }
        }
        tx.commit().map_err(db_err)
    }

    pub fn delete_review_card(&self, id: &CardId) -> Result<()> {
        self.conn.execute(DELETE_CARD, params![id.as_str()]).map_err(db_err)?;
        Ok(())
    }

    /// Appends one answer to the log.
    pub fn append_review_entry(&self, entry: &ReviewEntry) -> Result<()> {
        let values = params![
            entry.card_id.as_str(), entry.network_id.as_str(), entry.reviewed_at.to_rfc3339(), entry.day.to_string(),
            entry.grade.key(), entry.mode.key(), json(entry)?
        ];
        self.conn.execute(INSERT_ENTRY, values).map_err(db_err)?;
        Ok(())
    }
}

/// One card row (`None` when a JSON column no longer parses).
fn card_from_row(row: &Row<'_>) -> rusqlite::Result<Option<Card>> {
    let text = |i: usize| row.get::<_, String>(i);
    let (id, network, kind, source, tags, created, schedule) = (text(0)?, text(1)?, text(2)?, text(3)?, text(4)?, text(5)?, text(6)?);
    let suspended: i64 = row.get(7)?;
    let parsed = (serde_json::from_str(&kind), serde_json::from_str(&source), serde_json::from_str(&schedule));
    let (Ok(kind), Ok(source), Ok(schedule)) = parsed else { return Ok(None) };
    Ok(Some(Card {
        id: CardId::new(id),
        network_id: NetworkId::new(network),
        kind,
        source,
        tags: serde_json::from_str(&tags).unwrap_or_default(),
        created_at: parse_timestamp(&created),
        schedule,
        suspended: suspended != 0,
    }))
}
