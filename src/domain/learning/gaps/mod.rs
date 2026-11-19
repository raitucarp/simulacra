//! The knowledge-gap radar: for each concept the learner refers to, how well it is held on
//! six dimensions, and the weakest concepts first.
//!
//! - [`evidence`]: what is known about each concept (own posts, vault sources, review items,
//!   tutor mastery, co-occurring concepts, last touch), from plain inputs.
//! - [`score`]: the dimension scores (0–1), their documented weights and the ranking.

pub mod evidence;
pub mod score;

pub use evidence::{collect_evidence, ConceptEvidence, GapInputs, ItemText, SourceText};
pub use score::{gap_report, strength, DIMENSION_WEIGHTS};

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// One axis of the radar.
    GapDimension {
        /// The learner's own posts on it.
        Writing => "writing",
        /// Vault documents on it.
        Sources => "sources",
        /// Recall of its review items (FSRS retrievability).
        Retention => "retention",
        /// The highest Bloom level shown with the tutor.
        Mastery => "mastery",
        /// Other concepts it appears with.
        Connections => "connections",
        /// How recently it was written about or reviewed.
        Recency => "recency",
    }
}

/// Number of radar dimensions.
pub const DIMENSIONS: usize = 6;

/// Scores per dimension, in [`GapDimension::ALL`] order.
pub type DimensionScores = [f64; DIMENSIONS];

/// One concept on the radar.
#[derive(Debug, Clone, PartialEq)]
pub struct GapEntry {
    /// The concept's key.
    pub concept: String,
    /// Texts that refer to it (its weight among equals).
    pub mentions: usize,
    pub scores: DimensionScores,
    /// Weighted strength, 0–1 (the gap is its complement).
    pub strength: f64,
}

impl GapEntry {
    pub fn gap(&self) -> f64 {
        1.0 - self.strength
    }

    /// The dimension with the lowest score (the first such, in dimension order).
    pub fn weakest(&self) -> GapDimension {
        let at = (0..DIMENSIONS).fold(0, |best, i| if self.scores[i] < self.scores[best] { i } else { best });
        GapDimension::ALL[at]
    }

    pub fn score(&self, dimension: GapDimension) -> f64 {
        GapDimension::ALL.iter().position(|d| *d == dimension).map_or(0.0, |at| self.scores[at])
    }
}

/// The radar's data: every concept, weakest first, and the mean of each dimension.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GapReport {
    pub entries: Vec<GapEntry>,
    pub average: DimensionScores,
}

impl GapReport {
    /// The `count` weakest concepts.
    pub fn weakest(&self, count: usize) -> &[GapEntry] {
        &self.entries[..count.min(self.entries.len())]
    }

    pub fn entry(&self, concept: &str) -> Option<&GapEntry> {
        self.entries.iter().find(|entry| entry.concept == concept)
    }
}
