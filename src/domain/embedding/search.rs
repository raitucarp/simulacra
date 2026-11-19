//! Cosine top-k over the index: each owner scores by its best matching chunk.

use super::index::VectorIndex;
use super::owner::EmbeddingOwner;
use super::vector::cosine;
use std::collections::HashMap;

/// One ranked owner.
#[derive(Debug, Clone, PartialEq)]
pub struct ScoredOwner {
    pub owner: EmbeddingOwner,
    /// Cosine similarity of the best chunk, in `[-1, 1]`.
    pub score: f32,
    /// Index of that chunk (for the excerpt).
    pub best_chunk: usize,
}

/// Chunk candidates asked of the graph per wanted owner (owners have several chunks).
const CHUNKS_PER_OWNER: usize = 4;
/// Fewest chunk candidates asked for.
const MIN_CANDIDATES: usize = 64;

/// The `k` owners most similar to `query` among chunks of `model` that pass `keep`, best
/// first (ties keep owner order, so results are deterministic). Candidates come from the
/// index's USearch graph and are scored with exact cosine.
pub fn top_k(
    index: &VectorIndex,
    query: &[f32],
    model: &str,
    k: usize,
    keep: impl Fn(&EmbeddingOwner) -> bool,
) -> Vec<ScoredOwner> {
    if k == 0 {
        return Vec::new();
    }
    let wanted = (k * CHUNKS_PER_OWNER).max(MIN_CANDIDATES);
    let mut best: HashMap<EmbeddingOwner, (f32, usize)> = HashMap::new();
    for (owner, at) in index.nearest_chunks(query, model, wanted, &keep) {
        let Some(chunk) = index.chunk(&owner, at) else { continue };
        let score = cosine(query, &chunk.vector);
        let slot = best.entry(owner).or_insert((score, at));
        if score > slot.0 {
            *slot = (score, at);
        }
    }
    let mut ranked: Vec<ScoredOwner> = best.into_iter().map(|(owner, (score, best_chunk))| ScoredOwner { owner, score, best_chunk }).collect();
    ranked.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.owner.cmp(&b.owner)));
    ranked.truncate(k);
    ranked
}
