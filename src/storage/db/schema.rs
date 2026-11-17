//! Table definitions, indexes and connection pragmas for the local vault.

use super::{db_err, Database};
use crate::error::Result;

/// High-performance connection pragmas applied when opening an on-disk vault.
pub(super) const PERFORMANCE_PRAGMAS: &str = "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA temp_store = MEMORY;
             PRAGMA cache_size = -64000;
             PRAGMA mmap_size = 268435456;";

/// Base tables and indexes, created idempotently.
const CREATE_TABLES: &str = r#"
                CREATE TABLE IF NOT EXISTS networks (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    slug TEXT NOT NULL,
                    description TEXT NOT NULL,
                    doc_count INTEGER NOT NULL DEFAULT 0,
                    agent_count INTEGER NOT NULL DEFAULT 0,
                    color_accent TEXT NOT NULL DEFAULT '#8b5cf6'
                );

                CREATE TABLE IF NOT EXISTS profiles (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    handle TEXT NOT NULL,
                    avatar_initial TEXT NOT NULL,
                    role_json TEXT NOT NULL,
                    network_id TEXT,
                    avatar_path TEXT,
                    config_json TEXT NOT NULL DEFAULT '{}'
                );

                CREATE TABLE IF NOT EXISTS posts (
                    id TEXT PRIMARY KEY,
                    network_id TEXT NOT NULL,
                    author_id TEXT NOT NULL,
                    author_json TEXT NOT NULL,
                    content TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    likes INTEGER NOT NULL DEFAULT 0,
                    is_liked INTEGER NOT NULL DEFAULT 0,
                    replies_count INTEGER NOT NULL DEFAULT 0,
                    mentions_json TEXT NOT NULL DEFAULT '[]',
                    linked_posts_json TEXT NOT NULL DEFAULT '[]',
                    reply_to_id TEXT,
                    quote_id TEXT,
                    repost_id TEXT,
                    link_url TEXT,
                    image_path TEXT,
                    reposts_count INTEGER NOT NULL DEFAULT 0,
                    is_reposted INTEGER NOT NULL DEFAULT 0,
                    extras_json TEXT NOT NULL DEFAULT '{}'
                );

                CREATE TABLE IF NOT EXISTS trending (
                    tag TEXT PRIMARY KEY,
                    post_count INTEGER NOT NULL DEFAULT 0,
                    category TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS documents (
                    id TEXT PRIMARY KEY,
                    network_id TEXT NOT NULL,
                    title TEXT NOT NULL,
                    content TEXT NOT NULL,
                    chunk_count INTEGER NOT NULL DEFAULT 1,
                    created_at TEXT NOT NULL
                );

                CREATE INDEX IF NOT EXISTS idx_posts_network ON posts(network_id);
                CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at DESC);
                CREATE INDEX IF NOT EXISTS idx_documents_network ON documents(network_id);
                "#;

/// Indexes over columns that legacy vaults only gain after migration.
const CREATE_MIGRATED_INDEXES: &str = r#"
                CREATE INDEX IF NOT EXISTS idx_posts_reply_to ON posts(reply_to_id);
                CREATE INDEX IF NOT EXISTS idx_posts_repost_id ON posts(repost_id);
                "#;

impl Database {
    /// Initializes tables and indexes if they do not already exist,
    /// and performs safe, non-destructive column migrations.
    pub(super) fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(CREATE_TABLES).map_err(db_err)?;
        self.migrate_posts_schema()?;
        self.conn
            .execute_batch(CREATE_MIGRATED_INDEXES)
            .map_err(db_err)?;
        self.migrate_versions()
    }
}
