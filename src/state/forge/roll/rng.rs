//! Small deterministic PRNG (SplitMix64) so a seed always rolls the same thinker.

const GOLDEN_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
const MIX_1: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX_2: u64 = 0x94D0_49BB_1331_11EB;
const SHIFT_1: u32 = 30;
const SHIFT_2: u32 = 27;
const SHIFT_3: u32 = 31;

#[derive(Debug, Clone)]
pub(crate) struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub(crate) fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GOLDEN_GAMMA);
        let mut z = self.state;
        z = (z ^ (z >> SHIFT_1)).wrapping_mul(MIX_1);
        z = (z ^ (z >> SHIFT_2)).wrapping_mul(MIX_2);
        z ^ (z >> SHIFT_3)
    }

    /// Uniform index in `0..len` (`0` when `len` is zero).
    pub(crate) fn below(&mut self, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        (self.next_u64() % len as u64) as usize
    }

    /// `value` moved by a uniform offset in `-spread..=spread`, clamped to `0..=max`.
    pub(crate) fn jitter(&mut self, value: u8, spread: u8, max: u8) -> u8 {
        let width = usize::from(spread) * 2 + 1;
        let offset = self.below(width) as i16 - i16::from(spread);
        (i16::from(value) + offset).clamp(0, i16::from(max)) as u8
    }

    /// A value in `range` (inclusive).
    pub(crate) fn in_range(&mut self, low: u16, high: u16) -> u16 {
        let span = usize::from(high.saturating_sub(low)) + 1;
        low + self.below(span) as u16
    }
}
