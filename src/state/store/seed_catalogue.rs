//! Upgrade of vaults made before the catalogue: the demo documents still carrying the
//! derived legacy record (documents could not be edited then, so they are the shipped
//! seeds) become the current catalogued seeds, keeping their date added, and the demo
//! vocabulary and collection they are filed under are added. The user's own documents are
//! left alone.

use super::AppState;
use crate::domain::document::record::OriginKind;
use crate::domain::GroundingDocument;
use crate::state::seed::{seed_documents, seed_library};

impl AppState {
    /// Adopts the seed records of uncatalogued demo documents (runs once: an adopted record
    /// is no longer a legacy one).
    pub(super) fn adopt_seed_catalogue(&mut self) {
        let seeds = seed_documents();
        let adopted: Vec<GroundingDocument> = self
            .documents
            .iter()
            .filter(|doc| doc.record.provenance.origin == OriginKind::Legacy)
            .filter_map(|doc| seeds.iter().find(|seed| seed.id == doc.id).map(|seed| GroundingDocument { created_at: doc.created_at, ..seed.clone() }))
            .collect();
        if adopted.is_empty() {
            return;
        }
        self.add_missing_seed_terms();
        for doc in adopted {
            self.queue_document(&doc);
            if let Some(slot) = self.documents.iter_mut().find(|d| d.id == doc.id) {
                *slot = doc;
            }
        }
    }

    /// Adds the seed subjects and collections the library lacks (by id).
    fn add_missing_seed_terms(&mut self) {
        let seed = seed_library();
        let subjects: Vec<_> = seed.subjects.into_iter().filter(|s| self.library.subject(&s.id).is_none()).collect();
        for subject in subjects {
            if let Some(ref db) = self.db {
                let _ = db.save_subject(&subject);
            }
            self.library = std::mem::take(&mut self.library).with_subject(subject);
        }
        let collections: Vec<_> = seed.collections.into_iter().filter(|c| self.library.collection(&c.id).is_none()).collect();
        for collection in collections {
            if let Some(ref db) = self.db {
                let _ = db.save_collection(&collection);
            }
            self.library = std::mem::take(&mut self.library).with_collection(collection);
        }
    }

    fn queue_document(&self, doc: &GroundingDocument) {
        if let Some(ref db) = self.db {
            let _ = db.save_document(doc);
        }
    }
}
