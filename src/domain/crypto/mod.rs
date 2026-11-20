//! Password-based sealing shared by everything the app encrypts (snapshots now, encrypted
//! posts next). No primitive is implemented here; both come from the RustCrypto crates.
//!
//! - **Key:** Argon2id (`argon2`, version 0x13) over the password and a random 16-byte salt,
//!   with the cost parameters in [`KdfParams`] (stored next to the ciphertext).
//! - **Seal:** XChaCha20-Poly1305 (`chacha20poly1305`) with a random 24-byte nonce. Salt and
//!   nonce come from the operating system's generator and are fresh for every seal
//!   ([`SealParams::fresh`]), so a nonce is never reused with a key.
//! - **Associated data:** optional bytes authenticated with the ciphertext but not stored in
//!   it (a file header, a record id): opening fails unless the same bytes are given.
//! - **Secrets:** the password is never stored; the derived key and decrypted bytes are
//!   zeroized when dropped.

mod kdf;
mod seal;

pub use kdf::{KdfParams, KEY_LEN, MAX_ITERATIONS, MAX_LANES, MAX_MEMORY_KIB, SALT_LEN};
pub use seal::{open, open_with, seal, seal_with, Envelope, SealParams, NONCE_LEN, TAG_LEN};

/// Why sealing or opening failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    /// The key derivation rejected its parameters.
    KeyDerivation,
    /// The cipher refused the plaintext (longer than it can seal).
    Sealing,
    /// The password is wrong, or the ciphertext or associated data changed after sealing.
    Rejected,
}

#[cfg(test)]
mod tests;
