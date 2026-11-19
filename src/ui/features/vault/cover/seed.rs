//! The deterministic pseudo-random stream a cover is drawn from.

use super::COVER_HUES;
use crate::domain::document::record::fnv1a;
use crate::domain::document::DocumentKind;

/// Multiplier and increment of the 64-bit linear congruential step (Knuth's MMIX).
const LCG_MULTIPLIER: u64 = 6_364_136_223_846_793_005;
const LCG_INCREMENT: u64 = 1_442_695_040_888_963_407;
/// Bits dropped from a step's output (the high bits are the most random).
const OUTPUT_SHIFT: u32 = 33;
/// Separator between kind and title in the hashed text.
const SEED_SEPARATOR: &str = "\u{1f}";

/// A seed derived from a title and kind; [`Self::pick`] yields a repeatable sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverSeed(u64);

impl CoverSeed {
    /// The seed of `title` (trimmed, case-insensitive) as a `kind`.
    pub fn of(title: &str, kind: DocumentKind) -> Self {
        let text = format!("{}{SEED_SEPARATOR}{}", kind.key(), title.trim().to_lowercase());
        Self(fnv1a(text.as_bytes()))
    }

    pub fn value(self) -> u64 {
        self.0
    }

    /// The `n`-th number of the sequence, in `0..bound` (0 for a zero bound).
    pub fn pick(self, n: u64, bound: u64) -> u64 {
        if bound == 0 {
            return 0;
        }
        let mixed = (0..=n).fold(self.0, |state, _| state.wrapping_mul(LCG_MULTIPLIER).wrapping_add(LCG_INCREMENT));
        (mixed >> OUTPUT_SHIFT) % bound
    }

    /// A number in `low..=high` from position `n` of the sequence.
    pub fn between(self, n: u64, low: f32, high: f32) -> f32 {
        const STEPS: u64 = 1000;
        low + (high - low) * self.pick(n, STEPS + 1) as f32 / STEPS as f32
    }

    /// Palette hue number `n`.
    pub fn hue(self, n: u64) -> &'static str {
        COVER_HUES[self.pick(n, COVER_HUES.len() as u64) as usize]
    }

    /// A hue different from `other`.
    pub fn second_hue(self, other: &str) -> &'static str {
        let others: Vec<&'static str> = COVER_HUES.iter().copied().filter(|hue| *hue != other).collect();
        others[self.pick(1, others.len() as u64) as usize]
    }
}
