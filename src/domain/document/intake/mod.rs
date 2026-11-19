//! How documents enter the vault: [`NewDocument`] (the one constructor every path uses:
//! the editor, file import and later importers), validation, and the upgrade of documents
//! stored before catalogue records existed.

mod validate;

pub use validate::{validate, validate_in_year, FieldProblem, RecordField, MAX_TITLE_CHARS};

use super::ids::{generated_id, DOCUMENT_ID_PREFIX};
use super::record::{text_checksum, word_count, CatalogueRecord, OriginKind, Provenance};
use super::{chunk_count, GroundingDocument};
use crate::domain::NetworkId;
use jiff::Timestamp;

/// A document about to be added: what an editor, importer or seed knows about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewDocument {
    pub network_id: NetworkId,
    pub title: String,
    pub content: String,
    pub record: CatalogueRecord,
}

impl NewDocument {
    pub fn new(network_id: impl Into<NetworkId>, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self { network_id: network_id.into(), title: title.into(), content: content.into(), record: CatalogueRecord::default() }
    }

    pub fn with_record(mut self, record: CatalogueRecord) -> Self {
        self.record = record;
        self
    }

    /// Records where the document came from (`source` is a file name or URL).
    pub fn from_origin(mut self, origin: OriginKind, source: impl Into<String>) -> Self {
        self.record.provenance.origin = origin;
        self.record.provenance.source = source.into();
        self
    }

    /// Records who added the document.
    pub fn added_by(mut self, name: impl Into<String>) -> Self {
        self.record.provenance.added_by = name.into();
        self
    }

    /// Everything wrong with the document (empty when it can be added).
    pub fn problems(&self) -> Vec<FieldProblem> {
        validate(&self.title, &self.record)
    }

    /// The stored document with id `id`, added at `added_at`: title and text trimmed,
    /// derived fields (word count, chunks, checksum, default format) filled in.
    pub fn into_document(self, id: impl Into<String>, added_at: Timestamp) -> GroundingDocument {
        let content = self.content.trim().to_string();
        let record = with_derived_fields(self.record, &content);
        GroundingDocument {
            id: id.into(),
            network_id: self.network_id,
            title: self.title.trim().to_string(),
            chunk_count: chunk_count(record.extent.words),
            content,
            created_at: added_at,
            record,
        }
    }

    /// [`Self::into_document`] with a fresh id, added now.
    pub fn into_new_document(self) -> GroundingDocument {
        self.into_document(generated_id(DOCUMENT_ID_PREFIX), Timestamp::now())
    }
}

/// `record` with the fields derived from `content` recomputed.
pub fn with_derived_fields(mut record: CatalogueRecord, content: &str) -> CatalogueRecord {
    record.extent.words = word_count(content);
    record.provenance.checksum = text_checksum(content);
    if record.format.trim().is_empty() {
        record.format = record.kind.default_format().to_string();
    }
    record
}

/// Longest abstract derived for a legacy document, in characters.
const LEGACY_ABSTRACT_CHARS: usize = 240;
const ELLIPSIS: char = '…';

/// The record of a document stored before catalogue records existed: a note of unknown
/// origin whose abstract is its first sentence.
pub fn legacy_record(content: &str) -> CatalogueRecord {
    let record = CatalogueRecord {
        description: first_sentence(content),
        provenance: Provenance { origin: OriginKind::Legacy, ..Provenance::default() },
        ..CatalogueRecord::default()
    };
    with_derived_fields(record, content)
}

/// The first sentence of `text`, cut to [`LEGACY_ABSTRACT_CHARS`].
pub fn first_sentence(text: &str) -> String {
    let text = text.trim_start_matches(|c: char| c == '#' || c.is_whitespace());
    let end = text.find(". ").map_or(text.len(), |at| at + 1);
    let sentence = text[..end].trim();
    if sentence.chars().count() <= LEGACY_ABSTRACT_CHARS {
        return sentence.to_string();
    }
    let cut: String = sentence.chars().take(LEGACY_ABSTRACT_CHARS).collect();
    format!("{}{ELLIPSIS}", cut.trim_end())
}
