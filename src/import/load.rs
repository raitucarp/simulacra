//! Reading a file into memory (the filesystem edge): refused before reading when it is over
//! the size cap, then detected and parsed. Meant for a background thread.

use super::detect::{detect_format, extension_of};
use super::document::ParsedDocument;
use super::parse::parse_document;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A file read into memory and parsed.
#[derive(Debug, Clone)]
pub struct LoadedFile {
    pub name: String,
    pub path: Option<PathBuf>,
    pub bytes: Arc<Vec<u8>>,
    pub doc: Arc<ParsedDocument>,
}

impl LoadedFile {
    /// The lower-case extension of the file name.
    pub fn extension(&self) -> Option<String> {
        extension_of(&self.name)
    }
}

/// Why a file was not read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    TooLarge { size: u64, max: u64 },
    Unreadable(String),
}

/// Parses bytes that came from a drop, a paste or a picker.
pub fn load_bytes(name: &str, mime_hint: Option<&str>, bytes: Vec<u8>) -> LoadedFile {
    let format = detect_format(name, mime_hint, &bytes);
    let doc = parse_document(format, &bytes, name);
    LoadedFile { name: name.to_string(), path: None, bytes: Arc::new(bytes), doc: Arc::new(doc) }
}

/// Reads and parses the file at `path`, refusing files over `max_bytes` without reading them.
pub fn load_file(path: &Path, max_bytes: u64) -> Result<LoadedFile, LoadError> {
    let size = std::fs::metadata(path).map_err(|err| LoadError::Unreadable(err.to_string()))?.len();
    if size > max_bytes {
        return Err(LoadError::TooLarge { size, max: max_bytes });
    }
    let bytes = std::fs::read(path).map_err(|err| LoadError::Unreadable(err.to_string()))?;
    let name = path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
    Ok(LoadedFile { path: Some(path.to_path_buf()), ..load_bytes(&name, None, bytes) })
}
