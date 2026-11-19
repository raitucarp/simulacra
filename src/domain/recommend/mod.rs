//! Recommendations from post embeddings. Posts are the source of truth: what a profile wrote,
//! answered, quoted, reposted or liked says what it cares about, and every other post is
//! ranked by how close its meaning is to that, blended with how fresh it is.
//!
//! - [`interest`]: a profile's interest vector from its [`Signal`]s.
//! - [`rank`]: candidates retrieved from the vector index (USearch) and scored with a [`Blend`].
//!
//! Everything here is pure; `crate::state::recommend` reads the signals and candidates out of
//! the store.

pub mod interest;
pub mod rank;
#[cfg(test)]
mod tests;

pub use interest::{interest_vector, Signal};
pub use rank::{freshness, rank_all, recommend, Blend, Candidate, Recommendation};
