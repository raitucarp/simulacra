//! Category theory over the knowledge graph.
//!
//! Objects are the items of a network (thoughts, documents, concepts, people) and morphisms
//! are its typed relations (writes, replies to, quotes, reposts, links, cites, is about,
//! mentions). Paths compose: `x —f→ y —g→ z` gives the composite `g ∘ f : x → z`, and every
//! object has its identity (the empty path), so the paths form the free category on this
//! typed graph. [`schema`] collapses it to object kinds (the diagram that is drawn);
//! [`composition`] counts composable pairs per composite type, how often a composite is also
//! realised by a direct relation, and finds squares: two different two-step paths between the
//! same pair. A square *commutes in type* when both paths carry the same relations.

pub mod composition;
pub mod schema;

#[cfg(test)]
mod tests;

pub use composition::{compose, Composite, CompositionReport, Square};
pub use schema::{morphisms_of, schema_arrows, Morphism, MorphismKind, ObjectKind, SchemaArrow};
