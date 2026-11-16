//! Stateless seeded percentage rolls (SplitMix64 over an FNV-1a key hash).

use super::PERCENT_MAX;

/// SplitMix64 increment (the 64-bit golden ratio).
const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
/// SplitMix64 first finalizer multiplier.
const MIX_MUL_1: u64 = 0xBF58_476D_1CE4_E5B9;
/// SplitMix64 second finalizer multiplier.
const MIX_MUL_2: u64 = 0x94D0_49BB_1331_11EB;
/// SplitMix64 finalizer shifts.
const MIX_SHIFT_1: u32 = 30;
const MIX_SHIFT_2: u32 = 27;
const MIX_SHIFT_3: u32 = 31;
/// FNV-1a 64-bit offset basis.
const FNV_OFFSET: u64 = 0xCBF2_9CE4_8422_2325;
/// FNV-1a 64-bit prime.
const FNV_PRIME: u64 = 0x0000_0100_0000_01B3;

/// One SplitMix64 step: a well-distributed hash of `x`.
fn splitmix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(GOLDEN_GAMMA);
    z = (z ^ (z >> MIX_SHIFT_1)).wrapping_mul(MIX_MUL_1);
    z = (z ^ (z >> MIX_SHIFT_2)).wrapping_mul(MIX_MUL_2);
    z ^ (z >> MIX_SHIFT_3)
}

/// FNV-1a hash of `key`.
fn fnv1a(key: &str) -> u64 {
    key.bytes()
        .fold(FNV_OFFSET, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME))
}

/// A deterministic roll in `0..100` for `(seed, key, tick)`.
pub fn roll_percent(seed: u64, key: &str, tick: u64) -> u8 {
    let mixed = splitmix64(seed ^ fnv1a(key) ^ splitmix64(tick));
    (mixed % u64::from(PERCENT_MAX)) as u8
}
