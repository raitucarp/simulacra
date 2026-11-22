//! Reciprocal rank fusion: a post at rank `r` (from 1) of a ranking earns `1 / (RRF_K + r)`,
//! and its score is the sum over the rankings it is in. Ranks, not raw scores, are fused, so
//! bm25 and cosine never have to share a scale.

use super::SearchHit;
use crate::domain::PostId;
use std::collections::HashMap;

/// The fusion constant: how much weight the top ranks get over the tail (the usual 60).
pub const RRF_K: f64 = 60.0;

fn share(rank: usize) -> f64 {
    1.0 / (RRF_K + (rank + 1) as f64)
}

/// The text and meaning rankings merged into one list, best first; ties keep the text order.
pub(super) fn fuse(text: &[PostId], meaning: &[PostId]) -> Vec<SearchHit> {
    let mut hits: Vec<SearchHit> = Vec::with_capacity(text.len() + meaning.len());
    let mut at: HashMap<&PostId, usize> = HashMap::new();
    for (by_meaning, ranking) in [(false, text), (true, meaning)] {
        for (rank, id) in ranking.iter().enumerate() {
            let slot = *at.entry(id).or_insert_with(|| {
                hits.push(SearchHit { post_id: id.clone(), score: 0.0, by_text: false, by_meaning: false });
                hits.len() - 1
            });
            let hit = &mut hits[slot];
            if hit.by_text && !by_meaning || hit.by_meaning && by_meaning {
                continue; // listed twice in one ranking: the better rank counts
            }
            hit.score += share(rank);
            hit.by_text |= !by_meaning;
            hit.by_meaning |= by_meaning;
        }
    }
    hits.sort_by(|a, b| b.score.total_cmp(&a.score));
    hits
}

/// Posts in the given order (a filter-only query), scored by their place.
pub(super) fn listed(ids: impl Iterator<Item = PostId>) -> Vec<SearchHit> {
    ids.enumerate().map(|(rank, post_id)| SearchHit { post_id, score: share(rank), by_text: false, by_meaning: false }).collect()
}
