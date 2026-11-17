//! A stable content hash (64-bit FNV-1a, hex): unlike `std`'s hasher it never changes between
//! builds, so it can be stored next to a vector and compared on the next start.

const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
/// Separates the hashed parts, so `("ab", "c")` and `("a", "bc")` differ.
const PART_SEPARATOR: u8 = 0x1f;

/// The hash of `parts` (joined with a unit separator) as 16 lowercase hex digits.
pub fn content_hash(parts: &[&str]) -> String {
    let bytes = parts.iter().enumerate().flat_map(|(at, part)| {
        let separator = (at > 0).then_some(PART_SEPARATOR);
        separator.into_iter().chain(part.bytes())
    });
    let hash = bytes.fold(FNV_OFFSET_BASIS, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME));
    format!("{hash:016x}")
}
