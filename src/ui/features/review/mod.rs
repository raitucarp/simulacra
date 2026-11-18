//! The Review page (sb C1): spaced-repetition sessions, quizzes and statistics, plus the card
//! maker sheet that turns posts, vault documents and sentences into flashcards.
//!
//! - [`ReviewPanel`]: the page entity; the shell feeds it [`ReviewData`] and applies its
//!   [`ReviewEvent`]s (answers, quiz requests, card changes) to the deck.
//! - [`FlashcardMaker`]: the card maker; it emits [`MakerEvent`]s (save, AI draft).
//! - `keys`: reveal and grade actions, bound in the central keymap.
//! - `draft_request`: the AI draft through the configured LLM client.

pub mod data;
pub mod draft_request;
pub mod events;
pub mod keys;
pub mod maker;
mod panel;
mod render;
pub mod text;

#[cfg(test)]
mod tests;

pub use data::ReviewData;
pub use events::{QuizContent, ReviewEvent};
pub use keys::REVIEW_KEY_CONTEXT;
pub use maker::{FlashcardMaker, MakerEvent, MakerRequest, MakerSentence};
pub use panel::{ReviewPanel, Screen};
pub use render::{REVIEW_DONE_ART, REVIEW_EMPTY_ART};
