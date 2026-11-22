//! A feeling's parts (M1): its intensity on a five-step scale and the rules its emojis and
//! note follow.

use serde::{Deserialize, Serialize};

/// Most emojis one feeling holds.
pub const MAX_FEELING_EMOJIS: usize = 3;
/// Longest note (characters).
pub const MAX_FEELING_NOTE_CHARS: usize = 80;

/// How strongly the author feels it, from 1 (faintly) to 5 (overwhelmingly).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub struct Intensity(u8);

impl Intensity {
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(5);
    /// Every step, faintest first.
    pub const ALL: [Self; 5] = [Self(1), Self(2), Self(3), Self(4), Self(5)];

    /// `level` clamped to the scale.
    pub const fn new(level: u8) -> Self {
        Self(if level < Self::MIN.0 { Self::MIN.0 } else if level > Self::MAX.0 { Self::MAX.0 } else { level })
    }

    pub const fn level(self) -> u8 {
        self.0
    }
}

impl Default for Intensity {
    /// The middle of the scale.
    fn default() -> Self {
        Self(3)
    }
}

impl From<u8> for Intensity {
    fn from(level: u8) -> Self {
        Self::new(level)
    }
}

impl From<Intensity> for u8 {
    fn from(intensity: Intensity) -> Self {
        intensity.0
    }
}

/// Why a feeling cannot be posted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeelingProblem {
    /// No emoji was picked.
    NoEmoji,
    /// More than [`MAX_FEELING_EMOJIS`] emojis.
    TooManyEmojis,
    /// The note is longer than [`MAX_FEELING_NOTE_CHARS`].
    NoteTooLong,
}

impl FeelingProblem {
    /// Stable key (locale keys `feeling.problem.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::NoEmoji => "no_emoji",
            Self::TooManyEmojis => "too_many_emojis",
            Self::NoteTooLong => "note_too_long",
        }
    }
}

/// `emojis` trimmed, without blanks or repeats, and `note` trimmed, if they fit.
pub(super) fn checked(emojis: &[String], note: &str) -> Result<(Vec<String>, String), FeelingProblem> {
    let mut kept: Vec<String> = Vec::new();
    for emoji in emojis.iter().map(|emoji| emoji.trim()).filter(|emoji| !emoji.is_empty()) {
        if !kept.iter().any(|seen| seen == emoji) {
            kept.push(emoji.to_string());
        }
    }
    let note = note.trim().to_string();
    match (kept.len(), note.chars().count()) {
        (0, _) => Err(FeelingProblem::NoEmoji),
        (count, _) if count > MAX_FEELING_EMOJIS => Err(FeelingProblem::TooManyEmojis),
        (_, chars) if chars > MAX_FEELING_NOTE_CHARS => Err(FeelingProblem::NoteTooLong),
        _ => Ok((kept, note)),
    }
}
