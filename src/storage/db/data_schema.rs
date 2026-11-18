//! Tables of version history, bookmarks and collection items (sb F2), created idempotently
//! on every open:
//! - `revisions`: earlier versions of posts and vault documents (text, title, record JSON);
//! - `bookmarks`: bookmarked posts;
//! - `collection_items`: the order and note of each item of a vault collection, and the
//!   membership of posts (documents stay filed through their catalogue records).

/// Table and index definitions.
pub(super) const CREATE_DATA_TABLES: &str = r#"
                CREATE TABLE IF NOT EXISTS revisions (
                    id TEXT PRIMARY KEY,
                    subject_kind TEXT NOT NULL,
                    subject_id TEXT NOT NULL,
                    saved_at TEXT NOT NULL,
                    title TEXT NOT NULL DEFAULT '',
                    content TEXT NOT NULL,
                    record_json TEXT NOT NULL DEFAULT ''
                );
                CREATE INDEX IF NOT EXISTS idx_revisions_subject ON revisions(subject_kind, subject_id, saved_at);

                CREATE TABLE IF NOT EXISTS bookmarks (
                    post_id TEXT PRIMARY KEY,
                    created_at TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS collection_items (
                    collection_id TEXT NOT NULL,
                    item_kind TEXT NOT NULL,
                    item_id TEXT NOT NULL,
                    position INTEGER NOT NULL DEFAULT 0,
                    note TEXT NOT NULL DEFAULT '',
                    added_at TEXT NOT NULL,
                    PRIMARY KEY (collection_id, item_kind, item_id)
                );
                CREATE INDEX IF NOT EXISTS idx_collection_items_item ON collection_items(item_kind, item_id);
                "#;
