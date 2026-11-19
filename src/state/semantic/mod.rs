//! The store side of semantic search (sb B3): what is embedded, how ranked owners turn into
//! displayable hits, and the lexical (BM25) fallback.
//!
//! - [`sources`]: posts and documents as [`SourceText`]s, and the cheap change key.
//! - [`hits`]: [`SemanticHit`]s (title, excerpt, score) for owners of the active network.
//! - [`lexical`]: word-match hits when search by meaning is unavailable.

pub mod hits;
pub mod lexical;
pub mod sources;

#[cfg(test)]
mod tests;

pub use hits::{hits_for, HitTarget, SemanticHit};
pub use lexical::lexical_hits;
pub use sources::{index_sources, IndexKey};

use crate::domain::embedding::SourceText;
