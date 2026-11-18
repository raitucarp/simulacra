//! Sealing and opening: Argon2id derives the key, XChaCha20-Poly1305 seals the payload with
//! the header bytes as associated data. No primitive is implemented here; both come from
//! the RustCrypto crates.

use super::format::{Header, KdfParams, FORMAT_VERSION, NONCE_LEN, SALT_LEN};
use super::SnapshotError;
use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::rand_core::RngCore;
use chacha20poly1305::aead::{Aead, KeyInit, OsRng, Payload};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use zeroize::Zeroizing;

/// Bytes of the derived key.
const KEY_LEN: usize = 32;
/// Bytes the seal adds (the Poly1305 tag).
const TAG_LEN: usize = 16;

/// An opened snapshot: its header and the decrypted bytes (zeroized when dropped).
pub struct Opened {
    pub header: Header,
    pub plaintext: Zeroizing<Vec<u8>>,
}

/// The key for `passphrase` under `params` and `salt`, zeroized when dropped.
fn derive_key(passphrase: &str, params: KdfParams, salt: &[u8; SALT_LEN]) -> Result<Zeroizing<[u8; KEY_LEN]>, SnapshotError> {
    let params = Params::new(params.memory_kib, params.iterations, params.lanes, Some(KEY_LEN)).map_err(|_| SnapshotError::KeyDerivation)?;
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params).hash_password_into(passphrase.as_bytes(), salt, key.as_mut()).map_err(|_| SnapshotError::KeyDerivation)?;
    Ok(key)
}

/// Seals `plaintext` with a fresh random salt and nonce from the operating system.
pub fn seal(plaintext: &[u8], passphrase: &str, params: KdfParams, created_at: i64) -> Result<Vec<u8>, SnapshotError> {
    let (mut salt, mut nonce) = ([0u8; SALT_LEN], [0u8; NONCE_LEN]);
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);
    seal_with(plaintext, passphrase, params, created_at, salt, nonce)
}

/// Seals `plaintext` with the given salt and nonce (only tests pick them).
pub fn seal_with(plaintext: &[u8], passphrase: &str, params: KdfParams, created_at: i64, salt: [u8; SALT_LEN], nonce: [u8; NONCE_LEN]) -> Result<Vec<u8>, SnapshotError> {
    let header = Header { version: FORMAT_VERSION, kdf: params, salt, nonce, created_at, payload_len: (plaintext.len() + TAG_LEN) as u64 };
    let aad = header.encode();
    let key = derive_key(passphrase, params, &salt)?;
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_ref()));
    let sealed = cipher.encrypt(XNonce::from_slice(&nonce), Payload { msg: plaintext, aad: &aad }).map_err(|_| SnapshotError::KeyDerivation)?;
    Ok([aad, sealed].concat())
}

/// The header of `file`, checked, without opening it.
pub fn inspect(file: &[u8]) -> Result<Header, SnapshotError> {
    Header::decode(file).map(|(header, _)| header)
}

/// Opens `file` with `passphrase`: the header is checked first, then the payload is
/// authenticated (with the header) and decrypted.
pub fn open(file: &[u8], passphrase: &str) -> Result<Opened, SnapshotError> {
    let (header, payload) = Header::decode(file)?;
    let aad = header.encode();
    let key = derive_key(passphrase, header.kdf, &header.salt)?;
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key.as_ref()));
    let plaintext = cipher.decrypt(XNonce::from_slice(&header.nonce), Payload { msg: payload, aad: &aad }).map_err(|_| SnapshotError::WrongPassphrase)?;
    Ok(Opened { header, plaintext: Zeroizing::new(plaintext) })
}
