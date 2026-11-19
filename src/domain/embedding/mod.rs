//! Text embeddings, pure side: what gets embedded and how it is searched.
//!
//! - [`owner`]: what a vector belongs to (a post or a vault document).
//! - [`sentence`] / [`chunk`]: sentence-aware chunking with overlap, by tokens or characters.
//! - [`hash`]: the stable content hash that lets unchanged text skip re-embedding.
//! - [`vector`]: normalisation, cosine and the `f32` little-endian blob encoding.
//! - [`index`]: the in-memory vector index (loaded once from SQLite, updated in place).
//! - [`ann`]: USearch (HNSW) graphs that find nearest-neighbour candidates per model.
//! - [`search`] / [`similar`]: cosine top-k by query vector or by an indexed item.
//! - [`plan`]: which chunks are new or stale, which rows to trim or remove.
//! - [`queue`]: the background indexing queue as a pure state machine.
//! - [`budget`]: the daily request and token budget.
//! - [`pca`]: a 2D projection (power iteration) for the embedding map.
//!
//! Limits: every vector lives in memory (10 000 chunks of 768 dimensions take about 30 MB,
//! plus half that for the graph). Up to [`ann::EXACT_LIMIT`] chunks per model a search is
//! exhaustive; past that the HNSW graph answers in well under a millisecond, approximately,
//! and candidates are re-scored exactly.

pub mod ann;
pub mod budget;
pub mod chunk;
pub mod failure;
pub mod hash;
pub mod index;
pub mod neighbours;
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
pub use neighbours::{document_neighbours, NEIGHBOURS_PER_DOCUMENT};
pub use owner::{EmbeddingOwner, OwnerKind};
pub use pca::{project_2d, Projection};
pub use plan::{plan_index, ChunkJob, IndexPlan, PlanSpec, SourceText};
pub use queue::{Batch, IndexQueue, Progress, QueueStatus};
pub use search::{top_k, ScoredOwner};
pub use similar::{centroid, owner_vectors, similar_documents, similar_posts, similar_to};
pub use vector::{cosine, decode_f32_le, dot, encode_f32_le, l2_normalized, mean_vector};
