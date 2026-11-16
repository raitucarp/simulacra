//! A tiny, dependency-free deterministic generator (SplitMix64) so a seed always yields the
//! same portrait on every platform and release.

/// SplitMix64 increment (the 64-bit golden ratio).
const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
/// SplitMix64 output mixing multipliers and shifts.
const MIX_MUL_1: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX_MUL_2: u64 = 0x94D0_49BB_1331_11EB;
const MIX_SHIFT_1: u32 = 30;
const MIX_SHIFT_2: u32 = 27;
const MIX_SHIFT_3: u32 = 31;

/// Deterministic pseudo-random source for portrait generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortraitRng {
    state: u64,
}

impl PortraitRng {
    /// A generator whose whole sequence is determined by `seed`.
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Next 64 random bits.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GOLDEN_GAMMA);
        let mut z = self.state;
        z = (z ^ (z >> MIX_SHIFT_1)).wrapping_mul(MIX_MUL_1);
        z = (z ^ (z >> MIX_SHIFT_2)).wrapping_mul(MIX_MUL_2);
        z ^ (z >> MIX_SHIFT_3)
    }

    /// Uniform index in `0..len` (`0` when `len` is zero).
    pub fn below(&mut self, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        (self.next_u64() % len as u64) as usize
    }

    /// One element of `items`, uniformly (`items` must not be empty).
    pub fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        items[self.below(items.len())]
    }

    /// True with probability `numerator / denominator`.
    pub fn chance(&mut self, numerator: u64, denominator: u64) -> bool {
        denominator > 0 && self.next_u64() % denominator < numerator
    }
}
