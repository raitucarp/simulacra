//! Plain-language level names for 0..=100 intelligence quotients.

use crate::state::forge::step::FORGE_I18N_ROOT;
use serde::{Deserialize, Serialize};

const LEVEL_GROUP: &str = "level";

/// A friendly rank for a quotient score, from Novice to Legendary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum QuotientLevel {
    Novice,
    Apprentice,
    Adept,
    Expert,
    Master,
    Legendary,
}

/// Lowest score of each level, in [`QuotientLevel::ALL`] order.
pub const LEVEL_THRESHOLDS: [u8; 6] = [0, 20, 40, 60, 80, 95];

impl QuotientLevel {
    pub const ALL: [Self; 6] = [
        Self::Novice,
        Self::Apprentice,
        Self::Adept,
        Self::Expert,
        Self::Master,
        Self::Legendary,
    ];

    /// The level a score falls into.
    pub fn from_score(score: u8) -> Self {
        Self::ALL
            .into_iter()
            .zip(LEVEL_THRESHOLDS)
            .rev()
            .find_map(|(level, threshold)| (score >= threshold).then_some(level))
            .unwrap_or(Self::Novice)
    }

    /// Lowest score of this level.
    pub fn threshold(self) -> u8 {
        LEVEL_THRESHOLDS[self as usize]
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Novice => "novice",
            Self::Apprentice => "apprentice",
            Self::Adept => "adept",
            Self::Expert => "expert",
            Self::Master => "master",
            Self::Legendary => "legendary",
        }
    }

    /// i18n key of the level name: `forge.level.{key}`.
    pub fn label_key(self) -> String {
        format!("{FORGE_I18N_ROOT}.{LEVEL_GROUP}.{}", self.key())
    }
}
