//! Key/value settings and persisted design-system theme tokens.

use super::{db_err, Database};
use crate::error::{AppError, Result};
use crate::theme::ThemeTokens;
use rusqlite::params;

/// Settings key under which serialized [`ThemeTokens`] are stored.
pub const THEME_TOKENS_KEY: &str = "theme_tokens";

const SELECT_SETTING: &str = "SELECT value FROM settings WHERE key = ?1";

const UPSERT_SETTING: &str = r#"
                INSERT INTO settings (key, value)
                VALUES (?1, ?2)
                ON CONFLICT(key) DO UPDATE SET value = excluded.value;
                "#;

impl Database {
    /// Gets a setting value by key.
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare(SELECT_SETTING).map_err(db_err)?;
        let mut rows = stmt.query([key]).map_err(db_err)?;

        if let Some(row) = rows.next().map_err(db_err)? {
            let val: String = row.get(0).map_err(db_err)?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    /// Sets a setting value by key.
    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn
            .execute(UPSERT_SETTING, params![key, value])
            .map_err(db_err)?;
        Ok(())
    }

    /// Loads persisted design system theme tokens from SQLite.
    pub fn load_theme_tokens(&self) -> Result<Option<ThemeTokens>> {
        if let Some(json_str) = self.get_setting(THEME_TOKENS_KEY)? {
            let tokens: ThemeTokens = serde_json::from_str(&json_str).map_err(|e| {
                AppError::DatabaseError(format!("Failed to parse theme tokens: {}", e))
            })?;
            Ok(Some(tokens))
        } else {
            Ok(None)
        }
    }

    /// Persists design system theme tokens to SQLite without hardcoding.
    pub fn save_theme_tokens(&self, tokens: &ThemeTokens) -> Result<()> {
        let json_str = serde_json::to_string(tokens).map_err(|e| {
            AppError::DatabaseError(format!("Failed to serialize theme tokens: {}", e))
        })?;
        self.set_setting(THEME_TOKENS_KEY, &json_str)
    }
}
