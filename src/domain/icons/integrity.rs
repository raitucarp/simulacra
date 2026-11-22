//! The package's integrity: npm's `sha512-<base64>` form of the tarball's SHA-512 (S10).

use base64::Engine;
use sha2::{Digest, Sha512};

/// The prefix of an npm integrity string.
const SHA512_PREFIX: &str = "sha512-";

/// `bytes` in npm's integrity form (`sha512-` and the base64 digest).
pub fn sha512_integrity(bytes: &[u8]) -> String {
    format!("{SHA512_PREFIX}{}", base64::engine::general_purpose::STANDARD.encode(Sha512::digest(bytes)))
}

/// Whether `bytes` match the integrity string `expected`.
pub fn matches_integrity(bytes: &[u8], expected: &str) -> bool {
    expected.starts_with(SHA512_PREFIX) && sha512_integrity(bytes) == expected
}
