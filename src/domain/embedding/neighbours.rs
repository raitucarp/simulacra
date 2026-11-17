//! Neighbours in meaning for every vault document: the embedding side of the vault's
//! "related entries" (which merges them with subject and discipline scores).

use super::index::VectorIndex;
use super::similar::similar_documents;
use crate::domain::document::ids::DocumentId;
use crate::domain::document::links::ScoredDocument;
use std::collections::HashMap;

/// Neighbours kept per document.
pub const NEIGHBOURS_PER_DOCUMENT: usize = 6;
/// The related-entries scale starts at zero; opposite meanings count as unrelated.
const MIN_RELATED_SCORE: f32 = 0.0;
const MAX_RELATED_SCORE: f32 = 1.0;

/// The `k` closest documents (by `model` vectors) of each of `doc_ids`, with cosine scores
/// clamped to the related-entries scale. Documents without vectors are left out.
pub fn document_neighbours<'a>(
    index: &VectorIndex,
    model: &str,
    doc_ids: impl IntoIterator<Item = &'a str>,
    k: usize,
) -> HashMap<String, Vec<ScoredDocument>> {
    doc_ids
        .into_iter()
        .map(|id| {
            let near = similar_documents(index, model, id, k)
                .into_iter()
                .map(|hit| ScoredDocument { id: DocumentId::new(hit.owner.id), score: hit.score.clamp(MIN_RELATED_SCORE, MAX_RELATED_SCORE) })
                .collect::<Vec<_>>();
            (id.to_string(), near)
        })
        .filter(|(_, near)| !near.is_empty())
        .collect()
}
