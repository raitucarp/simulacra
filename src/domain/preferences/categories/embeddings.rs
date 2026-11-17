//! Embeddings: provider and model, how documents are chunked, and semantic search.
//!
//! The Gemini API serves `gemini-embedding-001` and `gemini-embedding-2`; Google's Gemma
//! embedding model (EmbeddingGemma) is open weights and is not served there, so it runs on a
//! local server (`embeddinggemma`). Each provider therefore has its own default model.

use super::super::bounds::Bounds;
use super::super::choices::EmbeddingProvider;
use serde::{Deserialize, Serialize};

/// The default model of the default provider (Gemini): text embedding with task types.
pub const DEFAULT_EMBEDDING_MODEL: &str = "gemini-embedding-001";
/// Google's Gemma embedding model as a local server names it.
pub const LOCAL_EMBEDDING_MODEL: &str = "embeddinggemma";
/// The OpenAI embedding model used when that provider is picked.
pub const OPENAI_EMBEDDING_MODEL: &str = "text-embedding-3-small";
/// Where a local embedding server listens unless the endpoint says otherwise.
pub const DEFAULT_LOCAL_ENDPOINT: &str = "http://localhost:11434";
/// Tokens per chunk.
pub const CHUNK_SIZE: Bounds = Bounds::new(128, 4096, 64);
/// Tokens shared by neighbouring chunks (at most half a chunk).
pub const CHUNK_OVERLAP: Bounds = Bounds::new(0, 1024, 16);
/// Vector length asked of the model (Matryoshka models such as Gemini and EmbeddingGemma
/// can shorten their vectors; 768 keeps quality high and storage small).
pub const DIMENSIONS: Bounds = Bounds::new(128, 3072, 128);
const DEFAULT_CHUNK_SIZE: u32 = 512;
const DEFAULT_CHUNK_OVERLAP: u32 = 64;
const DEFAULT_DIMENSIONS: u32 = 768;
/// The overlap may cover at most `1 / OVERLAP_DIVISOR` of a chunk.
const OVERLAP_DIVISOR: u32 = 2;

/// Embedding preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct EmbeddingPrefs {
    pub provider: EmbeddingProvider,
    /// Model id; blank falls back to the provider's default ([`default_model`]).
    pub model: String,
    pub chunk_size: u32,
    pub chunk_overlap: u32,
    pub semantic_search: bool,
    /// Base URL of the local server (and of a custom OpenAI-compatible service); blank is
    /// [`DEFAULT_LOCAL_ENDPOINT`].
    pub endpoint: String,
    /// Vector length asked of the model.
    pub dimensions: u32,
}

impl Default for EmbeddingPrefs {
    fn default() -> Self {
        Self {
            provider: EmbeddingProvider::default(),
            model: DEFAULT_EMBEDDING_MODEL.to_string(),
            chunk_size: DEFAULT_CHUNK_SIZE,
            chunk_overlap: DEFAULT_CHUNK_OVERLAP,
            semantic_search: true,
            endpoint: String::new(),
            dimensions: DEFAULT_DIMENSIONS,
        }
    }
}

/// The model a provider starts with.
pub const fn default_model(provider: EmbeddingProvider) -> &'static str {
    match provider {
        EmbeddingProvider::Gemini => DEFAULT_EMBEDDING_MODEL,
        EmbeddingProvider::LocalServer => LOCAL_EMBEDDING_MODEL,
        EmbeddingProvider::OpenAi | EmbeddingProvider::Custom => OPENAI_EMBEDDING_MODEL,
    }
}

impl EmbeddingPrefs {
    /// Clamps the chunk geometry (overlap at most half a chunk) and the dimensions; a blank
    /// model is the provider's default.
    pub fn validated(self) -> Self {
        let chunk_size = CHUNK_SIZE.clamp_u32(self.chunk_size);
        let chunk_overlap = CHUNK_OVERLAP.clamp_u32(self.chunk_overlap).min(chunk_size / OVERLAP_DIVISOR);
        let model = match self.model.trim() {
            "" => default_model(self.provider).to_string(),
            trimmed => trimmed.to_string(),
        };
        let endpoint = self.endpoint.trim().to_string();
        let dimensions = DIMENSIONS.clamp_u32(self.dimensions);
        Self { chunk_size, chunk_overlap, model, endpoint, dimensions, ..self }
    }

    /// These preferences with `provider` picked: a model still at the old provider's
    /// default moves to the new provider's default (a model the user typed stays).
    pub fn with_provider(self, provider: EmbeddingProvider) -> Self {
        let model = if self.model.trim().is_empty() || self.model == default_model(self.provider) {
            default_model(provider).to_string()
        } else {
            self.model
        };
        Self { provider, model, ..self }
    }

    /// The local server's base URL.
    pub fn endpoint_or_default(&self) -> &str {
        match self.endpoint.trim() {
            "" => DEFAULT_LOCAL_ENDPOINT,
            endpoint => endpoint,
        }
    }
}
