//! Seed data factory for development, network initialization, and testing.
//!
//! Kept for path compatibility: the seed data now lives as embedded JSON under
//! `assets/seed/` and is loaded by [`crate::state::seed`].

pub use super::seed::{seed_documents, seed_networks, seed_posts, seed_profiles, seed_trending};
