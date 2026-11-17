//! Documents with their catalogue records: save (one transaction), load, delete, text.

use super::record_row::{creator_text, record_params, UPSERT_RECORD};
use super::DocumentText;
use crate::domain::document::intake::legacy_record;
use crate::domain::document::{CatalogueRecord, CollectionId, DocumentId, SubjectId};
use crate::domain::{GroundingDocument, NetworkId};
use crate::error::Result;
use crate::storage::db::{collect_rows, db_err, parse_timestamp, Database};
use rusqlite::{params, params_from_iter, Row};

const UPSERT_DOCUMENT: &str = r#"
    INSERT INTO documents (id, network_id, title, content, chunk_count, created_at)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6)
    ON CONFLICT(id) DO UPDATE SET network_id = excluded.network_id, title = excluded.title,
        content = excluded.content, chunk_count = excluded.chunk_count, created_at = excluded.created_at;
"#;
const CLEAR_SUBJECT_LINKS: &str = "DELETE FROM document_subjects WHERE document_id = ?1";
const CLEAR_COLLECTION_LINKS: &str = "DELETE FROM collection_documents WHERE document_id = ?1";
const CLEAR_INDEX_ROW: &str = "DELETE FROM documents_fts WHERE document_id = ?1";
const CLEAR_RECORD: &str = "DELETE FROM catalogue_records WHERE document_id = ?1";
const DELETE_DOCUMENT: &str = "DELETE FROM documents WHERE id = ?1";
const LINK_SUBJECT: &str = "INSERT OR IGNORE INTO document_subjects (document_id, subject_id, position) VALUES (?1, ?2, ?3)";
const LINK_COLLECTION: &str =
    "INSERT OR IGNORE INTO collection_documents (collection_id, document_id, position) VALUES (?2, ?1, ?3)";
/// Index row; subject labels are read from the vocabulary as stored.
const INDEX_DOCUMENT: &str = r#"
    INSERT INTO documents_fts (document_id, title, creators, subjects, abstract, body)
    VALUES (?1, ?2, ?3, (SELECT COALESCE(group_concat(s.label, ' '), '') FROM document_subjects ds
        JOIN subjects s ON s.id = ds.subject_id WHERE ds.document_id = ?1), ?4, ?5)
"#;
const SELECT_DOCUMENTS: &str = r#"
    SELECT d.id, d.network_id, d.title, d.content, d.chunk_count, d.created_at, c.record_json,
        (SELECT json_group_array(subject_id) FROM (SELECT subject_id FROM document_subjects
            WHERE document_id = d.id ORDER BY position)),
        (SELECT json_group_array(collection_id) FROM (SELECT collection_id FROM collection_documents
            WHERE document_id = d.id ORDER BY position))
    FROM documents d LEFT JOIN catalogue_records c ON c.document_id = d.id
"#;
const BY_NETWORK: &str = " WHERE d.network_id = ?1";
const NEWEST_FIRST: &str = " ORDER BY d.created_at DESC";
const SELECT_TEXT: &str = "SELECT id, network_id, title, content FROM documents WHERE id = ?1";

impl Database {
    /// Saves `doc` with its record, subject and collection memberships and search index
    /// row, all or nothing. The single write path for documents.
    pub fn save_document(&self, doc: &GroundingDocument) -> Result<()> {
        let transaction = self.conn.unchecked_transaction().map_err(db_err)?;
        self.write_document_rows(doc)?;
        transaction.commit().map_err(db_err)
    }

