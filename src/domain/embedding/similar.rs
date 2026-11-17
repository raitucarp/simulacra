//! "Find similar" over already indexed items (no network call), and the vector export the
//! embedding map projects.
//!
//! Seams for other features: [`similar_documents`] and [`similar_posts`] (related entries on
//! a vault entry page, related thoughts), and [`owner_vectors`] (the 2D embedding map).

use super::index::VectorIndex;
use super::owner::{EmbeddingOwner, OwnerKind};
use super::search::{top_k, ScoredOwner};
use super::vector::{l2_normalized, mean_vector};
use crate::domain::PostId;

/// The `k` owners closest to `owner` (by the mean of its chunks), excluding itself and any
/// owner failing `keep`. Empty when `owner` has no vector of `model`.
pub fn similar_to(
    index: &VectorIndex,
    owner: &EmbeddingOwner,
    model: &str,
    k: usize,
    keep: impl Fn(&EmbeddingOwner) -> bool,
) -> Vec<ScoredOwner> {
    let Some(centroid) = centroid(index, owner, model) else { return Vec::new() };
    top_k(index, &centroid, model, k, |candidate| candidate != owner && keep(candidate))
}

/// Documents most similar to document `doc_id`.
pub fn similar_documents(index: &VectorIndex, model: &str, doc_id: &str, k: usize) -> Vec<ScoredOwner> {
    let owner = EmbeddingOwner::document(doc_id);
    similar_to(index, &owner, model, k, |candidate| candidate.kind == OwnerKind::Document)
}

/// Posts most similar to post `post_id`.
pub fn similar_posts(index: &VectorIndex, model: &str, post_id: &PostId, k: usize) -> Vec<ScoredOwner> {
    let owner = EmbeddingOwner::post(post_id);
    similar_to(index, &owner, model, k, |candidate| candidate.kind == OwnerKind::Post)
}

/// One vector per owner of `kind` embedded with `model` (the unit mean of its chunks).
pub fn owner_vectors(index: &VectorIndex, kind: OwnerKind, model: &str) -> Vec<(EmbeddingOwner, Vec<f32>)> {
    index
        .owners_of(kind)
        .filter_map(|owner| centroid(index, owner, model).map(|vector| (owner.clone(), vector)))
        .collect()
}

/// The unit mean of `owner`'s chunks of `model`.
fn centroid(index: &VectorIndex, owner: &EmbeddingOwner, model: &str) -> Option<Vec<f32>> {
    let vectors: Vec<&[f32]> = index.chunks(owner).iter().filter(|c| c.model == model).map(|c| c.vector.as_slice()).collect();
    mean_vector(&vectors).map(l2_normalized)
}
