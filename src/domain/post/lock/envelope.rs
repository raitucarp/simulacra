//! The stored form of a sealed post text (`extras.lock`): the key derivation's costs, the
//! salt, the nonce and the ciphertext (base64). None of it is secret. Field names are
//! persisted: never rename one.

use super::UnlockError;
use crate::domain::crypto::{self, CryptoError, KdfParams, SealParams, NONCE_LEN, SALT_LEN};
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

/// The envelope format this version writes and reads.
pub const LOCK_FORMAT: u32 = 1;

/// A sealed post text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostLock {
    pub format: u32,
    pub memory_kib: u32,
    pub iterations: u32,
    pub lanes: u32,
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}

fn fixed<const N: usize>(text: &str) -> Option<[u8; N]> {
    STANDARD.decode(text).ok()?.try_into().ok()
}

impl PostLock {
    /// Seals `plaintext` with a fresh salt and nonce, authenticating `aad` with it.
    pub(super) fn seal(plaintext: &[u8], password: &str, kdf: KdfParams, aad: &[u8]) -> Result<Self, CryptoError> {
        let sealed = crypto::seal_with(plaintext, password, SealParams::fresh(kdf), aad)?;
        let SealParams { kdf, salt, nonce } = sealed.params;
        Ok(Self {
            format: LOCK_FORMAT,
            memory_kib: kdf.memory_kib,
            iterations: kdf.iterations,
            lanes: kdf.lanes,
            salt: STANDARD.encode(salt),
            nonce: STANDARD.encode(nonce),
            ciphertext: STANDARD.encode(sealed.ciphertext),
        })
    }

    /// The seal parameters and ciphertext; `None` when a field cannot be read.
    fn decoded(&self) -> Option<(SealParams, Vec<u8>)> {
        let kdf = KdfParams { memory_kib: self.memory_kib, iterations: self.iterations, lanes: self.lanes };
        let params = SealParams { kdf, salt: fixed::<SALT_LEN>(&self.salt)?, nonce: fixed::<NONCE_LEN>(&self.nonce)? };
        (self.format == LOCK_FORMAT).then_some(())?;
        Some((params, STANDARD.decode(&self.ciphertext).ok()?))
    }

    /// The opened bytes (zeroized when dropped).
    pub(super) fn open(&self, password: &str, aad: &[u8]) -> Result<Zeroizing<Vec<u8>>, UnlockError> {
        let (params, ciphertext) = self.decoded().ok_or(UnlockError::Damaged)?;
        crypto::open_with(&params, &ciphertext, password, aad).map_err(|error| match error {
            CryptoError::Rejected => UnlockError::WrongPassword,
            CryptoError::KeyDerivation | CryptoError::Sealing => UnlockError::Damaged,
        })
    }
}
