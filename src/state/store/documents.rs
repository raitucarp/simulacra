//! Grounding documents of the knowledge vault.

use super::AppState;
use crate::domain::GroundingDocument;
use crate::error::{AppError, Result};
use uuid::Uuid;

const DOCUMENT_ID_PREFIX: &str = "doc_";

impl AppState {
    /// Returns all grounding documents for the currently active network.
    pub fn current_documents(&self) -> impl Iterator<Item = &GroundingDocument> {
        self.documents.iter().filter(|d| d.network_id == self.active_network_id)
    }

    /// Adds a new grounding document to the current network and persists to SQLite.
    pub fn add_document(&mut self, title: &str, content: &str) -> Result<String> {
        let trimmed_title = title.trim();
        let trimmed_content = content.trim();

        if trimmed_title.is_empty() || trimmed_content.is_empty() {
            return Err(AppError::EmptyContent);
        }

        let doc_id = format!("{DOCUMENT_ID_PREFIX}{}", Uuid::new_v4().simple());
        let doc = GroundingDocument::new(
            doc_id.clone(),
            self.active_network_id.clone(),
            trimmed_title,
            trimmed_content,
        );

        if let Some(ref db) = self.db {
            db.save_document(&doc)?;
        }

        self.documents.insert(0, doc);
        Ok(doc_id)
    }

    /// Deletes a grounding document by id.
    pub fn delete_document(&mut self, doc_id: &str) -> Result<()> {
        if let Some(ref db) = self.db {
            db.delete_document(doc_id)?;
        }
        self.documents.retain(|d| d.id != doc_id);
        Ok(())
    }

    /// Returns a zero-copy borrowed slice of all grounding documents currently stored in memory.
    #[inline]
    pub fn documents(&self) -> &[GroundingDocument] {
        &self.documents
    }
}
