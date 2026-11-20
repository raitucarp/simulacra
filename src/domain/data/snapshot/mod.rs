//! Encrypted snapshots (sb F2): a whole-database backup sealed with a passphrase. The
//! key derivation and seal are the shared [`crypto`](crate::domain::crypto) module's.
//!
//! - **Key:** Argon2id (RustCrypto `argon2`, version 0x13) over the passphrase and a random
//!   16-byte salt, with the cost parameters stored in the header.
//! - **Seal:** XChaCha20-Poly1305 (RustCrypto `chacha20poly1305`) with a random 24-byte
//!   nonce; salt and nonce come from the operating system's generator and are fresh for
//!   every snapshot, so a nonce is never reused with a key.
//! - **File:** a versioned [`Header`] (magic, version, KDF parameters, salt, nonce, creation
//!   time, payload length), then the sealed payload. The header bytes are the associated
//!   data, so changing any of them makes opening fail like a wrong passphrase.
//! - **Passphrase:** never stored, logged or written to the database; the derived key and
//!   decrypted bytes are zeroized when dropped.

mod format;
mod seal;
mod strength;

pub use format::{Header, KdfParams, FORMAT_VERSION, HEADER_LEN, MAGIC, NONCE_LEN, SALT_LEN};
pub use seal::{inspect, open, seal, seal_with, Opened};
pub use strength::{meets_minimum, strength, Strength, MIN_PASSPHRASE_CHARS};

/// File extension of snapshot files.
pub const SNAPSHOT_EXTENSION: &str = "simsnap";

/// Why a snapshot could not be read or written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotError {
    /// The file does not start with the snapshot magic.
    NotASnapshot,
    /// Written by a newer (or unknown) format version.
    UnknownVersion(u16),
    /// The file ends before its header or payload does.
    Truncated,
    /// The header is inconsistent (unknown key derivation, absurd costs, extra bytes).
    Corrupt,
    /// The passphrase is wrong, or the file was changed after sealing.
    WrongPassphrase,
    /// The key derivation rejected its parameters.
    KeyDerivation,
}

impl SnapshotError {
    /// Stable key of the variant (locale keys `data.snapshot.error.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::NotASnapshot => "not_a_snapshot",
            Self::UnknownVersion(_) => "unknown_version",
            Self::Truncated => "truncated",
            Self::Corrupt => "corrupt",
            Self::WrongPassphrase => "wrong_passphrase",
            Self::KeyDerivation => "key_derivation",
        }
    }
}

#[cfg(test)]
mod tests;
