//! The snapshot header, little-endian, [`HEADER_LEN`] bytes:
//!
//! | bytes | field |
//! |---|---|
//! | 8 | magic `SIMSNAP` and a zero byte |
//! | 2 | format version |
//! | 1 | key derivation (1 = Argon2id v0x13) |
//! | 4 + 4 + 4 | memory (KiB), iterations, lanes |
//! | 16 | salt |
//! | 24 | nonce |
//! | 8 | creation time (Unix seconds) |
//! | 8 | payload length (ciphertext with its 16-byte tag) |

use super::SnapshotError;

pub const MAGIC: &[u8; 8] = b"SIMSNAP\0";
pub const FORMAT_VERSION: u16 = 1;
pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 24;
/// Bytes of the header.
pub const HEADER_LEN: usize = MAGIC.len() + 2 + 1 + 12 + SALT_LEN + NONCE_LEN + 8 + 8;
/// The only key derivation of version 1.
const KDF_ARGON2ID: u8 = 1;
/// Largest costs a file may ask for (a hostile file must not exhaust memory or time).
const MAX_MEMORY_KIB: u32 = 1 << 20;
const MAX_ITERATIONS: u32 = 16;
const MAX_LANES: u32 = 8;

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

    fn is_sane(self) -> bool {
        (1..=MAX_LANES).contains(&self.lanes) && (1..=MAX_ITERATIONS).contains(&self.iterations) && self.memory_kib <= MAX_MEMORY_KIB
    }
}

/// Everything before the sealed payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub version: u16,
    pub kdf: KdfParams,
    pub salt: [u8; SALT_LEN],
    pub nonce: [u8; NONCE_LEN],
    pub created_at: i64,
    pub payload_len: u64,
}

impl Header {
    /// The header's bytes (also the associated data of the seal).
    pub fn encode(&self) -> Vec<u8> {
        let numbers = [self.kdf.memory_kib, self.kdf.iterations, self.kdf.lanes].map(u32::to_le_bytes);
        [MAGIC.as_slice(), &self.version.to_le_bytes(), &[KDF_ARGON2ID], &numbers.concat(), &self.salt, &self.nonce, &self.created_at.to_le_bytes(), &self.payload_len.to_le_bytes()].concat()
    }

    /// The header of `file` and its payload, checked for magic, version, length and costs.
    pub fn decode(file: &[u8]) -> Result<(Self, &[u8]), SnapshotError> {
        if file.len() < MAGIC.len() || &file[..MAGIC.len()] != MAGIC {
            return Err(if MAGIC.starts_with(file) && !file.is_empty() { SnapshotError::Truncated } else { SnapshotError::NotASnapshot });
        }
        let mut reader = Reader { bytes: file, at: MAGIC.len() };
        let version = u16::from_le_bytes(reader.take()?);
        if version != FORMAT_VERSION {
            return Err(SnapshotError::UnknownVersion(version));
        }
        if file.len() < HEADER_LEN {
            return Err(SnapshotError::Truncated);
        }
        if reader.take::<1>()?[0] != KDF_ARGON2ID {
            return Err(SnapshotError::Corrupt);
        }
        let kdf = KdfParams { memory_kib: u32::from_le_bytes(reader.take()?), iterations: u32::from_le_bytes(reader.take()?), lanes: u32::from_le_bytes(reader.take()?) };
        let (salt, nonce) = (reader.take()?, reader.take()?);
        let header = Self { version, kdf, salt, nonce, created_at: i64::from_le_bytes(reader.take()?), payload_len: u64::from_le_bytes(reader.take()?) };
        let payload = &file[HEADER_LEN..];
        match (payload.len() as u64).cmp(&header.payload_len) {
            _ if !kdf.is_sane() => Err(SnapshotError::Corrupt),
            std::cmp::Ordering::Less => Err(SnapshotError::Truncated),
            std::cmp::Ordering::Greater => Err(SnapshotError::Corrupt),
            std::cmp::Ordering::Equal => Ok((header, payload)),
        }
    }
}

/// Reads fixed-size fields in order.
struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn take<const N: usize>(&mut self) -> Result<[u8; N], SnapshotError> {
        let field = self.bytes.get(self.at..self.at + N).ok_or(SnapshotError::Truncated)?;
        self.at += N;
        Ok(field.try_into().expect("slice of N bytes"))
    }
}
