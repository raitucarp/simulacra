//! Provenance: who added a document, from where, and a checksum of its text.

use crate::domain::document::keyed::keyed_enum;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// How the document entered the vault.
    OriginKind {
        /// Typed or pasted into the editor.
        Typed => "typed",
        /// Read from a local file (`source` is the file name).
        File => "file",
        /// Captured from a web address (`source` is the URL).
        Url => "url",
        /// Created by an importer (`source` names the original file).
        Import => "import",
        /// Shipped with the demo networks.
        Seed => "seed",
        /// Stored before documents had catalogue records.
        Legacy => "legacy",
    }
}

impl Default for OriginKind {
    fn default() -> Self {
        Self::Typed
    }
}

/// Where a document came from. The date it was added is the document's `created_at`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Provenance {
    /// Display name of the persona that added it (empty when unknown).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub added_by: String,
    pub origin: OriginKind,
    /// File name or URL of the origin.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub source: String,
    /// [`text_checksum`] of the content when it was catalogued.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub checksum: String,
}

/// Label of the checksum algorithm, so a later algorithm can be told apart.
const CHECKSUM_LABEL: &str = "fnv1a64";
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// 64-bit FNV-1a hash of `bytes` (stable across platforms and releases).
pub fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(FNV_OFFSET, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(FNV_PRIME))
}

/// Integrity checksum of a text (`fnv1a64:<16 hex digits>`), for spotting changed or
/// duplicate imports. Not a security hash.
pub fn text_checksum(text: &str) -> String {
    format!("{CHECKSUM_LABEL}:{:016x}", fnv1a(text.as_bytes()))
}
