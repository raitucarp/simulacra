//! What the import sheet (or a drop, or quick capture) asks the apply step to do.

use crate::import::document::{DocumentMetadata, ParsedDocument};
use crate::import::plan::AuthorChoice;
use crate::import::segment::Unit;
use crate::import::vault_adapter::VaultEntry;
use std::sync::Arc;

/// Separator between the facts describing a file without text.
const FACT_SEPARATOR: &str = " · ";

/// The source file, for keeping a copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalFile {
    pub name: String,
    pub extension: Option<String>,
    pub sha256: String,
    pub bytes: Arc<Vec<u8>>,
}

/// One import to apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportRequest {
    pub entry: VaultEntry,
    pub original: Option<OriginalFile>,
    /// The selected units, in reading order.
    pub units: Vec<Unit>,
    pub chapter_titles: Vec<Option<String>>,
    pub author: AuthorChoice,
    /// Publish the units as posts (otherwise the file only goes into the vault).
    pub create_posts: bool,
}

/// A one-line description of a file without text: name, format, size and checksum.
fn describe_file(doc: &ParsedDocument, title: &str) -> String {
    let facts = [title.to_string(), doc.format.label().to_string(), doc.facts.mime.to_string(), format!("{} B", doc.facts.size_bytes), doc.facts.sha256.clone()];
    facts.join(FACT_SEPARATOR)
}

/// The vault entry for `doc` under the (possibly edited) title and metadata.
pub fn vault_entry(doc: &ParsedDocument, title: &str, metadata: DocumentMetadata) -> VaultEntry {
    let text = doc.full_text();
    let content = if text.trim().is_empty() { describe_file(doc, title) } else { text };
    VaultEntry {
        title: title.trim().to_string(),
        metadata,
        format_label: doc.format.label().to_string(),
        mime: doc.facts.mime.to_string(),
        size_bytes: doc.facts.size_bytes,
        sha256: doc.facts.sha256.clone(),
        content,
        original_file: None,
        source_name: String::new(),
    }
}
