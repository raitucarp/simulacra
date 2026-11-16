//! SQLite persistence layer for Simulacra.
//!
//! Stores network domains, profiles, thoughts/posts, likes, and settings locally.
//! `Database` is a single type whose `impl` blocks are spread across the sibling
//! modules by table (networks, profiles, posts, trending, settings, documents).

mod documents;
mod migrate;
mod networks;
mod post_updates;
mod posts;
mod profiles;
mod schema;
mod settings;
mod trending;

#[cfg(test)]
mod tests;

use crate::error::{AppError, Result};
use chrono::{DateTime, Utc};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

/// Default database file name in the workspace or application folder.
const DEFAULT_DB_FILE: &str = "simulacra.db";

/// SQLite integer encoding of a `true` boolean flag.
const SQL_TRUE: i64 = 1;
/// SQLite integer encoding of a `false` boolean flag.
const SQL_FALSE: i64 = 0;

/// Primary SQLite persistence manager.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens or creates a database at the specified path and runs migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path.as_ref()).map_err(db_err)?;
        conn.execute_batch(schema::PERFORMANCE_PRAGMAS).map_err(db_err)?;

        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Opens an in-memory SQLite database (primarily for testing).
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().map_err(db_err)?;

        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Default database file path in the workspace or application folder.
    pub fn default_path() -> PathBuf {
        PathBuf::from(DEFAULT_DB_FILE)
    }
}

/// Maps any displayable storage error into the application's database error variant.
fn db_err(e: impl ToString) -> AppError {
    AppError::DatabaseError(e.to_string())
}

/// Encodes a boolean flag as a SQLite integer.
fn sql_bool(flag: bool) -> i64 {
    if flag {
        SQL_TRUE
    } else {
        SQL_FALSE
    }
}

/// Parses an RFC 3339 timestamp column, falling back to the current time when malformed.
fn parse_timestamp(raw: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(raw)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

/// Collects mapped rows, converting the first row error into an application error.
fn collect_rows<T>(rows: impl Iterator<Item = rusqlite::Result<T>>) -> Result<Vec<T>> {
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(db_err)?);
    }
    Ok(out)
}
