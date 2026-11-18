//! Rows of the private Journal tables (sb E3): load every record of a kind, upsert one,
//! delete one. Each kind names its table through [`StoredRecord`]; rows whose JSON no longer
//! parses are skipped, never fatal.

use super::{collect_rows, db_err, Database};
use crate::domain::journal::records::Record;
use crate::domain::journal::{FocusSession, JournalBook, JournalEntry, MoodCheckin, RecordId, TimeCapsule};
use crate::error::Result;
use chrono::NaiveDate;
use rusqlite::params;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// A record kind kept in its own table.
pub trait StoredRecord: Record + Serialize + DeserializeOwned + Send + 'static {
    const TABLE: &'static str;
    fn day(&self) -> NaiveDate;
    /// RFC 3339 time the rows are ordered by.
    fn sort_at(&self) -> String;
}

impl StoredRecord for JournalEntry {
    const TABLE: &'static str = "journal_entries";
    fn day(&self) -> NaiveDate {
        self.day
    }
    fn sort_at(&self) -> String {
        self.created_at.to_rfc3339()
    }
}

impl StoredRecord for MoodCheckin {
    const TABLE: &'static str = "mood_checkins";
    fn day(&self) -> NaiveDate {
        self.day
    }
    fn sort_at(&self) -> String {
        self.at.to_rfc3339()
    }
}

impl StoredRecord for FocusSession {
    const TABLE: &'static str = "focus_sessions";
    fn day(&self) -> NaiveDate {
        self.day
    }
    fn sort_at(&self) -> String {
        self.started_at.to_rfc3339()
    }
}

impl StoredRecord for TimeCapsule {
    const TABLE: &'static str = "time_capsules";
    fn day(&self) -> NaiveDate {
        self.opens_on
    }
    fn sort_at(&self) -> String {
        self.written_at.to_rfc3339()
    }
}

impl Database {
    /// Every record of one kind, in time order.
    pub fn load_journal_records<T: StoredRecord>(&self) -> Result<Vec<T>> {
        let sql = format!("SELECT record_json FROM {} ORDER BY sort_at, id", T::TABLE);
        let mut stmt = self.conn.prepare(&sql).map_err(db_err)?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(db_err)?;
        Ok(collect_rows(rows)?.into_iter().filter_map(|raw| serde_json::from_str(&raw).ok()).collect())
    }

    /// Every private record.
    pub fn load_journal_book(&self) -> Result<JournalBook> {
        Ok(JournalBook {
            entries: self.load_journal_records()?,
            checkins: self.load_journal_records()?,
            sessions: self.load_journal_records()?,
            capsules: self.load_journal_records()?,
        })
    }

    /// Inserts `record` or replaces it by id.
    pub fn save_journal_record<T: StoredRecord>(&self, record: &T) -> Result<()> {
        let sql = format!(
            "INSERT INTO {} (id, day, sort_at, record_json) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET day = excluded.day, sort_at = excluded.sort_at, record_json = excluded.record_json",
            T::TABLE
        );
        let json = serde_json::to_string(record).map_err(db_err)?;
        self.conn.execute(&sql, params![record.id().as_str(), record.day().to_string(), record.sort_at(), json]).map_err(db_err)?;
        Ok(())
    }

    /// Deletes the record `id` of one kind.
    pub fn delete_journal_record<T: StoredRecord>(&self, id: &RecordId) -> Result<()> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", T::TABLE);
        self.conn.execute(&sql, params![id.as_str()]).map_err(db_err)?;
        Ok(())
    }
}
