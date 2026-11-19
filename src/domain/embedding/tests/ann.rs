//! The USearch graphs: they agree with an exhaustive scan, follow index changes in place,
//! and never leak changes into a shared copy of the index.

use super::*;
use crate::domain::embedding::ann::EXACT_LIMIT;

const MODEL: &str = "m";
const DIMENSIONS: usize = 24;

/// Deterministic pseudo-random unit vectors (a 64-bit LCG).
fn vectors(count: usize, seed: u64) -> Vec<Vec<f32>> {
    let mut state = seed;
    let mut next = move || {
        state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        ((state >> 33) as f32 / (1u64 << 31) as f32) - 0.5
    };
    (0..count).map(|_| l2_normalized((0..DIMENSIONS).map(|_| next()).collect())).collect()
}

fn index_of(vectors: &[Vec<f32>]) -> VectorIndex {
    VectorIndex::from_rows(vectors.iter().enumerate().map(|(i, v)| (post(&format!("p{i}")), stored(0, MODEL, "h", v))))
}

/// The `k` best owners by brute force.
fn brute_force(vectors: &[Vec<f32>], query: &[f32], k: usize) -> Vec<EmbeddingOwner> {
    let mut scored: Vec<(f32, usize)> = vectors.iter().enumerate().map(|(i, v)| (cosine(query, v), i)).collect();
    scored.sort_by(|a, b| b.0.total_cmp(&a.0));
    scored.into_iter().take(k).map(|(_, i)| post(&format!("p{i}"))).collect()
}

#[test]
fn a_large_index_searches_the_graph_with_high_recall() {
    let data = vectors(EXACT_LIMIT + 1_000, 7);
    let index = index_of(&data);
    let queries = vectors(20, 99);
    let found: usize = queries
        .iter()
        .map(|query| {
            let expected = brute_force(&data, query, 10);
            top_k(&index, query, MODEL, 10, |_| true).iter().filter(|hit| expected.contains(&hit.owner)).count()
        })
        .sum();
    assert!(found >= 180, "recall@10 is {found}/200");
    let own = top_k(&index, &data[42], MODEL, 1, |_| true);
    assert_eq!(own[0].owner, post("p42"), "a stored vector finds itself");
    assert!((own[0].score - 1.0).abs() < 1e-5, "scores are exact cosine");
}

#[test]
fn a_small_index_is_searched_exhaustively() {
    let data = vectors(300, 3);
    let index = index_of(&data);
    for query in vectors(5, 11) {
        let hits: Vec<EmbeddingOwner> = top_k(&index, &query, MODEL, 8, |_| true).into_iter().map(|hit| hit.owner).collect();
        assert_eq!(hits, brute_force(&data, &query, 8));
    }
}

#[test]
fn the_graph_follows_changes_in_place() {
    let data = vectors(50, 5);
    let index = index_of(&data);
    assert!(!top_k(&index, &data[0], MODEL, 1, |_| true).is_empty(), "first search builds the graph");
    let fresh = vectors(1, 1234).remove(0);
    let index = index.with_chunks([(post("new"), stored(0, MODEL, "h", &fresh))]);
    assert_eq!(top_k(&index, &fresh, MODEL, 1, |_| true)[0].owner, post("new"));
    let index = index.without(&post("new"));
    assert_ne!(top_k(&index, &fresh, MODEL, 1, |_| true)[0].owner, post("new"));
    let filtered = top_k(&index, &data[3], MODEL, 1, |owner| owner != &post("p3"));
    assert_ne!(filtered[0].owner, post("p3"), "the filter applies to graph candidates");
}

#[test]
fn a_shared_copy_keeps_its_own_view() {
    let data = vectors(40, 8);
    let index = index_of(&data);
    let _ = top_k(&index, &data[0], MODEL, 1, |_| true);
    let snapshot = index.clone();
    let fresh = vectors(1, 4321).remove(0);
    let changed = index.with_chunks([(post("new"), stored(0, MODEL, "h", &fresh))]);
    assert_eq!(top_k(&changed, &fresh, MODEL, 1, |_| true)[0].owner, post("new"));
    assert_ne!(top_k(&snapshot, &fresh, MODEL, 1, |_| true)[0].owner, post("new"), "the snapshot never sees it");
}
