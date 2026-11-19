//! Text embedding providers: Gemini (`batchEmbedContents`), a local server (`/api/embed`,
//! which serves Google's Gemma embedding model locally), OpenAI-compatible services, and an
//! offline mock for tests.
//!
//! - [`config`]: the resolved provider, model, endpoint, key and limits.
//! - [`task`]: task types and how each model receives them (field or text prefix).
//! - [`wire`]: pure request builders and reply parsers per provider.
//! - [`error`] / [`backoff`]: failure classification and retry timing.
//! - [`client`]: batching, retries and dispatch.
//! - [`mock`]: deterministic offline vectors.

pub mod backoff;
pub mod client;
pub mod config;
pub mod error;
pub mod mock;
pub mod task;
pub mod wire;

#[cfg(test)]
mod tests;

pub use client::{embed_blocking, EmbeddingClient};
pub use config::{EmbeddingConfig, ProviderKind};
pub use error::EmbeddingError;
pub use task::{TaskEncoding, TaskType};
pub use wire::EmbedInput;
