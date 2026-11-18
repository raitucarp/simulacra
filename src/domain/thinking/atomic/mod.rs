//! The atomic-note check (sb E1): a pure linter for a draft post, a post or a vault note.
//!
//! An atomic note says one thing, in its own words, with enough context to be read alone
//! and with links that place it among other notes. The linter looks for what breaks that:
//!
//! - [`clauses`] / [`claims`]: the text as pieces (list items, sentences of the
//!   locale-aware splitter, clauses joined by `;` or a coordinating conjunction between two
//!   independent clauses); questions and very short fragments are not claims.
//! - [`topics`]: concepts of the note grouped by the claims they share; groups that never
//!   meet in one claim are separate topics.
//! - [`context`]: missing context: no link, a factual-sounding claim without a source, an
//!   opening pronoun that points at nothing.
//! - [`findings`]: each [`Finding`] with its locale key and its penalty.
//! - [`report`]: [`check_atomicity`], the 0–100 score (100 minus the penalties) and the
//!   findings that explain it.
//! - [`split`]: [`split_proposal`], the note as one post per claim, for the user to edit.

pub mod claims;
pub mod clauses;
pub mod context;
pub mod findings;
pub mod report;
pub mod split;
pub mod topics;

#[cfg(test)]
mod tests;

pub use claims::claims_of;
pub use findings::Finding;
pub use report::{check_atomicity, AtomicReport, ATOMIC_SCORE, LONG_CHARS, SHORT_CHARS};
pub use split::{split_proposal, MAX_SPLIT};
