//! The Review page (sb C1, anchored on posts in sb C1b): spaced-repetition sessions on posts
//! and vault passages, connection recall, quizzes and statistics, plus the learn sheet that
//! adds a question or an answer to a post or a passage.
//!
//! - [`ReviewPanel`]: the page entity; the shell feeds it [`ReviewData`] and applies its
//!   [`ReviewEvent`]s (answers, quiz requests, item and map changes) to the deck.
//! - [`LearnSheet`]: the learn sheet; it emits [`LearnEvent`]s (save, AI suggestion).
//! - `keys`: reveal and grade actions, bound in the central keymap.
//! - `suggest_request`: the AI suggestion through the configured LLM client.

pub mod data;
pub mod events;
pub mod keys;
pub mod learn;
pub mod learning;
mod panel;
mod render;
pub mod suggest_request;
pub mod status;
pub mod text;

#[cfg(test)]
mod tests;

pub use data::{PostView, PostViews, ReviewData};
pub use events::{PickerMode, QuizContent, ReviewEvent};
pub use keys::REVIEW_KEY_CONTEXT;
pub use learn::{LearnEvent, LearnRequest, LearnSheet, LearnSource, Passage};
pub use panel::{ReviewPanel, Screen};
pub use render::{REVIEW_DONE_ART, REVIEW_EMPTY_ART};
