//! Thinking tools (sb E1), pure side: four ways to look harder at one's own thoughts.
//!
//! - [`argument`]: a post or thread as an argument graph (Toulmin roles in a bipolar
//!   argumentation framework), its edits, its AI extraction and its analysis (unsupported
//!   claims, circular support, unanswered objections, the grounded extension).
//! - [`credence`]: a stated probability on a post's claim, its resolution, and calibration
//!   scoring (Brier, log score, calibration curve with Wilson intervals, Murphy decomposition).
//! - [`tensions`]: pairs of posts or vault passages that may contradict each other: lexical
//!   signals, candidate ranking, and the AI classification of a candidate pair.
//! - [`atomic`]: an atomic-note linter (one claim per note, with context) and a proposal that
//!   splits a note into one post per claim.
//!
//! The effectful side (AI requests through the post actions runner, SQLite, UI) lives in
//! `agent::post_actions`, `storage::db::thinking*` and `ui::features::thinking`.

pub mod argument;
pub mod atomic;
pub mod credence;
pub mod tensions;
