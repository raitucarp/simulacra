//! Learning rows (sb E2): goals, tutor sessions and palaces as JSON records, loaded once at
//! startup and upserted or deleted as they change. Rows that no longer parse are skipped.

use super::{collect_rows, db_err, Database};
use crate::domain::learning::{Goal, LearningRecords, Palace, TutorSession};
use crate::error::Result;
use jiff::Timestamp;
use rusqlite::params;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// One learning table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Table {
    Goals,
    Sessions,
    Palaces,
}

impl Table {
    const fn select(self) -> &'static str {
        match self {
            Self::Goals => "SELECT record_json FROM learning_goals ORDER BY rowid",
            Self::Sessions => "SELECT record_json FROM learning_tutor_sessions ORDER BY updated_at",
            Self::Palaces => "SELECT record_json FROM learning_palaces ORDER BY rowid",
        }
    }

    const fn upsert(self) -> &'static str {
        match self {
            Self::Goals => "INSERT INTO learning_goals (id, network_id, updated_at, record_json) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(id) DO UPDATE SET network_id = excluded.network_id, updated_at = excluded.updated_at, record_json = excluded.record_json",
            Self::Sessions => "INSERT INTO learning_tutor_sessions (id, network_id, updated_at, record_json) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(id) DO UPDATE SET network_id = excluded.network_id, updated_at = excluded.updated_at, record_json = excluded.record_json",
            Self::Palaces => "INSERT INTO learning_palaces (id, network_id, updated_at, record_json) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(id) DO UPDATE SET network_id = excluded.network_id, updated_at = excluded.updated_at, record_json = excluded.record_json",
        }
    }

    const fn delete(self) -> &'static str {
        match self {
            Self::Goals => "DELETE FROM learning_goals WHERE id = ?1",
            Self::Sessions => "DELETE FROM learning_tutor_sessions WHERE id = ?1",
            Self::Palaces => "DELETE FROM learning_palaces WHERE id = ?1",
        }
    }
}

impl Database {
    /// Every goal, tutor session and palace.
    pub fn load_learning(&self) -> Result<LearningRecords> {
        Ok(LearningRecords { goals: self.load_records(Table::Goals)?, sessions: self.load_records(Table::Sessions)?, palaces: self.load_records(Table::Palaces)? })
    }

    fn load_records<T: DeserializeOwned>(&self, table: Table) -> Result<Vec<T>> {
        let mut stmt = self.conn.prepare(table.select()).map_err(db_err)?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(db_err)?;
        Ok(collect_rows(rows)?.into_iter().filter_map(|raw| serde_json::from_str(&raw).ok()).collect())
    }

    fn save_record<T: Serialize>(&self, table: Table, id: &str, network: &str, record: &T) -> Result<()> {
        let json = serde_json::to_string(record).map_err(db_err)?;
        self.conn.execute(table.upsert(), params![id, network, crate::time::rfc3339(Timestamp::now()), json]).map_err(db_err).map(|_| ())
    }

    fn delete_record(&self, table: Table, id: &str) -> Result<()> {
        self.conn.execute(table.delete(), params![id]).map_err(db_err).map(|_| ())
    }

    pub fn save_goal(&self, goal: &Goal) -> Result<()> {
        self.save_record(Table::Goals, &goal.id.0, goal.network_id.as_str(), goal)
    }

    pub fn delete_goal(&self, id: &str) -> Result<()> {
        self.delete_record(Table::Goals, id)
    }

    pub fn save_tutor_session(&self, session: &TutorSession) -> Result<()> {
        self.save_record(Table::Sessions, &session.id.0, session.network_id.as_str(), session)
    }

    pub fn delete_tutor_session(&self, id: &str) -> Result<()> {
        self.delete_record(Table::Sessions, id)
    }

    pub fn save_palace(&self, palace: &Palace) -> Result<()> {
        self.save_record(Table::Palaces, &palace.id.0, palace.network_id.as_str(), palace)
    }

    pub fn delete_palace(&self, id: &str) -> Result<()> {
        self.delete_record(Table::Palaces, id)
    }
}
