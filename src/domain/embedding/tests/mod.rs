mod budget;
mod chunking;
mod pca;
mod plan;
mod queue;
mod search;

use super::*;

/// A unit chunk of `model` with `vector`.
pub(super) fn stored(index: usize, model: &str, hash: &str, vector: &[f32]) -> StoredChunk {
    StoredChunk { index, model: model.into(), content_hash: hash.into(), vector: l2_normalized(vector.to_vec()) }
}

pub(super) fn doc(id: &str) -> EmbeddingOwner {
    EmbeddingOwner::document(id)
}

pub(super) fn post(id: &str) -> EmbeddingOwner {
    EmbeddingOwner::post(&crate::domain::PostId::new(id))
}
