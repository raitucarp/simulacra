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

/// The `k` owners most similar to `query` among chunks of `model` that pass `keep`, best
/// first (ties keep owner order, so results are deterministic).
pub fn top_k(
    index: &VectorIndex,
    query: &[f32],
    model: &str,
    k: usize,
    keep: impl Fn(&EmbeddingOwner) -> bool,
) -> Vec<ScoredOwner> {
    let mut best: HashMap<&EmbeddingOwner, (f32, usize)> = HashMap::new();
    let candidates = index
        .iter()
        .filter(|(_, chunk)| chunk.model == model && chunk.vector.len() == query.len())
        .filter(|(owner, _)| keep(owner));
    for (owner, chunk) in candidates {
        let score = cosine(query, &chunk.vector);
        let slot = best.entry(owner).or_insert((score, chunk.index));
        if score > slot.0 {
            *slot = (score, chunk.index);
        }
    }
    let mut ranked: Vec<ScoredOwner> = best
        .into_iter()
        .map(|(owner, (score, best_chunk))| ScoredOwner { owner: owner.clone(), score, best_chunk })
        .collect();
    ranked.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.owner.cmp(&b.owner)));
    ranked.truncate(k);
    ranked
}
