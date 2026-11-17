//! The one seam between the import pipeline and the Knowledge Vault.
//!
//! Everything the importer stores as a vault document goes through [`store_document`]. Today
//! it writes the plain `documents` table (title and text; the catalogue metadata has nowhere
//! to go yet). When the catalogue model lands, only this file changes: build its record from
//! the [`VaultEntry`] and store it there.

use super::document::DocumentMetadata;
use crate::error::Result;
use crate::state::AppState;
use std::path::PathBuf;

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
}

/// Stores `entry` in the active network's vault and returns the new document id.
pub fn store_document(state: &mut AppState, entry: &VaultEntry) -> Result<String> {
    state.add_document(&entry.title, &entry.content)
}
