//! The package's integrity: npm's `sha512-<base64>` form of the tarball's SHA-512 (S10).

use base64::Engine;
use sha2::{Digest, Sha512};

/// The prefix of an npm integrity string.
const SHA512_PREFIX: &str = "sha512-";

/// `bytes` in npm's integrity form (`sha512-` and the base64 digest).
pub fn sha512_integrity(bytes: &[u8]) -> String {
    format!("{SHA512_PREFIX}{}", base64::engine::general_purpose::STANDARD.encode(Sha512::digest(bytes)))
}

/// Prefix of a SHA-256 integrity string (illustration packs pin their archives with it).
const SHA256_PREFIX: &str = "sha256-";

/// Whether `bytes` match the integrity string `expected` (`sha512-` or `sha256-`, base64).
pub fn matches_integrity(bytes: &[u8], expected: &str) -> bool {
    if expected.starts_with(SHA512_PREFIX) {
        return sha512_integrity(bytes) == expected;
    }
    let hex = crate::domain::packs::sha256_hex(bytes);
    expected.starts_with(SHA256_PREFIX) && crate::domain::packs::sha256_integrity(&hex).is_some_and(|actual| actual == expected)
}
