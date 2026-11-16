//! Plain-language names for 0..=100 scores: quotient levels for intelligences, five bands
//! (very low → very high) for everything else.

use crate::state::forge::attributes::INTELLIGENCE_FIELDS;
use crate::state::forge::{QuotientLevel, ScoreField};

const BAND_ROOT: &str = "forge.ui.band";

/// A coarse description of a personality or style score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScoreBand {
    VeryLow,
    Low,
    Balanced,
    High,
    VeryHigh,
}

/// Lowest score of each band, in declaration order.
pub const BAND_THRESHOLDS: [u8; 5] = [0, 20, 40, 60, 80];

impl ScoreBand {
    pub const ALL: [Self; 5] = [Self::VeryLow, Self::Low, Self::Balanced, Self::High, Self::VeryHigh];

    /// The band a score falls into.
    pub fn from_score(score: u8) -> Self {
        Self::ALL
            .into_iter()
            .zip(BAND_THRESHOLDS)
            .rev()
            .find_map(|(band, threshold)| (score >= threshold).then_some(band))
            .unwrap_or(Self::VeryLow)
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::VeryLow => "very_low",
            Self::Low => "low",
            Self::Balanced => "balanced",
            Self::High => "high",
            Self::VeryHigh => "very_high",
        }
    }

    /// i18n key of the band name: `forge.ui.band.{key}`.
    pub fn label_key(self) -> String {
        format!("{BAND_ROOT}.{}", self.key())
    }
}

/// i18n key naming `score` on `field`: a quotient level for intelligences, a band otherwise.
pub fn score_level_key(field: ScoreField, score: u8) -> String {
    if INTELLIGENCE_FIELDS.contains(&field) {
        QuotientLevel::from_score(score).label_key()
    } else {
        ScoreBand::from_score(score).label_key()
    }
}
