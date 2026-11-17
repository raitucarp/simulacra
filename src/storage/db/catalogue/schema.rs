//! Tables of the library catalogue, created idempotently next to the base schema.
//!
//! - `catalogue_records`: one row per document. `record_json` holds the full record; the
//!   other columns repeat the fields the catalogue filters and sorts on.
//! - `subjects` (+ `subject_relations`): the controlled vocabulary.
//! - `document_subjects`, `collections`, `collection_documents`: memberships.
//! - `documents_fts`: the full-text index (FTS5) over titles, creators, subjects, abstracts
//!   and text.

use super::super::{db_err, Database};
use crate::error::Result;

const CREATE_CATALOGUE_TABLES: &str = r#"
    CREATE TABLE IF NOT EXISTS catalogue_records (
        document_id TEXT PRIMARY KEY,
        record_json TEXT NOT NULL DEFAULT '{}',
        kind TEXT NOT NULL DEFAULT 'note',
        sort_title TEXT NOT NULL DEFAULT '',
        year_published INTEGER,
        language TEXT NOT NULL DEFAULT '',
        discipline TEXT NOT NULL DEFAULT '',
        form TEXT NOT NULL DEFAULT '',
        period TEXT NOT NULL DEFAULT '',
        place TEXT NOT NULL DEFAULT '',
        reading_status TEXT NOT NULL DEFAULT 'unread',
        reading_progress INTEGER NOT NULL DEFAULT 0,
        updated_at TEXT
    );
    CREATE TABLE IF NOT EXISTS subjects (
        id TEXT PRIMARY KEY,
        label TEXT NOT NULL,
        scope_note TEXT NOT NULL DEFAULT '',
        broader_id TEXT
    );
    CREATE TABLE IF NOT EXISTS subject_relations (
        subject_id TEXT NOT NULL,
        related_id TEXT NOT NULL,
        PRIMARY KEY (subject_id, related_id)
    );
    CREATE TABLE IF NOT EXISTS document_subjects (
        document_id TEXT NOT NULL,
        subject_id TEXT NOT NULL,
        position INTEGER NOT NULL DEFAULT 0,
        PRIMARY KEY (document_id, subject_id)
    );
    CREATE TABLE IF NOT EXISTS collections (
        id TEXT PRIMARY KEY,
        network_id TEXT NOT NULL,
        name TEXT NOT NULL,
        description TEXT NOT NULL DEFAULT ''
    );
    CREATE TABLE IF NOT EXISTS collection_documents (
        collection_id TEXT NOT NULL,
        document_id TEXT NOT NULL,
        position INTEGER NOT NULL DEFAULT 0,
        PRIMARY KEY (collection_id, document_id)
    );
    CREATE VIRTUAL TABLE IF NOT EXISTS documents_fts USING fts5(
        document_id UNINDEXED, title, creators, subjects, abstract, body,
        tokenize = 'unicode61 remove_diacritics 2'
    );
    CREATE INDEX IF NOT EXISTS idx_catalogue_kind ON catalogue_records(kind);
    CREATE INDEX IF NOT EXISTS idx_document_subjects_subject ON document_subjects(subject_id);
    CREATE INDEX IF NOT EXISTS idx_collection_documents_document ON collection_documents(document_id);
    CREATE INDEX IF NOT EXISTS idx_collections_network ON collections(network_id);
    CREATE INDEX IF NOT EXISTS idx_subjects_broader ON subjects(broader_id);
"#;

impl Database {
    /// Creates the catalogue tables and indexes when missing.
    pub(in crate::storage::db) fn create_catalogue_tables(&self) -> Result<()> {
        self.conn.execute_batch(CREATE_CATALOGUE_TABLES).map_err(db_err)
    }
}
