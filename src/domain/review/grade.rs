//! The four answer grades and the metacognitive confidence of a quiz answer.

use serde::{Deserialize, Serialize};

/// How well a card was recalled (FSRS ratings 1–4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grade {
    /// Forgotten.
    Again,
    /// Recalled with serious effort.
    Hard,
    /// Recalled after a moment.
    Good,
    /// Recalled at once.
    Easy,
}

impl Grade {
    pub const ALL: [Self; 4] = [Self::Again, Self::Hard, Self::Good, Self::Easy];

    /// Zero-based position (`w[index]` is the grade's first stability).
    pub const fn index(self) -> usize {
        match self {
            Self::Again => 0,
            Self::Hard => 1,
            Self::Good => 2,
            Self::Easy => 3,
        }
    }

    /// The FSRS rating, 1–4.
    pub fn value(self) -> f64 {
        (self.index() + 1) as f64
    }

    /// Stable wire key (database, locale keys). Never rename one.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Again => "again",
            Self::Hard => "hard",
            Self::Good => "good",
            Self::Easy => "easy",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|grade| grade.key() == key)
    }

    /// The grade at zero-based `index` (the 1–4 keys).
    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    /// Whether the card was remembered.
    pub const fn is_recall(self) -> bool {
        !matches!(self, Self::Again)
    }
}

/// How sure the learner felt before seeing whether a quiz answer was right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Guess,
    Unsure,
    Sure,
}

impl Confidence {
    pub const ALL: [Self; 3] = [Self::Guess, Self::Unsure, Self::Sure];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Guess => "guess",
            Self::Unsure => "unsure",
            Self::Sure => "sure",
        }
    }

    /// The grade an objectively checked answer earns: wrong is Again; a right guess is Hard
    /// (it was barely retrieved); anything else right is Good (recognition is easier than
    /// recall, so a quiz never earns Easy).
    pub fn grade_for(confidence: Option<Self>, correct: bool) -> Grade {
        match (correct, confidence) {
            (false, _) => Grade::Again,
            (true, Some(Self::Guess)) => Grade::Hard,
            (true, _) => Grade::Good,
        }
    }
}
