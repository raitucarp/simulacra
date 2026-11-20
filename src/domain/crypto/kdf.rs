//! Argon2id key derivation and its cost parameters.

use super::CryptoError;
use argon2::{Algorithm, Argon2, Params, Version};
use zeroize::Zeroizing;

/// Bytes of the random salt.
pub const SALT_LEN: usize = 16;
/// Bytes of the derived key.
pub const KEY_LEN: usize = 32;
/// Largest costs a stored ciphertext may ask for (a hostile file must not exhaust memory or
/// time).
pub const MAX_MEMORY_KIB: u32 = 1 << 20;
pub const MAX_ITERATIONS: u32 = 16;
pub const MAX_LANES: u32 = 8;

/// Argon2id cost parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KdfParams {
    pub memory_kib: u32,
    pub iterations: u32,
    pub lanes: u32,
}

impl KdfParams {
    /// 64 MiB, three passes, one lane: about half a second on a laptop.
    pub const DEFAULT: Self = Self { memory_kib: 64 * 1024, iterations: 3, lanes: 1 };

    /// Whether the costs are within the bounds a stored ciphertext may ask for.
    pub fn is_sane(self) -> bool {
        (1..=MAX_LANES).contains(&self.lanes) && (1..=MAX_ITERATIONS).contains(&self.iterations) && self.memory_kib <= MAX_MEMORY_KIB
    }
}

/// The key for `password` under `params` and `salt`, zeroized when dropped.
pub(super) fn derive_key(password: &str, params: KdfParams, salt: &[u8; SALT_LEN]) -> Result<Zeroizing<[u8; KEY_LEN]>, CryptoError> {
    let params = Params::new(params.memory_kib, params.iterations, params.lanes, Some(KEY_LEN)).map_err(|_| CryptoError::KeyDerivation)?;
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params).hash_password_into(password.as_bytes(), salt, key.as_mut()).map_err(|_| CryptoError::KeyDerivation)?;
    Ok(key)
}
