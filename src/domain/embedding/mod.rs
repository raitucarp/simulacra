//! Text embeddings, pure side: what gets embedded and how it is searched.
//!
//! - [`owner`]: what a vector belongs to (a post or a vault document).
//! - [`sentence`] / [`chunk`]: sentence-aware chunking with overlap, by tokens or characters.
//! - [`hash`]: the stable content hash that lets unchanged text skip re-embedding.
//! - [`vector`]: normalisation, cosine and the `f32` little-endian blob encoding.
//! - [`index`]: the in-memory vector index (loaded once from SQLite, updated in place).
//! - [`search`] / [`similar`]: cosine top-k by query vector or by an indexed item.
//! - [`plan`]: which chunks are new or stale, which rows to trim or remove.
//! - [`queue`]: the background indexing queue as a pure state machine.
//! - [`budget`]: the daily request and token budget.
//! - [`pca`]: a 2D projection (power iteration) for the embedding map.
//!
//! Limits (personal scale): every vector lives in memory and search is a linear scan, so
//! 10 000 chunks of 768 dimensions take about 30 MB and a query about 10 ms. Past roughly
//! 100 000 chunks an approximate index would be needed.

pub mod budget;
pub mod chunk;
pub mod failure;
pub mod hash;
pub mod index;
pub mod owner;
pub mod pca;
pub mod plan;
pub mod queue;
pub mod search;
pub mod sentence;
pub mod similar;
pub mod vector;

#[cfg(test)]
mod tests;

pub use budget::{estimate_tokens, BudgetLimits, DailyUsage};
pub use chunk::{chunk_text, ChunkSpec, ChunkUnit, TextChunk};
pub use failure::FailureKind;
pub use hash::content_hash;
pub use index::{StoredChunk, VectorIndex};
pub use owner::{EmbeddingOwner, OwnerKind};
pub use pca::{project_2d, Projection};
pub use plan::{plan_index, ChunkJob, IndexPlan, PlanSpec, SourceText};
pub use queue::{Batch, IndexQueue, Progress, QueueStatus};
pub use search::{top_k, ScoredOwner};
pub use similar::{owner_vectors, similar_documents, similar_posts, similar_to};
pub use vector::{cosine, decode_f32_le, dot, encode_f32_le, l2_normalized, mean_vector};
