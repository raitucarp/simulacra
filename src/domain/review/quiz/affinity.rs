//! Closeness in meaning between cards' sources, from the embedding index (sb B3): each
//! post or document is one unit vector (the mean of its chunks); cards without an embedded
//! source score zero.

use crate::domain::embedding::{cosine, owner_vectors, EmbeddingOwner, OwnerKind, VectorIndex};
use crate::domain::review::item::ReviewItem;
use std::collections::HashMap;

/// Unit vectors of every embedded post and document of `model`.
#[derive(Debug, Clone, Default)]
pub struct SourceVectors(HashMap<EmbeddingOwner, Vec<f32>>);

impl SourceVectors {
    pub fn from_index(index: &VectorIndex, model: &str) -> Self {
        Self(OwnerKind::ALL.into_iter().flat_map(|kind| owner_vectors(index, kind, model)).collect())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn vector_of(&self, card: &ReviewItem) -> Option<&Vec<f32>> {
        self.0.get(&card.subject.anchor()?.owner())
    }

    /// Cosine of the two cards' sources clamped to 0–1 (0 when either is not embedded).
    pub fn affinity(&self, a: &ReviewItem, b: &ReviewItem) -> f64 {
        match (self.vector_of(a), self.vector_of(b)) {
            (Some(x), Some(y)) => f64::from(cosine(x, y)).clamp(0.0, 1.0),
            _ => 0.0,
        }
    }
}
