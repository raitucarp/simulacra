//! Random access to a file's bytes, so the container readers stay pure: the edge reads a
//! file, tests read a buffer.

/// Bytes read by offset.
pub trait ByteSource {
    /// Total length in bytes.
    fn len(&self) -> u64;
    /// Up to `len` bytes from `offset` (fewer at the end); `None` when unreadable.
    fn read_at(&mut self, offset: u64, len: usize) -> Option<Vec<u8>>;
}

/// A buffer as a [`ByteSource`].
pub struct Bytes<'a>(pub &'a [u8]);

impl ByteSource for Bytes<'_> {
    fn len(&self) -> u64 {
        self.0.len() as u64
    }

    fn read_at(&mut self, offset: u64, len: usize) -> Option<Vec<u8>> {
        let start = usize::try_from(offset).ok().filter(|start| *start <= self.0.len())?;
        Some(self.0[start..start.saturating_add(len).min(self.0.len())].to_vec())
    }
}

pub(super) fn be_u32(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

pub(super) fn be_u64(bytes: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_be_bytes(bytes.get(at..at + 8)?.try_into().ok()?))
}

pub(super) fn le_u32(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

/// The offset of the first `needle` in `bytes` at or after `from`.
pub(super) fn find(bytes: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    bytes.get(from..)?.windows(needle.len()).position(|window| window == needle).map(|at| at + from)
}
