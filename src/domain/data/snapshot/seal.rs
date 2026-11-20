//! Sealing and opening a snapshot file: the payload goes through the shared
//! [`crypto`](crate::domain::crypto) seal (Argon2id key, XChaCha20-Poly1305) with the header
//! bytes as associated data, and the file is the header followed by the ciphertext.

use super::format::{Header, KdfParams, FORMAT_VERSION, NONCE_LEN, SALT_LEN};
use super::SnapshotError;
use crate::domain::crypto::{self, CryptoError, SealParams, TAG_LEN};
use zeroize::Zeroizing;

/// An opened snapshot: its header and the decrypted bytes (zeroized when dropped).
pub struct Opened {
    pub header: Header,
    pub plaintext: Zeroizing<Vec<u8>>,
}

impl From<CryptoError> for SnapshotError {
    fn from(error: CryptoError) -> Self {
        match error {
            CryptoError::KeyDerivation | CryptoError::Sealing => Self::KeyDerivation,
            CryptoError::Rejected => Self::WrongPassphrase,
        }
    }
}

/// Seals `plaintext` with a fresh random salt and nonce from the operating system.
pub fn seal(plaintext: &[u8], passphrase: &str, params: KdfParams, created_at: i64) -> Result<Vec<u8>, SnapshotError> {
    let fresh = SealParams::fresh(params);
    seal_with(plaintext, passphrase, params, created_at, fresh.salt, fresh.nonce)
}

/// Seals `plaintext` with the given salt and nonce (only tests pick them).
pub fn seal_with(plaintext: &[u8], passphrase: &str, params: KdfParams, created_at: i64, salt: [u8; SALT_LEN], nonce: [u8; NONCE_LEN]) -> Result<Vec<u8>, SnapshotError> {
    let header = Header { version: FORMAT_VERSION, kdf: params, salt, nonce, created_at, payload_len: (plaintext.len() + TAG_LEN) as u64 };
    let aad = header.encode();
    let sealed = crypto::seal_with(plaintext, passphrase, SealParams { kdf: params, salt, nonce }, &aad)?;
    Ok([aad, sealed.ciphertext].concat())
}

/// The header of `file`, checked, without opening it.
pub fn inspect(file: &[u8]) -> Result<Header, SnapshotError> {
    Header::decode(file).map(|(header, _)| header)
}

/// Opens `file` with `passphrase`: the header is checked first, then the payload is
/// authenticated (with the header) and decrypted.
pub fn open(file: &[u8], passphrase: &str) -> Result<Opened, SnapshotError> {
    let (header, payload) = Header::decode(file)?;
    let params = SealParams { kdf: header.kdf, salt: header.salt, nonce: header.nonce };
    let plaintext = crypto::open_with(&params, payload, passphrase, &header.encode())?;
    Ok(Opened { header, plaintext })
}
