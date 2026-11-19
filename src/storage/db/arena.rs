//! Rows of the saved debates and thought experiments (sb F1): load all, upsert one, delete
//! one. Each kind names its table through [`ArenaRecord`]; rows whose JSON no longer parses
//! are skipped, never fatal.

use super::{collect_rows, db_err, Database};
use crate::domain::arena::debate::Debate;
use crate::domain::arena::lab::Experiment;
use crate::domain::arena::ArenaId;
use crate::error::Result;
use rusqlite::params;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// A saved Arena record kept in its own table.
pub trait ArenaRecord: Serialize + DeserializeOwned + Send + 'static {
    const TABLE: &'static str;
    fn id(&self) -> &ArenaId;
    /// RFC 3339 time the rows are ordered by (newest first).
    fn updated(&self) -> String;
}

impl ArenaRecord for Debate {
    const TABLE: &'static str = "arena_debates";
    fn id(&self) -> &ArenaId {
        &self.id
    }
    fn updated(&self) -> String {
        crate::time::rfc3339(self.updated_at)
    }
}

impl ArenaRecord for Experiment {
    const TABLE: &'static str = "arena_experiments";
    fn id(&self) -> &ArenaId {
        &self.id
    }
    fn updated(&self) -> String {
        crate::time::rfc3339(self.updated_at)
    }
}

impl Database {
    /// Every record of one kind, newest first.
    pub fn load_arena_records<T: ArenaRecord>(&self) -> Result<Vec<T>> {
        let sql = format!("SELECT record_json FROM {} ORDER BY updated_at DESC, id", T::TABLE);
        let mut stmt = self.conn.prepare(&sql).map_err(db_err)?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(db_err)?;
        Ok(collect_rows(rows)?.into_iter().filter_map(|raw| serde_json::from_str(&raw).ok()).collect())
    }

    /// Inserts `record` or replaces it by id.
    pub fn save_arena_record<T: ArenaRecord>(&self, record: &T) -> Result<()> {
        let sql = format!(
            "INSERT INTO {} (id, updated_at, record_json) VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET updated_at = excluded.updated_at, record_json = excluded.record_json",
            T::TABLE
        );
        let json = serde_json::to_string(record).map_err(db_err)?;
        self.conn.execute(&sql, params![record.id().as_str(), record.updated(), json]).map_err(db_err)?;
        Ok(())
    }

    /// Deletes the record `id` of one kind.
    pub fn delete_arena_record<T: ArenaRecord>(&self, id: &ArenaId) -> Result<()> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", T::TABLE);
        self.conn.execute(&sql, params![id.as_str()]).map_err(db_err)?;
        Ok(())
    }
}
