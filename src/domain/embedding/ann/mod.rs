//! Approximate nearest-neighbour search with USearch (HNSW), one graph per embedding model.
//!
//! [`super::VectorIndex`] stays the store of the vectors (loaded from SQLite, saved there);
//! the graphs only find candidates fast. A model's graph is built from the index on its first
//! search and then follows every change in place, unless the index value is shared (a clone
//! handed to a background task), in which case the changed copy rebuilds on its next search.
//! Callers re-score the candidates with exact cosine, so results stay exact for small indexes
//! (see [`EXACT_LIMIT`]) and deterministic at any size.

mod graph;

pub use graph::{ModelGraph, EXACT_LIMIT};

use super::owner::EmbeddingOwner;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, PoisonError};

/// One chunk: its owner and chunk index.
pub type ChunkRef = (EmbeddingOwner, usize);

/// The built graphs, by model id.
#[derive(Clone, Default)]
pub struct AnnCache(Arc<Mutex<HashMap<String, ModelGraph>>>);

impl AnnCache {
    /// Runs `search` on `model`'s graph, building it with `build` first when needed.
    /// `None` when no graph can be built.
    pub fn with_graph<R>(&self, model: &str, build: impl FnOnce() -> Option<ModelGraph>, search: impl FnOnce(&ModelGraph) -> R) -> Option<R> {
        let mut graphs = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if !graphs.contains_key(model) {
            graphs.insert(model.to_owned(), build()?);
        }
        graphs.get(model).map(search)
    }

    /// Applies `change` to every built graph when this cache is not shared; otherwise starts
    /// an empty cache (rebuilt on demand) so other holders keep their own view.
    pub fn update(&mut self, change: impl Fn(&str, &mut ModelGraph)) {
        match Arc::get_mut(&mut self.0) {
            Some(graphs) => {
                let graphs = graphs.get_mut().unwrap_or_else(PoisonError::into_inner);
                graphs.iter_mut().for_each(|(model, graph)| change(model, graph));
            }
            None => *self = Self::default(),
        }
    }

    /// Number of built graphs (tests and diagnostics).
    pub fn built(&self) -> usize {
        self.0.lock().unwrap_or_else(PoisonError::into_inner).len()
    }
}

impl fmt::Debug for AnnCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AnnCache({} graphs)", self.built())
    }
}

/// A cache is derived data: two indexes with the same vectors are equal whatever is built.
impl PartialEq for AnnCache {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
