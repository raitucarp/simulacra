//! SHA-256 of pack files: lowercase hex in manifests, SRI form (`sha256-<base64>`) for the
//! download check shared with the icon library.

use base64::Engine;
use sha2::{Digest, Sha256};

/// Prefix of an SRI SHA-256 integrity string.
const SHA256_PREFIX: &str = "sha256-";
/// Hex digits of a SHA-256 digest.
pub(super) const SHA256_HEX_LEN: usize = 64;

/// Lowercase hex SHA-256 of `bytes` (what `pack.json` pins).
pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The SRI integrity string (`sha256-<base64>`) of a lowercase hex digest, or `None` when
/// `hex` is not one.
pub fn sha256_integrity(hex: &str) -> Option<String> {
    if !is_sha256_hex(hex) {
        return None;
    }
    let digest: Option<Vec<u8>> = (0..hex.len()).step_by(2).map(|at| u8::from_str_radix(&hex[at..at + 2], 16).ok()).collect();
    Some(format!("{SHA256_PREFIX}{}", base64::engine::general_purpose::STANDARD.encode(digest?)))
}

/// Whether `hex` is a lowercase hex SHA-256 digest.
pub(super) fn is_sha256_hex(hex: &str) -> bool {
    hex.len() == SHA256_HEX_LEN && hex.chars().all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}
