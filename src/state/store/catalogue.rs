//! Catalogued documents: adding and editing them with their records, reading progress,
//! and the vocabulary terms and collections they are filed under.
//!
//! [`AppState::add_new_document`] is the one entry point for new documents (the editor,
//! file import and later importers); it validates, stores and indexes the document.

use super::AppState;
use crate::domain::document::ids::{generated_id, COLLECTION_ID_PREFIX, SUBJECT_ID_PREFIX};
use crate::domain::document::intake::{validate, with_derived_fields, FieldProblem};
use crate::domain::document::record::Reading;
use crate::domain::document::{chunk_count, CatalogueRecord, Collection, CollectionId, DocumentId, NewDocument, Subject, SubjectId};
use jiff::Timestamp;

impl AppState {
    /// Validates and stores `new` in its network; returns the new document's id.
    pub fn add_new_document(&mut self, new: NewDocument) -> Result<String, Vec<FieldProblem>> {
        let problems = new.problems();
        if !problems.is_empty() {
            return Err(problems);
        }
        let doc = new.into_new_document();
        let id = doc.id.clone();
        self.queue(|db| db.save_document(&doc));
        self.documents.insert(0, doc);
        self.sync_network_counts();
        Ok(id)
    }

    /// Replaces the title, text and record of document `id` (stamping the edit time).
    pub fn update_document(&mut self, id: &str, title: &str, content: &str, record: CatalogueRecord) -> Result<(), Vec<FieldProblem>> {
        let problems = validate(title, &record);
        if !problems.is_empty() {
            return Err(problems);
        }
        let Some(doc) = self.documents.iter_mut().find(|d| d.id == id) else { return Ok(()) };
        let before = doc.clone(); // sb F2: the version this edit replaces
        doc.title = title.trim().to_string();
        doc.content = content.trim().to_string();
        doc.record = with_derived_fields(CatalogueRecord { updated_at: Some(Timestamp::now()), ..record }, &doc.content);
        doc.chunk_count = chunk_count(doc.record.extent.words);
        let saved = doc.clone();
        self.keep_document_revision(&before, &saved);
        self.queue(|db| db.save_document(&saved));
        Ok(())
    }

    /// Records reading progress of document `id` (stored only when it changed).
    pub fn set_reading(&mut self, id: &str, reading: Reading) {
        let Some(doc) = self.documents.iter_mut().find(|d| d.id == id) else { return };
        if doc.record.reading == reading {
            return;
        }
        doc.record.reading = reading;
        let saved = doc.clone();
        self.queue(|db| db.save_document(&saved));
    }

    /// Ids of the terms labelled `labels`, reusing vocabulary terms (case-insensitively)
    /// and adding new top terms for the rest.
    pub fn ensure_subjects(&mut self, labels: &[String]) -> Vec<SubjectId> {
        labels
            .iter()
            .map(|label| label.trim())
            .filter(|label| !label.is_empty())
            .map(|label| match self.library.subject_by_label(label) {
                Some(subject) => subject.id.clone(),
                None => {
                    let subject = Subject::new(generated_id(SUBJECT_ID_PREFIX).as_str(), label);
                    self.queue(|db| db.save_subject(&subject));
                    let id = subject.id.clone();
                    self.library = std::mem::take(&mut self.library).with_subject(subject);
                    id
                }
            })
            .collect()
    }

    /// Id of the active network's collection named `name`, created when missing.
    pub fn ensure_collection(&mut self, name: &str) -> Option<CollectionId> {
        let name = name.trim();
        if name.is_empty() {
            return None;
        }
        if let Some(found) = self.library.collection_by_name(&self.active_network_id, name) {
            return Some(found.id.clone());
        }
        let collection = Collection {
            id: CollectionId::new(generated_id(COLLECTION_ID_PREFIX)),
            network_id: self.active_network_id.clone(),
            name: name.to_string(),
            description: String::new(),
        };
        self.queue(|db| db.save_collection(&collection));
        let id = collection.id.clone();
        self.library = std::mem::take(&mut self.library).with_collection(collection);
        Some(id)
    }

    /// The stored text of document `id` (the in-memory seam for indexers).
    pub fn document_text(&self, id: &DocumentId) -> Option<&str> {
        self.documents.iter().find(|d| d.id == id.as_str()).map(|d| d.content.as_str())
    }

    /// Queues a vault write; a dead worker is logged (writes report failures themselves).
    fn queue(&self, write: impl FnOnce(&crate::storage::Vault) -> crate::error::Result<()>) {
        if let Some(ref db) = self.db {
            if let Err(err) = write(db) {
                eprintln!("simulacra: catalogue write not queued: {err}");
            }
        }
    }
}
