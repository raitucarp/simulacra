//! L2: the reference a post keeps to a local file.

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Files attached to one post at most.
pub const MAX_FILES_PER_POST: usize = 8;
/// Separators of a path written on any platform.
const PATH_SEPARATORS: [char; 2] = ['/', '\\'];

/// A local file attached to a post, as it was when attached. Persisted in `extras.files`:
/// never rename a field.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FileRef {
    /// Absolute path of the file (files are linked where they are, not copied).
    pub path: String,
    /// Bytes.
    pub size: u64,
    /// Last modification, when the file system reports one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<Timestamp>,
    /// MIME type found from the bytes, then the extension; empty when unknown.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub mime: String,
    /// Lower-case hex SHA-256 of the contents when attached.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sha256: String,
}

impl FileRef {
    /// A reference to the file at `path`, nothing else known yet.
    pub fn at(path: impl Into<String>) -> Self {
        Self { path: path.into(), ..Self::default() }
    }

    /// The file name (the last path component), whatever separator the path was written with.
    pub fn name(&self) -> &str {
        let trimmed = self.path.trim_end_matches(PATH_SEPARATORS);
        trimmed.rsplit(PATH_SEPARATORS).next().unwrap_or(trimmed)
    }

    /// The lower-case extension of the name, without the dot.
    pub fn extension(&self) -> Option<String> {
        let (stem, extension) = self.name().rsplit_once('.')?;
        (!stem.is_empty() && !extension.is_empty()).then(|| extension.to_ascii_lowercase())
    }
}
