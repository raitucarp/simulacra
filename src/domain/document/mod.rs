//! Knowledge Vault documents: the stored text that grounds agents, plus its library
//! catalogue record.
//!
//! - `record`: Dublin Core-style descriptive metadata, classification, provenance, reading.
//! - `vocabulary`: controlled subjects (broader / narrower / related) and collections.
//! - `intake`: [`NewDocument`], the pure constructor every way of adding a document uses,
//!   its validation, and the upgrade of documents stored before records existed.
//! - `citation`, `facets`, `query`, `links`, `text`: pure functions over documents.

pub mod citation;
pub mod facets;
pub mod ids;
pub mod intake;
pub(crate) mod keyed;
pub mod links;
pub mod query;
pub mod record;
pub mod text;
pub mod vocabulary;

#[cfg(test)]
pub(crate) mod fixtures;
#[cfg(test)]
mod tests;

pub use ids::{CollectionId, DocumentId, SubjectId};
pub use intake::NewDocument;
pub use record::{CatalogueRecord, DocumentKind};
pub use vocabulary::{Collection, Library, Subject};

use super::ids::NetworkId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Words per grounding chunk (the agents' retrieval unit).
pub const WORDS_PER_CHUNK: usize = 150;

/// Document stored in a network's knowledge vault for grounding agents and semantic retrieval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroundingDocument {
    pub id: String,
    pub network_id: NetworkId,
    pub title: String,
    pub content: String,
    pub chunk_count: usize,
    /// When the document was added to the vault ("date added").
    pub created_at: Timestamp,
    /// Catalogue record (a legacy record derived from the text until it is catalogued).
    #[serde(default)]
    pub record: CatalogueRecord,
}

/// Grounding chunks of a text of `words` words (at least one).
pub fn chunk_count(words: usize) -> usize {
    (words / WORDS_PER_CHUNK).max(1)
}

impl GroundingDocument {
    /// Instantiates a new document added now, with a record derived from its text.
    pub fn new(
        id: impl Into<String>,
        network_id: impl Into<NetworkId>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        let content: String = content.into();
        let created_at = Timestamp::now();
        let record = intake::legacy_record(&content);
        Self {
            id: id.into(),
            network_id: network_id.into(),
            title: title.into(),
            chunk_count: chunk_count(record.extent.words),
            content,
            created_at,
            record,
        }
    }

    /// The typed id.
    pub fn document_id(&self) -> DocumentId {
        DocumentId::new(self.id.as_str())
    }

    /// This document with `record` (its derived extent and checksum recomputed).
    pub fn with_record(mut self, record: CatalogueRecord) -> Self {
        self.record = intake::with_derived_fields(record, &self.content);
        self
    }

    /// Title and subtitle joined for display ("Ethics: Demonstrated in Geometrical Order").
    pub fn display_title(&self) -> String {
        match self.record.subtitle.trim() {
            "" => self.title.clone(),
            subtitle => format!("{}: {subtitle}", self.title),
        }
    }
}
