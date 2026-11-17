//! Catalogue persistence: documents with their records, the subject vocabulary and
//! collections, full-text search (FTS5) and the v2 upgrade of pre-catalogue vaults.
//!
//! Seams for other features:
//! - [`Database::save_document`] is the single write path for a document (record,
//!   memberships and search index in one transaction); build the document with
//!   [`crate::domain::document::NewDocument`].
//! - [`Database::document_text`] reads one document's text by [`DocumentId`] (for
//!   embedding and chunking).

mod documents;
mod library;
mod migration;
mod record_row;
mod schema;
mod search;
mod search_sql;

pub use search::{build_catalogue_search, fts_match_expression, CatalogueSearch};

use crate::domain::document::DocumentId;
use crate::domain::NetworkId;

/// One document's text, as handed to indexers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentText {
    pub id: DocumentId,
    pub network_id: NetworkId,
    pub title: String,
    pub text: String,
    /// Checksum of `text` (changes whenever the text does).
    pub checksum: String,
}
