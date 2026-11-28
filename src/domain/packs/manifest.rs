//! `pack.json`: a pack's identity and the file, checksum and pixel size of every key it
//! covers.

use super::checksum::is_sha256_hex;
use super::error::{PackError, PackRule};
use super::keys::{is_safe_file, is_safe_id, is_safe_key};
use super::CLASSIC_PACK_ID;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Files a pack may hold.
pub const MAX_FILES: usize = 8192;
/// Bytes one pack file may take.
pub const MAX_FILE_BYTES: u64 = 8 * 1024 * 1024;
/// Bytes a whole pack (or its archive) may take.
pub const MAX_PACK_BYTES: u64 = 512 * 1024 * 1024;
/// Largest side of a pack image.
const MAX_SIDE: u32 = 4096;

/// One covered key: its file (relative to the pack folder), pinned checksum and size.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackFile {
    pub file: String,
    pub sha256: String,
    pub width: u32,
    pub height: u32,
}

/// A raster pack's manifest.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PackManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    /// Style id (`flat`, `clay3d`, …), informational.
    #[serde(default)]
    pub style: String,
    /// Keys shown on the pack's preview card.
    #[serde(default)]
    pub samples: Vec<String>,
    /// Every covered key.
    #[serde(default)]
    pub coverage: BTreeMap<String, PackFile>,
}

impl PackManifest {
    /// Parses and validates `pack.json`.
    pub fn parse(json: &str) -> Result<Self, PackError> {
        let manifest: Self = serde_json::from_str(json).map_err(|e| PackError::Manifest(e.to_string()))?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Checks every rule: a plain, unreserved id, a name and version, plain keys and file
    /// paths, hex checksums, sane sizes and a bounded file count.
    pub fn validate(&self) -> Result<(), PackError> {
        if !is_safe_id(&self.id) || self.id == CLASSIC_PACK_ID {
            return Err(PackError::Invalid(PackRule::Id, self.id.clone()));
        }
        if self.name.trim().is_empty() {
            return Err(PackError::Invalid(PackRule::Name, self.id.clone()));
        }
        if self.version.trim().is_empty() {
            return Err(PackError::Invalid(PackRule::Version, self.id.clone()));
        }
        if self.coverage.len() > MAX_FILES {
            return Err(PackError::TooLarge);
        }
        self.coverage.iter().try_for_each(|(key, file)| validate_entry(key, file))
    }

    /// The file covering `key`, if any.
    pub fn file(&self, key: &str) -> Option<&PackFile> {
        self.coverage.get(key)
    }

    /// Whether the pack draws `key`.
    pub fn covers(&self, key: &str) -> bool {
        self.coverage.contains_key(key)
    }

    /// How many of `keys` the pack covers.
    pub fn covered_of<'a>(&self, keys: impl IntoIterator<Item = &'a str>) -> usize {
        keys.into_iter().filter(|key| self.covers(key)).count()
    }

    /// Pretty JSON of the manifest (sorted keys, as the tooling writes it).
    pub fn to_json(&self) -> String {
        serde_json::to_value(self).and_then(|value| serde_json::to_string_pretty(&value)).unwrap_or_default()
    }
}

fn validate_entry(key: &str, file: &PackFile) -> Result<(), PackError> {
    if !is_safe_key(key) {
        return Err(PackError::Invalid(PackRule::Key, key.to_string()));
    }
    if !is_safe_file(&file.file) {
        return Err(PackError::Invalid(PackRule::File, file.file.clone()));
    }
    if !is_sha256_hex(&file.sha256) {
        return Err(PackError::Invalid(PackRule::Checksum, key.to_string()));
    }
    let side = 1..=MAX_SIDE;
    if !side.contains(&file.width) || !side.contains(&file.height) {
        return Err(PackError::Invalid(PackRule::Size, key.to_string()));
    }
    Ok(())
}
