//! The packs offered for download: a small list shipped with the app
//! (`assets/packs/downloadable.json`), each entry pinning its archive's SHA-256. The list is
//! empty until a pack is released; entries are added there, not in code.

use super::checksum::is_sha256_hex;
use super::keys::is_safe_id;
use super::{is_built_in, MAX_PACK_BYTES};
use serde::{Deserialize, Serialize};

/// The built-in list.
pub const DOWNLOADABLE_JSON: &str = include_str!("../../../assets/packs/downloadable.json");
/// Only secure downloads are offered.
const HTTPS: &str = "https://";

/// One downloadable pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadablePack {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: String,
    /// The `.simpack` archive.
    pub url: String,
    /// Lowercase hex SHA-256 of the archive.
    pub sha256: String,
    /// The archive's size.
    pub bytes: u64,
}

#[derive(Debug, Default, Deserialize)]
struct DownloadableList {
    #[serde(default)]
    packs: Vec<DownloadablePack>,
}

impl DownloadablePack {
    /// Whether the entry is safe to offer: a plain, non-built-in id, an `https` URL, a pinned
    /// checksum and a size within the cap.
    pub fn is_valid(&self) -> bool {
        is_safe_id(&self.id)
            && !is_built_in(&self.id)
            && !self.name.trim().is_empty()
            && self.url.starts_with(HTTPS)
            && is_sha256_hex(&self.sha256)
            && (1..=MAX_PACK_BYTES).contains(&self.bytes)
    }
}

/// The valid entries of a list (`{"packs": [...]}`); a malformed list offers nothing.
pub fn parse_downloadable(json: &str) -> Vec<DownloadablePack> {
    let list: DownloadableList = serde_json::from_str(json).unwrap_or_default();
    list.packs.into_iter().filter(DownloadablePack::is_valid).collect()
}

/// The packs the app offers for download.
pub fn builtin_downloadable_packs() -> Vec<DownloadablePack> {
    parse_downloadable(DOWNLOADABLE_JSON)
}
