//! Ranking candidate posts for an interest: closeness of meaning blended with freshness.

use crate::domain::embedding::{centroid, cosine, top_k, EmbeddingOwner, OwnerKind, VectorIndex};
use crate::domain::PostId;
use jiff::Timestamp;
use std::collections::HashMap;

/// Candidates retrieved from the graph per recommendation wanted, and at least this many,
/// so freshness can still reorder them.
const POOL_FACTOR: usize = 5;
const MIN_POOL: usize = 50;
const SECONDS_PER_HOUR: f64 = 3_600.0;

/// A post that may be recommended.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub post: PostId,
    pub created_at: Timestamp,
}

/// A ranked post.
#[derive(Debug, Clone, PartialEq)]
pub struct Recommendation {
    pub post: PostId,
    /// The blended score (higher first).
    pub score: f32,
    /// Cosine of the post's meaning to the interest (0 when it is not embedded yet).
    pub similarity: f32,
}

/// How much closeness of meaning and freshness weigh in a score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Blend {
    pub similarity: f32,
    pub freshness: f32,
    /// Hours after which a post's freshness has halved.
    pub half_life_hours: f64,
}

impl Blend {
    /// The For You stream: mostly meaning, with a pull towards the last couple of days.
    pub const FEED: Self = Self { similarity: 0.75, freshness: 0.25, half_life_hours: 48.0 };
    /// The post an agent answers next: what it cares about, among recent posts.
    pub const AGENT: Self = Self { similarity: 0.8, freshness: 0.2, half_life_hours: 24.0 };
    /// A persona's recommendations: nearly all meaning, over about a week.
    pub const PERSONA: Self = Self { similarity: 0.9, freshness: 0.1, half_life_hours: 168.0 };

    fn score(self, similarity: f32, created_at: Timestamp, now: Timestamp) -> f32 {
        self.similarity * similarity + self.freshness * freshness(now, created_at, self.half_life_hours)
    }
}

/// 1 for a post made `now`, halving every `half_life_hours` (future times count as fresh).
pub fn freshness(now: Timestamp, at: Timestamp, half_life_hours: f64) -> f32 {
    let hours = now.duration_since(at).as_secs_f64().max(0.0) / SECONDS_PER_HOUR;
    0.5_f64.powf(hours / half_life_hours) as f32
}

/// Every candidate scored with `blend`, best first; posts not embedded yet score on freshness.
pub fn rank_all(index: &VectorIndex, model: &str, interest: &[f32], candidates: &[Candidate], now: Timestamp, blend: Blend) -> Vec<Recommendation> {
    let ranked = candidates.iter().map(|candidate| {
        let similarity = centroid(index, &EmbeddingOwner::post(&candidate.post), model).map_or(0.0, |vector| cosine(interest, &vector));
        (candidate, similarity)
    });
    sorted(ranked.map(|(candidate, similarity)| scored(candidate, similarity, now, blend)).collect())
}

/// The `k` best candidates: the ones closest in meaning, retrieved through the index's
/// USearch graph, then ranked with `blend`.
pub fn recommend(index: &VectorIndex, model: &str, interest: &[f32], candidates: &[Candidate], now: Timestamp, blend: Blend, k: usize) -> Vec<Recommendation> {
    let by_id: HashMap<&str, &Candidate> = candidates.iter().map(|c| (c.post.as_str(), c)).collect();
    let keep = |owner: &EmbeddingOwner| owner.kind == OwnerKind::Post && by_id.contains_key(owner.id.as_str());
    let pool = top_k(index, interest, model, (k * POOL_FACTOR).max(MIN_POOL), keep);
    let ranked = pool.iter().filter_map(|hit| by_id.get(hit.owner.id.as_str()).map(|candidate| scored(candidate, hit.score, now, blend)));
    let mut ranked = sorted(ranked.collect());
    ranked.truncate(k);
    ranked
}

fn scored(candidate: &Candidate, similarity: f32, now: Timestamp, blend: Blend) -> (Recommendation, Timestamp) {
    let score = blend.score(similarity, candidate.created_at, now);
    (Recommendation { post: candidate.post.clone(), score, similarity }, candidate.created_at)
}

/// Best score first; ties go to the newer post, then the post id.
fn sorted(mut ranked: Vec<(Recommendation, Timestamp)>) -> Vec<Recommendation> {
    ranked.sort_by(|(a, a_at), (b, b_at)| b.score.total_cmp(&a.score).then(b_at.cmp(a_at)).then_with(|| a.post.as_str().cmp(b.post.as_str())));
    ranked.into_iter().map(|(recommendation, _)| recommendation).collect()
}
