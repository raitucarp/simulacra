//! The Journal page (sb E3): the user's private self-reflection, in five tabs.
//!
//! - Today: the day's prompt, a quick mood check-in, a focus start and capsules that opened.
//! - Journal: entries with prompts and templates, search, the heat strip and streaks.
//! - Mood: the affect-grid check-in, charts by day, weekday and hour, and associations.
//! - Focus: the timer with an intention, its log and statistics; focus mode hides the rest.
//! - Capsules: notes to the future self, sealed until their day.
//!
//! [`JournalPanel`] is the page entity; the shell owns the records ([`JournalData`]) and
//! applies its [`JournalEvent`]s. Nothing on this page leaves it (see `domain::journal::privacy`).

pub mod data;
pub mod events;
mod panel;
mod render;
pub mod tab;
pub mod text;

#[cfg(test)]
mod tests;

pub use data::{JournalData, WikilinkPreviews};
pub use events::JournalEvent;
pub use panel::JournalPanel;
pub use tab::JournalTab;
