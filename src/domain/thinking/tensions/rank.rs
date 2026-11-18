//! Ranking candidate pairs. A contradiction looks like *the same topic with opposite
//! polarity*, so the score adds the two:
//!
//! `score = SIMILARITY_WEIGHT · topic + LEXICAL_WEIGHT · lexical.strength()`
//!
//! where `topic` is the cosine of a pair found by meaning, or the Jaccard overlap of the two
//! content-term sets for a pair found by words (both clamped to `[0, 1]`). A pair is kept
//! only if it shows a lexical signal, or if its sides are so close in meaning
//! (`STRONG_SIMILARITY`) that a quiet disagreement is plausible without one. Ties are broken
//! by pair key, so the order is deterministic.

use super::candidates::{candidate_pairs, CandidateInput, IndexPair};
use super::lexical::{lexical_signals, LexicalSignals};
use super::pair::pair_key;
use super::side::TensionSide;
use super::terms::{content_terms, jaccard};
use std::collections::BTreeSet;

/// Pairs returned when no limit is given.
pub const DEFAULT_LIMIT: usize = 60;
/// Least cosine for two items to count as the same topic.
pub const MIN_SIMILARITY: f32 = 0.55;
/// Cosine at which a pair is kept even without a lexical signal.
pub const STRONG_SIMILARITY: f32 = 0.85;
/// Weight of the shared topic in the score.
const SIMILARITY_WEIGHT: f32 = 0.45;
/// Weight of the lexical opposition in the score.
const LEXICAL_WEIGHT: f32 = 0.55;

/// What to rank.
#[derive(Debug, Clone, PartialEq)]
pub struct RankOptions {
    /// A side key: only pairs including it (that side then comes first).
    pub focus: Option<String>,
    pub limit: usize,
    pub min_similarity: f32,
}

impl Default for RankOptions {
    fn default() -> Self {
        Self { focus: None, limit: DEFAULT_LIMIT, min_similarity: MIN_SIMILARITY }
    }
}

/// A ranked pair that may contradict.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub left: TensionSide,
    pub right: TensionSide,
    /// Cosine of the pair when found by meaning.
    pub similarity: Option<f32>,
    pub lexical: LexicalSignals,
    pub score: f32,
}

impl Candidate {
    /// The pair's cache key ([`pair_key`]).
    pub fn key(&self) -> String {
        pair_key(&self.left, &self.right)
    }
}

/// The score of a pair with shared `topic` (0–1) and `lexical` signals.
pub fn candidate_score(topic: f32, lexical: &LexicalSignals) -> f32 {
    SIMILARITY_WEIGHT * topic.clamp(0.0, 1.0) + LEXICAL_WEIGHT * lexical.strength()
}

/// The best candidate pairs of `items`, best first.
pub fn rank_candidates(items: &[CandidateInput], options: &RankOptions) -> Vec<Candidate> {
    let focus = match &options.focus {
        Some(key) => match items.iter().position(|item| &item.side.key() == key) {
            Some(index) => Some(index),
            None => return Vec::new(),
        },
        None => None,
    };
    let terms: Vec<BTreeSet<String>> = items.iter().map(|item| content_terms(&item.side.text)).collect();
    let pairs = candidate_pairs(items, &terms, focus, options.min_similarity);
    let mut ranked: Vec<(String, Candidate)> = pairs
        .into_iter()
        .filter_map(|(pair, similarity)| candidate(items, &terms, pair, similarity, focus))
        .map(|candidate| (candidate.key(), candidate))
        .collect();
    ranked.sort_by(|a, b| b.1.score.total_cmp(&a.1.score).then_with(|| a.0.cmp(&b.0)));
    ranked.into_iter().take(options.limit).map(|(_, candidate)| candidate).collect()
}

/// The candidate of one pair, or `None` when it shows nothing worth a look.
fn candidate(items: &[CandidateInput], terms: &[BTreeSet<String>], (a, b): IndexPair, similarity: Option<f32>, focus: Option<usize>) -> Option<Candidate> {
    let (left, right) = if focus == Some(b) { (b, a) } else { (a, b) };
    let lexical = lexical_signals(&items[left].side.text, &items[right].side.text);
    let strong = similarity.is_some_and(|score| score >= STRONG_SIMILARITY);
    if lexical.is_empty() && !strong {
        return None;
    }
    let topic = similarity.unwrap_or_else(|| jaccard(&terms[left], &terms[right]));
    let score = candidate_score(topic, &lexical);
    Some(Candidate { left: items[left].side.clone(), right: items[right].side.clone(), similarity, lexical, score })
}
