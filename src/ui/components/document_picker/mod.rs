//! Native "pick a text document" dialog and the reader turning a picked file into a vault
//! document (title from the file name, content from its UTF-8 text).

use std::path::{Path, PathBuf};

/// Plain-text extensions accepted by the picker.
const DOCUMENT_EXTENSIONS: [&str; 4] = ["md", "markdown", "txt", "text"];
/// Bytes per kibibyte.
const BYTES_PER_KB: u64 = 1024;
/// Largest file accepted, in kibibytes (documents are stored whole in SQLite).
pub const MAX_DOCUMENT_KB: u64 = 1024;
/// Largest file accepted, in bytes.
pub const MAX_DOCUMENT_BYTES: u64 = MAX_DOCUMENT_KB * BYTES_PER_KB;

/// Why a picked file cannot become a vault document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentImportError {
    /// The file could not be read as UTF-8 text.
    Unreadable(String),
    /// The file holds no text.
    Empty,
    /// The file is larger than [`MAX_DOCUMENT_BYTES`].
    TooLarge,
}

/// Opens a native "pick a text document" dialog titled `title`; `None` when cancelled.
pub async fn pick_document_path(title: &str) -> Option<PathBuf> {
    let file = rfd::AsyncFileDialog::new()
        .set_title(title)
        .add_filter(rust_i18n::t!("picker.document_filter").to_string(), &DOCUMENT_EXTENSIONS)
        .pick_file()
        .await?;
    Some(file.path().to_path_buf())
}

/// Document title for `path`: its file stem, or the whole file name when it has none.
pub fn document_title(path: &Path) -> String {
    path.file_stem()
        .or_else(|| path.file_name())
        .map(|name| name.to_string_lossy().trim().to_string())
        .unwrap_or_default()
}

/// Reads `path` into a `(title, content)` pair ready for the vault's add-document path.
pub fn read_document_file(path: &Path) -> Result<(String, String), DocumentImportError> {
    let size = std::fs::metadata(path)
        .map_err(|e| DocumentImportError::Unreadable(e.to_string()))?
        .len();
    if size > MAX_DOCUMENT_BYTES {
        return Err(DocumentImportError::TooLarge);
    }
    let content = std::fs::read_to_string(path)
        .map_err(|e| DocumentImportError::Unreadable(e.to_string()))?;
    if content.trim().is_empty() {
        return Err(DocumentImportError::Empty);
    }
    Ok((document_title(path), content))
}

#[cfg(test)]
mod tests;
