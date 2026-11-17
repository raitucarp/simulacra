//! What the vault asks the shell to do to the store.

use crate::domain::document::record::{CatalogueRecord, Reading};
use crate::domain::PostId;

/// What the editor asks the store to save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorSubmission {
    /// The edited document (none to add a new one).
    pub editing: Option<String>,
    pub title: String,
    pub content: String,
    /// The record without subjects and collections (filled from the lists below).
    pub record: CatalogueRecord,
    pub subject_labels: Vec<String>,
    pub collection_names: Vec<String>,
}

/// Requests the shell carries out on the store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaultEvent {
    Save(EditorSubmission),
    Delete(String),
    /// Pick a file and add it to the vault.
    Import,
    SetReading(String, Reading),
    /// Show a linking post in the timeline.
    OpenPost(PostId),
}
