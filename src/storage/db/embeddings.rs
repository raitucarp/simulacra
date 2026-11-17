//! Embedding rows: load every vector, upsert finished chunks, trim and delete owners.

use super::{collect_rows, db_err, Database};
use crate::domain::embedding::{decode_f32_le, encode_f32_le, EmbeddingOwner, OwnerKind, StoredChunk};
use crate::error::Result;
use rusqlite::{params, Row};

const UPSERT_EMBEDDING: &str = r#"
                INSERT INTO embeddings (owner_kind, owner_id, chunk_index, model, dims, content_hash, vector)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                ON CONFLICT(owner_kind, owner_id, chunk_index) DO UPDATE SET
                    model = excluded.model,
                    dims = excluded.dims,
                    content_hash = excluded.content_hash,
                    vector = excluded.vector;
                "#;

const SELECT_EMBEDDINGS: &str =
    "SELECT owner_kind, owner_id, chunk_index, model, content_hash, vector FROM embeddings ORDER BY owner_kind, owner_id, chunk_index";
const TRIM_OWNER: &str = "DELETE FROM embeddings WHERE owner_kind = ?1 AND owner_id = ?2 AND chunk_index >= ?3";
const COUNT_EMBEDDINGS: &str = "SELECT COUNT(*) FROM embeddings";

/// One loaded row (`None` for an unknown kind or a corrupt blob, which are skipped).
type LoadedRow = Option<(EmbeddingOwner, StoredChunk)>;

impl Database {
    /// Every stored chunk vector.
    pub fn load_embeddings(&self) -> Result<Vec<(EmbeddingOwner, StoredChunk)>> {
        let mut stmt = self.conn.prepare(SELECT_EMBEDDINGS).map_err(db_err)?;
        let rows = stmt.query_map([], embedding_from_row).map_err(db_err)?;
        Ok(collect_rows(rows)?.into_iter().flatten().collect())
    }

    /// Upserts finished chunks in one transaction.
    pub fn save_embeddings(&self, rows: &[(EmbeddingOwner, StoredChunk)]) -> Result<()> {
        let tx = self.conn.unchecked_transaction().map_err(db_err)?;
        {
            let mut stmt = tx.prepare(UPSERT_EMBEDDING).map_err(db_err)?;
            for (owner, chunk) in rows {
                let blob = encode_f32_le(&chunk.vector);
                let values = params![owner.kind.key(), owner.id, chunk.index as i64, chunk.model, chunk.vector.len() as i64, chunk.content_hash, blob];
                stmt.execute(values).map_err(db_err)?;
            }
        }
        tx.commit().map_err(db_err)
    }

    /// Deletes `owner`'s chunks from index `keep` on (`0` deletes the owner).
    pub fn trim_embeddings(&self, owner: &EmbeddingOwner, keep: usize) -> Result<()> {
        self.conn.execute(TRIM_OWNER, params![owner.kind.key(), owner.id, keep as i64]).map_err(db_err)?;
        Ok(())
    }

    /// Number of stored chunk vectors.
    pub fn count_embeddings(&self) -> Result<usize> {
        let count: i64 = self.conn.query_row(COUNT_EMBEDDINGS, [], |row| row.get(0)).map_err(db_err)?;
        Ok(count as usize)
    }
}

fn embedding_from_row(row: &Row<'_>) -> rusqlite::Result<LoadedRow> {
    let kind: String = row.get(0)?;
    let id: String = row.get(1)?;
    let index: i64 = row.get(2)?;
    let model: String = row.get(3)?;
    let content_hash: String = row.get(4)?;
    let blob: Vec<u8> = row.get(5)?;
    let (Some(kind), Some(vector)) = (OwnerKind::from_key(&kind), decode_f32_le(&blob)) else { return Ok(None) };
    let chunk = StoredChunk { index: index.max(0) as usize, model, content_hash, vector };
    Ok(Some((EmbeddingOwner { kind, id }, chunk)))
}
