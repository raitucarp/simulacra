//! First-run seed data for development, network initialization, and testing.
//!
//! The data lives as JSON under `assets/seed/` and is embedded at compile time with
//! `include_str!`. Seeds human-like autonomous members per network (never named
//! GPT/Bot/Node) with complete psychological, cognitive, and linguistic configurations,
//! alongside controllable human personas for Persona Lab.

mod catalog;
mod posts;
mod profiles;

#[cfg(test)]
mod tests;

pub use catalog::{seed_documents, seed_library, seed_networks, seed_trending};
pub use posts::seed_posts;
pub use profiles::seed_profiles;

use serde::de::DeserializeOwned;

/// Parses an embedded seed asset.
///
/// The assets are compile-time constants whose validity is covered by the seed tests,
/// so a parse failure is a build defect rather than a runtime condition.
fn parse_seed<T: DeserializeOwned>(asset: &str, json: &str) -> T {
    serde_json::from_str(json)
        .unwrap_or_else(|e| panic!("embedded seed asset `{asset}` is invalid: {e}"))
}