    /// The rows of [`Self::save_document`], inside the caller's transaction.
    pub(in crate::storage::db) fn write_document_rows(&self, doc: &GroundingDocument) -> Result<()> {
        let conn = &self.conn;
        let created = doc.created_at.to_rfc3339();
        conn.execute(UPSERT_DOCUMENT, params![doc.id, doc.network_id.as_str(), doc.title, doc.content, doc.chunk_count as i64, created])
            .map_err(db_err)?;
        conn.execute(UPSERT_RECORD, params_from_iter(record_params(doc))).map_err(db_err)?;
        for clear in [CLEAR_SUBJECT_LINKS, CLEAR_COLLECTION_LINKS, CLEAR_INDEX_ROW] {
            conn.execute(clear, [&doc.id]).map_err(db_err)?;
        }
        for (position, subject) in doc.record.subjects.iter().enumerate() {
            conn.execute(LINK_SUBJECT, params![doc.id, subject.as_str(), position as i64]).map_err(db_err)?;
        }
        for (position, collection) in doc.record.collections.iter().enumerate() {
            conn.execute(LINK_COLLECTION, params![doc.id, collection.as_str(), position as i64]).map_err(db_err)?;
        }
        let record = &doc.record;
        let title = format!("{} {}", doc.title, record.subtitle);
        conn.execute(INDEX_DOCUMENT, params![doc.id, title, creator_text(doc), record.description, doc.content])
            .map_err(db_err)?;
        Ok(())
    }

    /// Loads all grounding documents for a specific network domain, newest first.
    pub fn load_documents_for_network(&self, network_id: &NetworkId) -> Result<Vec<GroundingDocument>> {
        let mut stmt = self.conn.prepare(&format!("{SELECT_DOCUMENTS}{BY_NETWORK}{NEWEST_FIRST}")).map_err(db_err)?;
        let rows = stmt.query_map([network_id.as_str()], document_from_row).map_err(db_err)?;
        collect_rows(rows)
    }

    /// Loads all documents across all networks, newest first.
    pub fn load_all_documents(&self) -> Result<Vec<GroundingDocument>> {
        let mut stmt = self.conn.prepare(&format!("{SELECT_DOCUMENTS}{NEWEST_FIRST}")).map_err(db_err)?;
        let rows = stmt.query_map([], document_from_row).map_err(db_err)?;
        collect_rows(rows)
    }

    /// Deletes a document with its record, memberships and index row.
    pub fn delete_document(&self, doc_id: &str) -> Result<()> {
        let transaction = self.conn.unchecked_transaction().map_err(db_err)?;
        for statement in [CLEAR_SUBJECT_LINKS, CLEAR_COLLECTION_LINKS, CLEAR_INDEX_ROW, CLEAR_RECORD, DELETE_DOCUMENT] {
            self.conn.execute(statement, [doc_id]).map_err(db_err)?;
        }
        transaction.commit().map_err(db_err)
    }

    /// The text of document `id` (none when it does not exist).
    pub fn document_text(&self, id: &DocumentId) -> Result<Option<DocumentText>> {
        let mut stmt = self.conn.prepare(SELECT_TEXT).map_err(db_err)?;
        let mut rows = stmt.query_map([id.as_str()], text_from_row).map_err(db_err)?;
        rows.next().transpose().map_err(db_err)
    }
}

fn text_from_row(row: &Row<'_>) -> rusqlite::Result<DocumentText> {
    let text: String = row.get(3)?;
    Ok(DocumentText {
        id: DocumentId::new(row.get::<_, String>(0)?),
        network_id: NetworkId::new(row.get::<_, String>(1)?),
        title: row.get(2)?,
        checksum: crate::domain::document::record::text_checksum(&text),
        text,
    })
}

/// Ids in a `json_group_array` column.
fn id_list<T: serde::de::DeserializeOwned>(raw: Option<String>) -> Vec<T> {
    raw.and_then(|json| serde_json::from_str(&json).ok()).unwrap_or_default()
}

/// Maps a joined `documents` row into a [`GroundingDocument`]; a missing or unreadable
/// record is derived from the text as for a legacy document.
fn document_from_row(row: &Row<'_>) -> rusqlite::Result<GroundingDocument> {
    let content: String = row.get(3)?;
    let stored: Option<String> = row.get(6)?;
    let mut record: CatalogueRecord = stored
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_else(|| legacy_record(&content));
    record.subjects = id_list::<SubjectId>(row.get(7)?);
    record.collections = id_list::<CollectionId>(row.get(8)?);
    Ok(GroundingDocument {
        id: row.get(0)?,
        network_id: NetworkId::new(row.get::<_, String>(1)?),
        title: row.get(2)?,
        content,
        chunk_count: row.get::<_, i64>(4)? as usize,
        created_at: parse_timestamp(&row.get::<_, String>(5)?),
        record,
    })
}
