//! Grounding document (knowledge vault) persistence.

use super::{collect_rows, db_err, parse_timestamp, Database};
use crate::domain::{GroundingDocument, NetworkId};
use crate::error::Result;
use rusqlite::{params, Row};

const UPSERT_DOCUMENT: &str = r#"
                INSERT INTO documents (id, network_id, title, content, chunk_count, created_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(id) DO UPDATE SET
                    network_id = excluded.network_id,
                    title = excluded.title,
                    content = excluded.content,
                    chunk_count = excluded.chunk_count,
                    created_at = excluded.created_at;
                "#;

const SELECT_NETWORK_DOCUMENTS: &str = r#"
                SELECT id, network_id, title, content, chunk_count, created_at
                FROM documents
                WHERE network_id = ?1
                ORDER BY created_at DESC
                "#;

const SELECT_ALL_DOCUMENTS: &str = r#"
                SELECT id, network_id, title, content, chunk_count, created_at
                FROM documents
                ORDER BY created_at DESC
                "#;

const DELETE_DOCUMENT: &str = "DELETE FROM documents WHERE id = ?1";

impl Database {
    /// Saves a grounding document into the knowledge vault.
    pub fn save_document(&self, doc: &GroundingDocument) -> Result<()> {
        let created_at_str = doc.created_at.to_rfc3339();
        self.conn
            .execute(
                UPSERT_DOCUMENT,
                params![
                    doc.id,
                    doc.network_id.as_str(),
                    doc.title,
                    doc.content,
                    doc.chunk_count as i64,
                    created_at_str,
                ],
            )
            .map_err(db_err)?;
        Ok(())
    }

    /// Loads all grounding documents for a specific network domain.
    pub fn load_documents_for_network(&self, network_id: &NetworkId) -> Result<Vec<GroundingDocument>> {
        let mut stmt = self.conn.prepare(SELECT_NETWORK_DOCUMENTS).map_err(db_err)?;
        let rows = stmt
            .query_map([network_id.as_str()], document_from_row)
            .map_err(db_err)?;
        collect_rows(rows)
    }

    /// Loads all documents across all networks.
    pub fn load_all_documents(&self) -> Result<Vec<GroundingDocument>> {
        let mut stmt = self.conn.prepare(SELECT_ALL_DOCUMENTS).map_err(db_err)?;
        let rows = stmt.query_map([], document_from_row).map_err(db_err)?;
        collect_rows(rows)
    }

    /// Deletes a document from the vault.
    pub fn delete_document(&self, doc_id: &str) -> Result<()> {
        self.conn
            .execute(DELETE_DOCUMENT, params![doc_id])
            .map_err(db_err)?;
        Ok(())
    }
}

/// Maps a `documents` row into a [`GroundingDocument`].
fn document_from_row(row: &Row<'_>) -> rusqlite::Result<GroundingDocument> {
    let id: String = row.get(0)?;
    let net_id: String = row.get(1)?;
    let title: String = row.get(2)?;
    let content: String = row.get(3)?;
    let chunk_count: i64 = row.get(4)?;
    let created_at_str: String = row.get(5)?;

    Ok(GroundingDocument {
        id,
        network_id: NetworkId::new(net_id),
        title,
        content,
        chunk_count: chunk_count as usize,
        created_at: parse_timestamp(&created_at_str),
    })
}
