//! Restoring a verified snapshot (sb F2): the decrypted database is checked in its own file,
//! then copied over the live database with SQLite's online backup, page by page, through a
//! second connection (the vault worker's own connection is idle while this runs on it).

use super::{db_err, Database};
use crate::error::Result;
use rusqlite::{Connection, DatabaseName, OpenFlags};
use std::path::Path;

/// `PRAGMA quick_check` answers this when the file is sound.
const CHECK_OK: &str = "ok";
const QUICK_CHECK: &str = "PRAGMA quick_check";
/// Tables every vault has; a file without them is not a Simulacra database.
const REQUIRED_TABLES: [&str; 3] = ["posts", "profiles", "settings"];
const TABLE_EXISTS: &str = "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1";

/// What a verified database holds (the restore confirmation names it).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DatabaseSummary {
    pub posts: usize,
    pub profiles: usize,
    pub documents: usize,
}

fn count(conn: &Connection, table: &str) -> usize {
    let exists: i64 = conn.query_row(TABLE_EXISTS, [table], |row| row.get(0)).unwrap_or_default();
    if exists == 0 {
        return 0;
    }
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| row.get::<_, i64>(0)).map_or(0, |n| n as usize)
}

/// Opens `path` read-only, checks its integrity and its tables, and counts what it holds.
pub fn verify_database_file(path: &Path) -> Result<DatabaseSummary> {
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY).map_err(db_err)?;
    let check: String = conn.query_row(QUICK_CHECK, [], |row| row.get(0)).map_err(db_err)?;
    if check != CHECK_OK {
        return Err(db_err(check));
    }
    let missing = REQUIRED_TABLES.iter().find(|table| conn.query_row(TABLE_EXISTS, [**table], |row| row.get::<_, i64>(0)).unwrap_or_default() == 0);
    if let Some(table) = missing {
        return Err(db_err(table));
    }
    Ok(DatabaseSummary { posts: count(&conn, "posts"), profiles: count(&conn, "profiles"), documents: count(&conn, "documents") })
}

impl Database {
    /// Replaces the whole live database with the database at `source` (verify it first).
    pub fn restore_from_file(&self, source: &Path) -> Result<()> {
        let mut target = Connection::open(self.path()).map_err(db_err)?;
        target.restore(DatabaseName::Main, source, None::<fn(rusqlite::backup::Progress)>).map_err(db_err)
    }
}
