//! Typed catalogue operations on the vault worker: writes are queued, reads round-trip.

use super::Vault;
use crate::domain::document::query::CatalogueQuery;
use crate::domain::document::vocabulary::SubjectTree;
use crate::domain::document::{Collection, CollectionId, DocumentId, Library, Subject, SubjectId};
use crate::error::Result;
use crate::storage::db::DocumentText;

impl Vault {
    /// Queues an upsert of a vocabulary term.
    pub fn save_subject(&self, subject: &Subject) -> Result<()> {
        let subject = subject.clone();
        self.write(move |db| db.save_subject(&subject))
    }

    /// Queues deleting a vocabulary term.
    pub fn delete_subject(&self, id: &SubjectId) -> Result<()> {
        let id = id.clone();
        self.write(move |db| db.delete_subject(&id))
    }

    /// Queues an upsert of a collection.
    pub fn save_collection(&self, collection: &Collection) -> Result<()> {
        let collection = collection.clone();
        self.write(move |db| db.save_collection(&collection))
    }

    /// Queues deleting a collection.
    pub fn delete_collection(&self, id: &CollectionId) -> Result<()> {
        let id = id.clone();
        self.write(move |db| db.delete_collection(&id))
    }

    /// Reads the vocabulary and collections (after every queued write).
    pub fn load_library(&self) -> Result<Library> {
        self.read(|db| db.load_library())
    }

    /// Ids matching `query`, in its order (after every queued write).
    pub fn search_catalogue(&self, query: &CatalogueQuery, tree: &SubjectTree) -> Result<Vec<String>> {
        let (query, tree) = (query.clone(), tree.clone());
        self.read(move |db| db.search_catalogue(&query, &tree))
    }

    /// One document's text by id (after every queued write): the embedding seam.
    pub fn document_text(&self, id: &DocumentId) -> Result<Option<DocumentText>> {
        let id = id.clone();
        self.read(move |db| db.document_text(&id))
    }
}
