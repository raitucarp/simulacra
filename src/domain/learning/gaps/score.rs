//! Dimension scores (each 0–1), their weights and the ranking of the radar.
//!
//! | Dimension | Score | Weight |
//! |---|---|---|
//! | writing | own posts `n / (n + 2)`: two posts make one half | 0.20 |
//! | sources | vault documents `n / (n + 1)`: one source makes one half | 0.15 |
//! | retention | mean FSRS retrievability of its review items (0 without items) | 0.25 |
//! | mastery | highest Bloom level shown with the tutor, as a share of the ladder | 0.20 |
//! | connections | co-occurring concepts `n / (n + 3)` | 0.10 |
//! | recency | `0.5^(days / 14)` since it was last written about or reviewed (0 if never) | 0.10 |
//!
//! Strength is the weighted sum; the gap is `1 − strength`. Weakest first; among equals the
//! more mentioned concept (it matters more to the learner) comes first, then by name.

use super::evidence::{collect_evidence, ConceptEvidence, GapInputs};
use super::{DimensionScores, GapEntry, GapReport, DIMENSIONS};
use crate::domain::learning::bloom::BloomLevel;
use crate::domain::review::day::days_between;
use chrono::NaiveDate;

/// Weights in [`super::GapDimension::ALL`] order; they sum to one.
pub const DIMENSION_WEIGHTS: DimensionScores = [0.20, 0.15, 0.25, 0.20, 0.10, 0.10];
/// Counts at which writing, sources and connections score one half.
pub const WRITING_HALF: f64 = 2.0;
pub const SOURCES_HALF: f64 = 1.0;
pub const CONNECTIONS_HALF: f64 = 3.0;
/// Days after which recency halves.
pub const RECENCY_HALF_LIFE_DAYS: f64 = 14.0;
const HALF: f64 = 0.5;

/// `count / (count + half)`: zero at zero, one half at `half`, approaching one.
pub fn saturate(count: usize, half: f64) -> f64 {
    let n = count as f64;
    n / (n + half)
}

/// Recency on `today` of a concept last touched on `last`.
pub fn recency(last: Option<NaiveDate>, today: NaiveDate) -> f64 {
    last.map_or(0.0, |day| HALF.powf(f64::from(days_between(day, today)) / RECENCY_HALF_LIFE_DAYS))
}

/// The six scores of `evidence` on `today`.
pub fn dimension_scores(evidence: &ConceptEvidence, today: NaiveDate) -> DimensionScores {
    [
        saturate(evidence.own_posts, WRITING_HALF),
        saturate(evidence.sources, SOURCES_HALF),
        evidence.retention.unwrap_or(0.0).clamp(0.0, 1.0),
        evidence.mastery.map_or(0.0, BloomLevel::share),
        saturate(evidence.degree, CONNECTIONS_HALF),
        recency(evidence.last_touch, today),
    ]
}

/// The weighted strength of `scores`.
pub fn strength(scores: &DimensionScores) -> f64 {
    scores.iter().zip(DIMENSION_WEIGHTS).map(|(score, weight)| score * weight).sum()
}

/// Entries sorted weakest first (ties: more mentions, then name).
pub fn ranked(mut entries: Vec<GapEntry>) -> Vec<GapEntry> {
    entries.sort_by(|a, b| a.strength.total_cmp(&b.strength).then(b.mentions.cmp(&a.mentions)).then(a.concept.cmp(&b.concept)));
    entries
}

/// The mean score of each dimension over `entries`.
pub fn average(entries: &[GapEntry]) -> DimensionScores {
    let count = entries.len().max(1) as f64;
    let mut sums = [0.0; DIMENSIONS];
    entries.iter().for_each(|entry| (0..DIMENSIONS).for_each(|d| sums[d] += entry.scores[d]));
    sums.map(|sum| sum / count)
}

/// The radar of `inputs`.
pub fn gap_report(inputs: &GapInputs<'_>) -> GapReport {
    let entries = collect_evidence(inputs)
        .into_iter()
        .map(|evidence| {
            let scores = dimension_scores(&evidence, inputs.today);
            GapEntry { concept: evidence.concept, mentions: evidence.mentions, strength: strength(&scores), scores }
        })
        .collect::<Vec<_>>();
    let entries = ranked(entries);
    GapReport { average: average(&entries), entries }
}
