//! v2: documents stored before the catalogue existed get a legacy record (a note whose
//! abstract is its first sentence, with word count and checksum) and a search index row.
//! Their title, text and date added are kept as they were.

use crate::domain::document::intake::legacy_record;
use crate::domain::{GroundingDocument, NetworkId};
use crate::error::Result;
use crate::storage::db::{collect_rows, db_err, parse_timestamp, Database};
use rusqlite::Row;

/// Documents that have no catalogue record yet.
const SELECT_UNCATALOGUED: &str = r#"
    SELECT id, network_id, title, content, chunk_count, created_at FROM documents
    WHERE id NOT IN (SELECT document_id FROM catalogue_records)
"#;

fn legacy_document(row: &Row<'_>) -> rusqlite::Result<GroundingDocument> {
    let content: String = row.get(3)?;
    Ok(GroundingDocument {
        id: row.get(0)?,
        network_id: NetworkId::new(row.get::<_, String>(1)?),
        title: row.get(2)?,
        record: legacy_record(&content),
        content,
        chunk_count: row.get::<_, i64>(4)? as usize,
        created_at: parse_timestamp(&row.get::<_, String>(5)?),
    })
}

impl Database {
    /// Catalogues every uncatalogued document (runs inside the migration transaction).
    pub(in crate::storage::db) fn migrate_catalogue_records(&self) -> Result<()> {
        let documents = {
            let mut stmt = self.conn.prepare(SELECT_UNCATALOGUED).map_err(db_err)?;
            let rows = stmt.query_map([], legacy_document).map_err(db_err)?;
            collect_rows(rows)?
        };
        documents.iter().try_for_each(|doc| self.write_document_rows(doc))
    }
}
