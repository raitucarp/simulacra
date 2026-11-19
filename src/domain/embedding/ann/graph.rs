//! One model's USearch graph: chunk vectors under `u64` keys, the key ↔ chunk mapping, and
//! nearest-neighbour lookups (exhaustive while small, HNSW beyond [`EXACT_LIMIT`]).

use super::ChunkRef;
use crate::domain::embedding::EmbeddingOwner;
use std::collections::HashMap;
use usearch::{Index, IndexOptions, MetricKind, ScalarKind};

/// Up to this many vectors a search scans them all: exact, and as fast as walking the graph.
pub const EXACT_LIMIT: usize = 2_048;
/// HNSW links per node, and the candidate lists kept while inserting and searching.
const CONNECTIVITY: usize = 16;
const EXPANSION_ADD: usize = 128;
const EXPANSION_SEARCH: usize = 96;
/// Smallest reservation; capacity doubles from here as vectors arrive.
const MIN_CAPACITY: usize = 64;

/// The graph of one embedding model's chunk vectors.
pub struct ModelGraph {
    index: Index,
    /// Chunk of each key (`None` once removed; keys are never reused).
    slots: Vec<Option<ChunkRef>>,
    keys: HashMap<ChunkRef, u64>,
}

impl ModelGraph {
    /// An empty graph of `dimensions`-long vectors (half-precision storage, cosine metric;
    /// callers re-score the candidates exactly).
    pub fn new(dimensions: usize) -> Option<Self> {
        let options = IndexOptions {
            dimensions,
            metric: MetricKind::Cos,
            quantization: ScalarKind::F16,
            connectivity: CONNECTIVITY,
            expansion_add: EXPANSION_ADD,
            expansion_search: EXPANSION_SEARCH,
            multi: false,
        };
        let index = Index::new(&options).ok()?;
        Some(Self { index, slots: Vec::new(), keys: HashMap::new() })
    }

    pub fn dimensions(&self) -> usize {
        self.index.dimensions()
    }

    /// Live vectors.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Adds (or replaces) `chunk`'s vector; a vector of another length is not stored.
    pub fn insert(&mut self, chunk: ChunkRef, vector: &[f32]) {
        self.remove(&chunk);
        if vector.len() != self.dimensions() || !self.has_room() {
            return;
        }
        let key = self.slots.len() as u64;
        if self.index.add(key, vector).is_ok() {
            self.slots.push(Some(chunk.clone()));
            self.keys.insert(chunk, key);
        }
    }

    /// Makes room for one more vector, doubling the reservation when full.
    fn has_room(&self) -> bool {
        let needed = self.index.size() + 1;
        needed <= self.index.capacity() || self.index.reserve((needed * 2).max(MIN_CAPACITY)).is_ok()
    }

    /// Drops `chunk`'s vector (nothing when it has none).
    pub fn remove(&mut self, chunk: &ChunkRef) {
        if let Some(key) = self.keys.remove(chunk) {
            let _ = self.index.remove(key);
            self.slots[key as usize] = None;
        }
    }

    /// Drops every chunk failing `keep`.
    pub fn retain(&mut self, keep: impl Fn(&ChunkRef) -> bool) {
        let gone: Vec<ChunkRef> = self.keys.keys().filter(|chunk| !keep(chunk)).cloned().collect();
        gone.iter().for_each(|chunk| self.remove(chunk));
    }

    /// Chunks nearest `query` whose owner passes `keep`, nearest first: every passing chunk
    /// while the graph is small, else up to `count` from the HNSW search. `None` when the
    /// query has another length or the search fails.
    pub fn nearest(&self, query: &[f32], count: usize, keep: &dyn Fn(&EmbeddingOwner) -> bool) -> Option<Vec<ChunkRef>> {
        if query.len() != self.dimensions() {
            return None;
        }
        if self.is_empty() {
            return Some(Vec::new());
        }
        let passes = |key: u64| self.slot(key).is_some_and(|(owner, _)| keep(owner));
        let matches = if self.len() <= EXACT_LIMIT {
            self.index.exact_search(query, self.index.size()).ok()?
        } else {
            self.index.filtered_search(query, count, passes).ok()?
        };
        Some(matches.keys.into_iter().filter(|&key| passes(key)).filter_map(|key| self.slot(key).cloned()).collect())
    }

    fn slot(&self, key: u64) -> Option<&ChunkRef> {
        self.slots.get(key as usize).and_then(Option::as_ref)
    }
}
