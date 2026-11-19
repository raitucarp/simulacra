//! The Knowledge Vault: a library catalogue of the network's documents.
//!
//! - Browse by facets (subject tree, collections, classification) with full-text search,
//!   sorting, and a shelf of generated covers or a sortable table.
//! - An encyclopedic entry per document (abstract, record, filing, contents, citations,
//!   related entries, backlinks), a reader with saved progress, and a subject index.
//! - An add / edit form with progressive disclosure and validation.
//!
//! [`VaultPanel`] is the view entity; the shell feeds it [`VaultData`] and applies its
//! [`VaultEvent`]s to the store.

pub mod cover;
mod data;
mod editor;
mod panel;
mod render;
mod screen;
mod text;

pub use data::{BacklinkPreview, VaultData};
pub use panel::{EditorSubmission, VaultEvent, VaultPanel};
pub use render::{EMPTY_VAULT_ART, NO_MATCHES_ART, SUBJECT_INDEX_ART};
pub use text::problem_message;
