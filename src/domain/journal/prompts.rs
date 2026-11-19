//! Writing prompts drawn from research on reflective writing: expressive writing about
//! thoughts and feelings (Pennebaker), counting blessings (Emmons and McCullough),
//! implementation intentions "if X, then I will Y" (Gollwitzer), noticing what was learnt
//! and what was surprising, and the best possible self (King). Each kind has two wordings.
//!
//! [`prompt_for_day`] is a pure, seeded rotation: every prompt comes once per cycle of
//! [`PROMPT_COUNT`] days in a shuffled order, a day always gets the same prompt, and two
//! days in a row never share one.

use jiff::civil::Date;
use serde::{Deserialize, Serialize};

/// The research tradition a prompt comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptKind {
    ExpressiveWriting,
    Gratitude,
    Intention,
    Learned,
    Surprised,
    BestSelf,
}

impl PromptKind {
    pub const ALL: [Self; 6] = [Self::ExpressiveWriting, Self::Gratitude, Self::Intention, Self::Learned, Self::Surprised, Self::BestSelf];

    pub const fn key(self) -> &'static str {
        match self {
            Self::ExpressiveWriting => "expressive",
            Self::Gratitude => "gratitude",
            Self::Intention => "intention",
            Self::Learned => "learned",
            Self::Surprised => "surprised",
            Self::BestSelf => "best_self",
        }
    }
}

/// Wordings per kind.
pub const WORDINGS: u8 = 2;
/// Prompts in the catalogue.
pub const PROMPT_COUNT: usize = PromptKind::ALL.len() * WORDINGS as usize;

/// One prompt: its kind and which wording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Prompt {
    pub kind: PromptKind,
    pub wording: u8,
}

impl Prompt {
    /// Locale key of its text (`journal.prompt.<kind>_<n>`).
    pub fn text_key(self) -> String {
        format!("journal.prompt.{}_{}", self.kind.key(), self.wording + 1)
    }

    /// Locale key of the kind's short label.
    pub fn kind_key(self) -> String {
        format!("journal.prompt_kind.{}", self.kind.key())
    }

    /// The prompt after this one in catalogue order (the "another prompt" button).
    pub fn next(self) -> Self {
        let index = catalogue().iter().position(|p| *p == self).unwrap_or(0);
        catalogue()[(index + 1) % PROMPT_COUNT]
    }
}

/// Every prompt, kind by kind.
pub fn catalogue() -> [Prompt; PROMPT_COUNT] {
    std::array::from_fn(|i| Prompt { kind: PromptKind::ALL[i / WORDINGS as usize], wording: (i % WORDINGS as usize) as u8 })
}

/// A small, well-mixed hash (splitmix64) used as the seed of a cycle's order.
fn mix(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// The shuffled catalogue order of `cycle` (Fisher–Yates driven by the seed).
fn raw_order(cycle: u64) -> [usize; PROMPT_COUNT] {
    let mut order: [usize; PROMPT_COUNT] = std::array::from_fn(|i| i);
    let mut state = mix(cycle);
    for i in (1..PROMPT_COUNT).rev() {
        state = mix(state);
        order.swap(i, (state % (i as u64 + 1)) as usize);
    }
    order
}

/// The order of `cycle`, its first two swapped when it would repeat the last prompt of the
/// cycle before.
fn order(cycle: u64) -> [usize; PROMPT_COUNT] {
    let mut order = raw_order(cycle);
    if cycle > 0 && order[0] == raw_order(cycle - 1)[PROMPT_COUNT - 1] {
        order.swap(0, 1);
    }
    order
}

/// Today's prompt: stable for the day.
pub fn prompt_for_day(day: Date) -> Prompt {
    let number = crate::time::days_from_ce(day).max(0) as u64;
    let (cycle, position) = (number / PROMPT_COUNT as u64, (number % PROMPT_COUNT as u64) as usize);
    catalogue()[order(cycle)[position]]
}
