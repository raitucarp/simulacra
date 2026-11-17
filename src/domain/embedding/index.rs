//! The in-memory vector index: every stored chunk vector by owner, loaded once from the
//! `embeddings` table and updated by value as batches finish.

use super::owner::{EmbeddingOwner, OwnerKind};
use std::collections::BTreeMap;

/// One embedded chunk of an owner.
#[derive(Debug, Clone, PartialEq)]
pub struct StoredChunk {
    pub index: usize,
    /// Model id the vector came from (vectors of different models are never compared).
    pub model: String,
    /// [`super::content_hash`] of the embedded input; unchanged input is never re-embedded.
    pub content_hash: String,
    /// Unit-length vector.
    pub vector: Vec<f32>,
}

/// Every stored vector, by owner, chunks in index order.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VectorIndex {
    owners: BTreeMap<EmbeddingOwner, Vec<StoredChunk>>,
}

impl VectorIndex {
    /// An index over loaded rows.
    pub fn from_rows(rows: impl IntoIterator<Item = (EmbeddingOwner, StoredChunk)>) -> Self {
        Self::default().with_chunks(rows)
    }

    /// The chunks of `owner`, in index order (empty when it has none).
    pub fn chunks(&self, owner: &EmbeddingOwner) -> &[StoredChunk] {
        self.owners.get(owner).map_or(&[], Vec::as_slice)
    }

    /// Chunk `index` of `owner`.
    pub fn chunk(&self, owner: &EmbeddingOwner, index: usize) -> Option<&StoredChunk> {
        self.chunks(owner).iter().find(|chunk| chunk.index == index)
    }

    /// Every (owner, chunk) pair.
    pub fn iter(&self) -> impl Iterator<Item = (&EmbeddingOwner, &StoredChunk)> {
        self.owners.iter().flat_map(|(owner, chunks)| chunks.iter().map(move |chunk| (owner, chunk)))
    }

    /// Every owner with at least one chunk.
    pub fn owners(&self) -> impl Iterator<Item = &EmbeddingOwner> {
        self.owners.keys()
    }

    /// Owners of `kind`.
    pub fn owners_of(&self, kind: OwnerKind) -> impl Iterator<Item = &EmbeddingOwner> {
        self.owners().filter(move |owner| owner.kind == kind)
    }

    /// Number of stored chunks.
    pub fn len(&self) -> usize {
        self.owners.values().map(Vec::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    /// Number of chunks embedded with `model`.
    pub fn count_for_model(&self, model: &str) -> usize {
        self.iter().filter(|(_, chunk)| chunk.model == model).count()
    }

    /// The index with `rows` inserted, replacing chunks with the same owner and index.
    pub fn with_chunks(mut self, rows: impl IntoIterator<Item = (EmbeddingOwner, StoredChunk)>) -> Self {
        for (owner, chunk) in rows {
            let chunks = self.owners.entry(owner).or_default();
            chunks.retain(|existing| existing.index != chunk.index);
            let at = chunks.partition_point(|existing| existing.index < chunk.index);
            chunks.insert(at, chunk);
        }
        self
    }

    /// The index keeping only the first `keep` chunks of `owner` (none removes the owner).
    pub fn truncated(mut self, owner: &EmbeddingOwner, keep: usize) -> Self {
        if let Some(chunks) = self.owners.get_mut(owner) {
            chunks.retain(|chunk| chunk.index < keep);
            if chunks.is_empty() {
                self.owners.remove(owner);
            }
        }
        self
    }

    /// The index without `owner`.
    pub fn without(self, owner: &EmbeddingOwner) -> Self {
        self.truncated(owner, 0)
    }
}
