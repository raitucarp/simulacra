//! The user's private self-reflection (sb E3): a daily journal with rotating prompts and
//! templates, a mood and energy log on the affect grid, focus sessions, and time capsules.
//!
//! Everything here is pure: records are immutable values replaced by id (`records`), and the
//! shell keeps them in a [`JournalBook`] loaded once from the vault. Nothing in this module is
//! shown outside the Journal page; `privacy` states the rule and its one opt-in.

pub mod activity;
pub mod calendar;
pub mod capsule;
pub mod entry;
pub mod focus;
pub mod ids;
pub mod mood;
pub mod privacy;
pub mod prompts;
pub mod records;
pub mod search;
pub mod streak;
pub mod templates;

#[cfg(test)]
mod tests;

pub use capsule::TimeCapsule;
pub use entry::JournalEntry;
pub use focus::FocusSession;
pub use ids::RecordId;
pub use mood::MoodCheckin;

use chrono::{DateTime, NaiveDate, TimeZone};

/// The journal day `at` falls in: local time with the review page's night rollover, so a
/// late-night entry still belongs to the evening it was written in.
pub fn journal_day<Tz: TimeZone>(at: &DateTime<Tz>) -> NaiveDate {
    crate::domain::review::day::review_day(at)
}

/// Every private record of the user.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct JournalBook {
    pub entries: Vec<JournalEntry>,
    pub checkins: Vec<MoodCheckin>,
    pub sessions: Vec<FocusSession>,
    pub capsules: Vec<TimeCapsule>,
}

impl JournalBook {
    /// Days with at least one written entry.
    pub fn written_days(&self) -> impl Iterator<Item = NaiveDate> + '_ {
        self.entries.iter().filter(|e| !e.is_blank()).map(|e| e.day)
    }

    /// Whether nothing at all has been recorded.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.checkins.is_empty() && self.sessions.is_empty() && self.capsules.is_empty()
    }
}
