//! Import: how documents are split, who the posts are from, the size cap and keeping the
//! original file.

use super::super::bounds::Bounds;
use super::super::choices::{ImportSegmentation, PostIdentity};
use serde::{Deserialize, Serialize};

/// Largest importable file, in megabytes.
pub const MAX_FILE_MB: Bounds = Bounds::new(1, 1024, 5);
const DEFAULT_MAX_FILE_MB: u32 = 50;

/// Import preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ImportPrefs {
    /// Consumed by a later wave.
    pub segmentation: ImportSegmentation,
    /// Consumed by a later wave.
    pub identity: PostIdentity,
    /// Consumed by a later wave.
    pub max_file_mb: u32,
    /// Copy the source file into the vault. Consumed by a later wave.
    pub keep_original: bool,
}

impl Default for ImportPrefs {
    fn default() -> Self {
        Self {
            segmentation: ImportSegmentation::default(),
            identity: PostIdentity::default(),
            max_file_mb: DEFAULT_MAX_FILE_MB,
            keep_original: true,
        }
    }
}

impl ImportPrefs {
    /// Clamps the size cap.
    pub fn validated(self) -> Self {
        Self { max_file_mb: MAX_FILE_MB.clamp_u32(self.max_file_mb), ..self }
    }
}
