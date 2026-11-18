//! Argument maps (sb E1): the structure of a post or thread as an argument graph.
//!
//! - [`model`]: nodes in Toulmin's roles (claim, ground, warrant, backing, qualifier,
//!   rebuttal) joined by support and attack edges, each node citing its post.
//! - [`edit`]: manual edits as pure, validated steps (add, remove, relabel, re-role, edges,
//!   reorder).
//! - [`extract`]: the AI extraction prompt and the lenient, validating reader of its reply.
//! - [`analysis`]: unsupported claims, circular support, unanswered objections and the
//!   grounded labelling (which statements stand given the attacks).
//! - [`layout`]: the tiered canvas positions.

pub mod analysis;
pub mod edit;
pub mod extract;
pub mod layout;
pub mod model;

#[cfg(test)]
mod tests;

pub use analysis::{analyse, ArgumentReport, Standing};
pub use edit::{apply_edit, EditError, MapEdit, MAX_NODE_CHARS};
pub use extract::{extraction_prompt, parse_extraction, Extraction, ThreadPost, EXTRACT_ANSWER_TOKENS};
pub use layout::{layout, MapLayout};
pub use model::{ArgEdge, ArgNode, ArgumentMap, EdgeKind, NodeRole};
