//! Key/value rows of the `settings` table, addressed by [`SettingKey`].

use super::{db_err, Database, SettingKey};
use crate::error::Result;
use rusqlite::params;

const SELECT_SETTING: &str = "SELECT value FROM settings WHERE key = ?1";

const UPSERT_SETTING: &str = r#"
                INSERT INTO settings (key, value)
                VALUES (?1, ?2)
                ON CONFLICT(key) DO UPDATE SET value = excluded.value;
                "#;

const DELETE_SETTING: &str = "DELETE FROM settings WHERE key = ?1";

impl Database {
    /// Gets a setting value by key.
    pub fn get_setting(&self, key: SettingKey) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare(SELECT_SETTING).map_err(db_err)?;
        let mut rows = stmt.query([key.as_str()]).map_err(db_err)?;

        if let Some(row) = rows.next().map_err(db_err)? {
            let val: String = row.get(0).map_err(db_err)?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    /// Sets a setting value by key.
    pub fn set_setting(&self, key: SettingKey, value: &str) -> Result<()> {
        self.conn
            .execute(UPSERT_SETTING, params![key.as_str(), value])
            .map_err(db_err)?;
        Ok(())
    }

    /// Removes a setting row (no-op when absent).
    pub fn delete_setting(&self, key: SettingKey) -> Result<()> {
        self.conn.execute(DELETE_SETTING, params![key.as_str()]).map_err(db_err)?;
        Ok(())
    }
}
