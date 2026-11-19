//! Non-destructive column migrations for vaults created by older releases.

use super::Database;
use crate::error::{AppError, Result};

/// SQLite error fragment reported when a column already exists.
const DUPLICATE_COLUMN_MARKER: &str = "duplicate column";

/// Column additions applied to existing user vaults, in order.
const MIGRATION_STATEMENTS: [&str; 12] = [
    "ALTER TABLE posts ADD COLUMN reply_to_id TEXT;",
    "ALTER TABLE posts ADD COLUMN quote_id TEXT;",
    "ALTER TABLE posts ADD COLUMN repost_id TEXT;",
    "ALTER TABLE posts ADD COLUMN link_url TEXT;",
    "ALTER TABLE posts ADD COLUMN image_path TEXT;",
    "ALTER TABLE posts ADD COLUMN reposts_count INTEGER NOT NULL DEFAULT 0;",
    "ALTER TABLE posts ADD COLUMN is_reposted INTEGER NOT NULL DEFAULT 0;",
    "ALTER TABLE profiles ADD COLUMN network_id TEXT;",
    "ALTER TABLE profiles ADD COLUMN avatar_path TEXT;",
    "ALTER TABLE profiles ADD COLUMN config_json TEXT NOT NULL DEFAULT '{}';",
    "ALTER TABLE posts ADD COLUMN extras_json TEXT NOT NULL DEFAULT '{}';",
    "ALTER TABLE profiles ADD COLUMN kind TEXT NOT NULL DEFAULT 'standard';",
];

impl Database {
    /// Safely applies non-destructive schema additions for existing user vaults.
    /// Silently ignores duplicate column errors if columns were already provisioned.
    pub(super) fn migrate_posts_schema(&self) -> Result<()> {
        for stmt in MIGRATION_STATEMENTS {
            match self.conn.execute(stmt, []) {
                Ok(_) => {}
                Err(rusqlite::Error::SqliteFailure(_, Some(ref msg)))
                    if msg.contains(DUPLICATE_COLUMN_MARKER) =>
                {
                    // Existing column in user vault; continue without error
                }
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains(DUPLICATE_COLUMN_MARKER) {
                        // Safe to ignore duplicate column error
                    } else {
                        return Err(AppError::DatabaseError(format!(
                            "Database migration failed for statement '{}': {}",
                            stmt, e
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}
