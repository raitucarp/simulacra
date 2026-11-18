//! Insights: pure analytics behind the Insights page (sb D2). Everything here is a pure
//! function of owned data, so the shell computes it on the background executor.
//!
//! - [`graph`]: graph theory (degree, influence, betweenness, clustering, components,
//!   communities, bridges and articulation points, paths) and a layout that scales.
//! - [`sets`]: set operations, Jaccard similarity, Venn regions and UpSet rows.
//! - [`symmetry`]: group theory (thread automorphisms, orbits, turn-taking permutations).
//! - [`category`]: typed relations as morphisms, composition and squares.
//! - [`embedding`]: the 2D embedding map with k-means clusters and neighbours in meaning.
//! - [`activity`]: posts over time, per author and kind, the weekly rhythm, distributions.
//! - [`facts`], [`filter`], [`network`], [`report`]: what is counted and the full report.

pub mod activity;
pub mod category;
pub mod embedding;
pub mod facts;
pub mod filter;
pub mod graph;
pub mod lenses; // sb F3: concept timeline, viewpoint diversity, readability
pub mod network;
pub mod report;
pub mod sets;
pub mod symmetry;

pub use facts::{AuthorFacts, PostKind};
pub use filter::{Audience, DateRange, InsightsFilter};
pub use network::{GraphScope, NetworkReport};
pub use report::{InsightsInput, InsightsReport, PostSummary};
