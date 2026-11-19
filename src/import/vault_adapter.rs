//! The one seam between the import pipeline and the Knowledge Vault.
//!
//! Everything the importer stores as a vault document goes through [`store_document`]: the
//! [`VaultEntry`] becomes a catalogued [`NewDocument`] (see [`super::catalogue_record`]) and
//! enters the vault through [`AppState::add_new_document`], like every other document.

use super::catalogue_record::{catalogue_record, catalogue_title};
use super::document::DocumentMetadata;
use crate::domain::document::record::OriginKind;
use crate::domain::document::NewDocument;
use crate::domain::NetworkId;
use crate::error::{AppError, Result};
use crate::state::AppState;
use std::path::PathBuf;

/// Separator between the problems of a refused document.
const PROBLEM_SEPARATOR: &str = ", ";

/// What the importer hands the vault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultEntry {
    pub title: String,
    pub metadata: DocumentMetadata,
    /// Format label (`EPUB`, `PDF`) and MIME type of the source file.
    pub format_label: String,
    pub mime: String,
    /// Size and SHA-256 of the source file.
    pub size_bytes: u64,
    pub sha256: String,
    /// The full text (or, for files without text, a description of the file).
    pub content: String,
    /// The kept copy of the original file, when one was saved.
    pub original_file: Option<PathBuf>,
    /// File name of the source (empty for pasted or captured content).
    pub source_name: String,
}

/// The catalogued document for `entry`, added by `added_by` into `network_id`.
pub fn new_document(entry: &VaultEntry, network_id: NetworkId, added_by: &str) -> NewDocument {
    let source = if entry.source_name.is_empty() { entry.format_label.clone() } else { entry.source_name.clone() };
    let mut new = NewDocument::new(network_id, catalogue_title(entry), entry.content.clone())
        .with_record(catalogue_record(entry))
        .from_origin(OriginKind::Import, source)
        .added_by(added_by);
    new.record.provenance.original = entry.original_file.as_deref().and_then(std::path::Path::file_name).map(|name| name.to_string_lossy().to_string()).unwrap_or_default(); // sb F2
    new
}

/// Stores `entry` in the active network's vault and returns the new document id.
pub fn store_document(state: &mut AppState, entry: &VaultEntry) -> Result<String> {
    let added_by = state.current_profile().map(|profile| profile.name.clone()).unwrap_or_default();
    let new = new_document(entry, state.active_network_id.clone(), &added_by);
    state.add_new_document(new).map_err(|problems| {
        let reasons = problems.iter().map(|problem| format!("{problem:?}")).collect::<Vec<_>>();
        AppError::InvalidDocument(reasons.join(PROBLEM_SEPARATOR))
    })
}
