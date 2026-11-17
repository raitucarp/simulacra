//! The `embeddings` table (sb B3): one row per embedded chunk, the vector as a packed
//! little-endian `f32` BLOB. Created idempotently on every open.

/// Table and index definitions.
pub(super) const CREATE_EMBEDDINGS: &str = r#"
                CREATE TABLE IF NOT EXISTS embeddings (
                    owner_kind TEXT NOT NULL,
                    owner_id TEXT NOT NULL,
                    chunk_index INTEGER NOT NULL,
                    model TEXT NOT NULL,
                    dims INTEGER NOT NULL,
                    content_hash TEXT NOT NULL,
                    vector BLOB NOT NULL,
                    PRIMARY KEY (owner_kind, owner_id, chunk_index)
                );

                CREATE INDEX IF NOT EXISTS idx_embeddings_model ON embeddings(model);
                "#;

/// v4 (sb B3): the Gemma embedding model is not served by the Gemini API, so a stored
/// Gemini embedding model of `embeddinggemma` (the earlier placeholder default) becomes the
/// working default, `gemini-embedding-001`. A local-server choice keeps its model.
pub(super) const EMBEDDING_MODEL_VERSION: i64 = 4;

pub(super) const MIGRATE_EMBEDDING_MODEL: &str = r#"
    UPDATE settings SET value = json_set(value, '$.model', 'gemini-embedding-001')
        WHERE key = 'preferences.embeddings' AND json_valid(value)
        AND json_extract(value, '$.model') = 'embeddinggemma'
        AND coalesce(json_extract(value, '$.provider'), 'gemini') = 'gemini';
"#;

impl super::Database {
    /// The v4 rewrite (run once, inside its transaction, by the versioned migrations).
    pub(super) fn migrate_embedding_model(&self) -> crate::error::Result<()> {
        self.conn.execute_batch(MIGRATE_EMBEDDING_MODEL).map_err(super::db_err)
    }
}
