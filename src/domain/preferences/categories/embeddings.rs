//! Embeddings: provider and model, how documents are chunked, and semantic search.

use super::super::bounds::Bounds;
use super::super::choices::EmbeddingProvider;
use serde::{Deserialize, Serialize};

/// The Gemma embedding model from Google, served through the Gemini API.
pub const DEFAULT_EMBEDDING_MODEL: &str = "embeddinggemma";
/// Tokens per chunk.
pub const CHUNK_SIZE: Bounds = Bounds::new(128, 4096, 64);
/// Tokens shared by neighbouring chunks (at most half a chunk).
pub const CHUNK_OVERLAP: Bounds = Bounds::new(0, 1024, 16);
const DEFAULT_CHUNK_SIZE: u32 = 512;
const DEFAULT_CHUNK_OVERLAP: u32 = 64;
/// The overlap may cover at most `1 / OVERLAP_DIVISOR` of a chunk.
const OVERLAP_DIVISOR: u32 = 2;

/// Embedding preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmbeddingPrefs {
    /// Consumed by a later wave.
    pub provider: EmbeddingProvider,
    /// Model id; blank falls back to [`DEFAULT_EMBEDDING_MODEL`]. Consumed by a later wave.
    pub model: String,
    /// Consumed by a later wave.
    pub chunk_size: u32,
    /// Consumed by a later wave.
    pub chunk_overlap: u32,
    /// Consumed by a later wave.
    pub semantic_search: bool,
}

impl Default for EmbeddingPrefs {
    fn default() -> Self {
        Self {
            provider: EmbeddingProvider::default(),
            model: DEFAULT_EMBEDDING_MODEL.to_string(),
            chunk_size: DEFAULT_CHUNK_SIZE,
            chunk_overlap: DEFAULT_CHUNK_OVERLAP,
            semantic_search: true,
        }
    }
}

impl EmbeddingPrefs {
    /// Clamps the chunk geometry (overlap at most half a chunk); a blank model is the default.
    pub fn validated(self) -> Self {
        let chunk_size = CHUNK_SIZE.clamp_u32(self.chunk_size);
        let chunk_overlap = CHUNK_OVERLAP.clamp_u32(self.chunk_overlap).min(chunk_size / OVERLAP_DIVISOR);
        let model = match self.model.trim() {
            "" => DEFAULT_EMBEDDING_MODEL.to_string(),
            trimmed => trimmed.to_string(),
        };
        Self { chunk_size, chunk_overlap, model, ..self }
    }
}
