//! Sealing and opening: Argon2id derives the key, XChaCha20-Poly1305 seals the plaintext
//! with the caller's associated data.

use super::kdf::{derive_key, KdfParams, SALT_LEN};
use super::CryptoError;
use chacha20poly1305::aead::rand_core::RngCore;
use chacha20poly1305::aead::{Aead, KeyInit, OsRng, Payload};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use zeroize::Zeroizing;

/// Bytes of the random nonce.
pub const NONCE_LEN: usize = 24;
/// Bytes the seal adds to the plaintext (the Poly1305 tag).
pub const TAG_LEN: usize = 16;

/// Everything besides the password needed to open a ciphertext: the key derivation's costs
/// and salt, and the nonce. None of it is secret.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SealParams {
    pub kdf: KdfParams,
    pub salt: [u8; SALT_LEN],
    pub nonce: [u8; NONCE_LEN],
}

impl SealParams {
    /// `kdf` with a fresh random salt and nonce from the operating system.
    pub fn fresh(kdf: KdfParams) -> Self {
        let (mut salt, mut nonce) = ([0u8; SALT_LEN], [0u8; NONCE_LEN]);
        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce);
        Self { kdf, salt, nonce }
    }
}

/// A sealed plaintext: its parameters and the ciphertext (with its tag, [`TAG_LEN`] bytes
/// longer than the plaintext).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub params: SealParams,
    pub ciphertext: Vec<u8>,
}

fn cipher(password: &str, params: &SealParams) -> Result<XChaCha20Poly1305, CryptoError> {
    let key = derive_key(password, params.kdf, &params.salt)?;
    Ok(XChaCha20Poly1305::new(Key::from_slice(key.as_ref())))
}

/// Seals `plaintext` under `password` with the default costs, a fresh salt and nonce and no
/// associated data.
pub fn seal(plaintext: &[u8], password: &str) -> Result<Envelope, CryptoError> {
    seal_with(plaintext, password, SealParams::fresh(KdfParams::DEFAULT), &[])
}

/// Seals `plaintext` under `password` with `params` (take them from [`SealParams::fresh`];
/// only tests pick a salt and nonce), authenticating `aad` with it.
pub fn seal_with(plaintext: &[u8], password: &str, params: SealParams, aad: &[u8]) -> Result<Envelope, CryptoError> {
    let payload = Payload { msg: plaintext, aad };
    let ciphertext = cipher(password, &params)?.encrypt(XNonce::from_slice(&params.nonce), payload).map_err(|_| CryptoError::Sealing)?;
    Ok(Envelope { params, ciphertext })
}

/// Opens an envelope sealed without associated data.
pub fn open(envelope: &Envelope, password: &str) -> Result<Zeroizing<Vec<u8>>, CryptoError> {
    open_with(&envelope.params, &envelope.ciphertext, password, &[])
}

/// Opens `ciphertext` sealed under `params` with `aad` as associated data. Costs beyond the
/// sane bounds are refused before any key is derived.
pub fn open_with(params: &SealParams, ciphertext: &[u8], password: &str, aad: &[u8]) -> Result<Zeroizing<Vec<u8>>, CryptoError> {
    if !params.kdf.is_sane() {
        return Err(CryptoError::KeyDerivation);
    }
    let payload = Payload { msg: ciphertext, aad };
    let plaintext = cipher(password, params)?.decrypt(XNonce::from_slice(&params.nonce), payload).map_err(|_| CryptoError::Rejected)?;
    Ok(Zeroizing::new(plaintext))
}
