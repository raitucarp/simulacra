//! Rows of the thinking tools (sb E1): argument maps by thread root, credences by post, and
//! the remembered tension pairs by content hash. Everything is loaded once at startup and
//! upserted as the user works.

use super::{collect_rows, db_err, Database};
use crate::domain::thinking::argument::ArgumentMap;
use crate::domain::thinking::credence::{Credence, Outcome};
use crate::domain::thinking::tensions::{TensionRecord, TensionStatus, Verdict};
use crate::domain::PostId;
use crate::error::Result;
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{params, Row};

const UPSERT_MAP: &str = "INSERT INTO argument_maps (root_id, map_json, updated_at) VALUES (?1, ?2, ?3)
    ON CONFLICT(root_id) DO UPDATE SET map_json = excluded.map_json, updated_at = excluded.updated_at";
const SELECT_MAPS: &str = "SELECT map_json FROM argument_maps";
const DELETE_MAP: &str = "DELETE FROM argument_maps WHERE root_id = ?1";
const UPSERT_CREDENCE: &str = "INSERT INTO credences (post_id, percent, sentence, resolves_by, note, outcome, created_at, resolved_at)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) ON CONFLICT(post_id) DO UPDATE SET percent = excluded.percent,
    sentence = excluded.sentence, resolves_by = excluded.resolves_by, note = excluded.note, outcome = excluded.outcome,
    created_at = excluded.created_at, resolved_at = excluded.resolved_at";
const SELECT_CREDENCES: &str = "SELECT post_id, percent, sentence, resolves_by, note, outcome, created_at, resolved_at FROM credences";
const DELETE_CREDENCE: &str = "DELETE FROM credences WHERE post_id = ?1";
const UPSERT_TENSION: &str = "INSERT INTO tension_pairs (pair_key, left_key, right_key, verdict, reason, status, updated_at)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) ON CONFLICT(pair_key) DO UPDATE SET left_key = excluded.left_key,
    right_key = excluded.right_key, verdict = excluded.verdict, reason = excluded.reason, status = excluded.status,
    updated_at = excluded.updated_at";
const SELECT_TENSIONS: &str = "SELECT pair_key, left_key, right_key, verdict, reason, status FROM tension_pairs";
/// Date format of the `resolves_by` column.
const DAY_FORMAT: &str = "%Y-%m-%d";

impl Database {
    /// Every stored argument map (unreadable rows are skipped).
    pub fn load_argument_maps(&self) -> Result<Vec<ArgumentMap>> {
        let mut stmt = self.conn.prepare(SELECT_MAPS).map_err(db_err)?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(db_err)?;
        Ok(collect_rows(rows)?.iter().filter_map(|json| ArgumentMap::from_json(json)).collect())
    }

    /// Inserts or replaces the map of its thread root (an empty map deletes the row).
    pub fn save_argument_map(&self, map: &ArgumentMap, now: i64) -> Result<()> {
        if map.is_empty() {
            return self.conn.execute(DELETE_MAP, params![map.root.as_str()]).map_err(db_err).map(|_| ());
        }
        self.conn.execute(UPSERT_MAP, params![map.root.as_str(), map.to_json(), now]).map_err(db_err).map(|_| ())
    }

    pub fn load_credences(&self) -> Result<Vec<Credence>> {
        let mut stmt = self.conn.prepare(SELECT_CREDENCES).map_err(db_err)?;
        let rows = stmt.query_map([], credence_from_row).map_err(db_err)?;
        Ok(collect_rows(rows)?.into_iter().flatten().collect())
    }

    pub fn save_credence(&self, credence: &Credence) -> Result<()> {
        let c = credence;
        let values = params![
            c.post_id.as_str(), c.percent, c.sentence, c.resolves_by.map(|d| d.format(DAY_FORMAT).to_string()), c.note,
            c.outcome.map(Outcome::key), c.created_at.to_rfc3339(), c.resolved_at.map(|at| at.to_rfc3339())
        ];
        self.conn.execute(UPSERT_CREDENCE, values).map_err(db_err).map(|_| ())
    }

    pub fn delete_credence(&self, post_id: &PostId) -> Result<()> {
        self.conn.execute(DELETE_CREDENCE, params![post_id.as_str()]).map_err(db_err).map(|_| ())
    }

    pub fn load_tension_records(&self) -> Result<Vec<TensionRecord>> {
        let mut stmt = self.conn.prepare(SELECT_TENSIONS).map_err(db_err)?;
        let rows = stmt.query_map([], tension_from_row).map_err(db_err)?;
        collect_rows(rows)
    }

    pub fn save_tension_record(&self, record: &TensionRecord, now: i64) -> Result<()> {
        let r = record;
        let values = params![r.pair, r.left, r.right, r.verdict.map(Verdict::key), r.reason, r.status.key(), now];
        self.conn.execute(UPSERT_TENSION, values).map_err(db_err).map(|_| ())
    }
}

fn instant(text: Option<String>) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(&text?).ok().map(|at| at.with_timezone(&Utc))
}

/// A credence row (`None` when its percent or creation time is unreadable).
fn credence_from_row(row: &Row<'_>) -> rusqlite::Result<Option<Credence>> {
    let percent: i64 = row.get(1)?;
    let due: Option<String> = row.get(3)?;
    let outcome: Option<String> = row.get(5)?;
    let Some(created_at) = instant(row.get(6)?) else { return Ok(None) };
    Ok(u8::try_from(percent).ok().map(|percent| Credence {
        post_id: PostId::from(row.get::<_, String>(0).unwrap_or_default().as_str()),
        percent,
        sentence: row.get(2).unwrap_or_default(),
        resolves_by: due.and_then(|d| NaiveDate::parse_from_str(&d, DAY_FORMAT).ok()),
        note: row.get(4).unwrap_or_default(),
        outcome: outcome.and_then(|key| Outcome::from_key(&key)),
        created_at,
        resolved_at: instant(row.get(7).unwrap_or_default()),
    }))
}

fn tension_from_row(row: &Row<'_>) -> rusqlite::Result<TensionRecord> {
    let verdict: Option<String> = row.get(3)?;
    let status: String = row.get(5)?;
    Ok(TensionRecord {
        pair: row.get(0)?,
        left: row.get(1)?,
        right: row.get(2)?,
        verdict: verdict.and_then(|key| Verdict::from_key(&key)),
        reason: row.get(4)?,
        status: TensionStatus::from_key(&status).unwrap_or_default(),
    })
}
