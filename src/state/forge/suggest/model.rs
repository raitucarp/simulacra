//! The shapes a parameter's suggestions take: example values for texts, a typical band for
//! numbers, good starting picks (and what pairs or clashes with them) for choices, or plain
//! advice for settings only the user can know.

use super::param::Param;

pub use super::phrase::{Example, Joiner, Phrase};

/// Suggestions of a free-text parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextHints {
    pub examples: Vec<Example>,
    /// One line on what makes a good value.
    pub tip: Phrase,
    pub joiner: Joiner,
}

/// What a range's numbers mean, for showing them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeUnit {
    Plain,
    /// Stored ×100 (temperature): 75 reads 0.75.
    Hundredths,
    /// Stored ×1000 (photo zoom): 1500 reads 1.5×.
    PerMille,
    Percent,
    /// An hour of the day: 7 reads 07:00.
    Hour,
}

/// Stored units per shown unit of [`RangeUnit::Hundredths`] and [`RangeUnit::PerMille`].
pub const HUNDREDTHS: f32 = 100.0;
pub const PER_MILLE: f32 = 1000.0;

impl RangeUnit {
    /// `value` as the user reads it (`0.75`, `1.5×`, `30%`, `07:00`).
    pub fn format(self, value: u16) -> String {
        match self {
            Self::Plain => value.to_string(),
            Self::Hundredths => format!("{:.2}", f32::from(value) / HUNDREDTHS),
            Self::PerMille => format!("{:.1}×", f32::from(value) / PER_MILLE),
            Self::Percent => format!("{value}%"),
            Self::Hour => format!("{value:02}:00"),
        }
    }
}

/// Suggestions of a numeric parameter: the valid range, the usual band and a default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeHint {
    pub valid: (u16, u16),
    pub typical: (u16, u16),
    pub recommended: u16,
    pub unit: RangeUnit,
    /// Why the band sits where it does.
    pub why: Phrase,
}

/// Whether two choices help or hurt each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verdict {
    Pairs,
    Clashes,
}

/// One option of one parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OptionRef {
    pub param: Param,
    pub key: &'static str,
}

/// A known good or bad combination involving this parameter (`mine` is its side).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Combo {
    pub verdict: Verdict,
    pub mine: OptionRef,
    pub other: OptionRef,
    pub reason: Phrase,
}

/// Suggestions of a single- or multi-choice parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceHints {
    /// One to three good starting options, best first.
    pub picks: Vec<&'static str>,
    pub why: Phrase,
    pub combos: Vec<Combo>,
}

/// Everything the catalog says about one parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldSuggestions {
    Text(TextHints),
    Range(RangeHint),
    Choice(ChoiceHints),
    /// Advice only: the value is something only the user knows (a key, a server's reply shape).
    Guide(Phrase),
}
