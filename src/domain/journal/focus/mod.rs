//! Focus sessions: an intention ("what will I finish?"), a planned length from a preset or
//! custom, an optional link to a post, a vault document, a review session or a goal, the
//! timer (`timer`), distractions captured to the Inbox, and a one-line reflection at the end.

pub mod stats;
pub mod timer;

pub use timer::{FocusTimer, Phase};

use super::ids::RecordId;
use super::records::Record;
use jiff::Timestamp;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

/// Preset lengths in minutes: a pomodoro, a deep block, an ultradian cycle.
pub const PRESET_MINUTES: [u32; 3] = [25, 50, 90];
/// Longest custom session.
pub const MAX_MINUTES: u32 = 240;

/// What a session works on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum FocusLink {
    Post(String),
    Document(String),
    /// Today's review session.
    Review,
    /// A goal, by its name.
    Goal(String),
}

/// How the session went, in the user's own judgement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Done,
    Partly,
    NotDone,
}

impl Outcome {
    pub const ALL: [Self; 3] = [Self::Done, Self::Partly, Self::NotDone];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Done => "done",
            Self::Partly => "partly",
            Self::NotDone => "not_done",
        }
    }
}

/// A finished session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FocusSession {
    pub id: RecordId,
    pub day: Date,
    pub started_at: Timestamp,
    pub ended_at: Timestamp,
    pub intention: String,
    pub planned_minutes: u32,
    pub focused_minutes: u32,
    #[serde(default)]
    pub link: Option<FocusLink>,
    #[serde(default)]
    pub distractions: u32,
    #[serde(default)]
    pub outcome: Option<Outcome>,
    #[serde(default)]
    pub reflection: String,
}

impl Record for FocusSession {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

/// A custom length typed by the user, within 1..=[`MAX_MINUTES`].
pub fn custom_minutes(text: &str) -> Option<u32> {
    text.trim().parse::<u32>().ok().filter(|m| (1..=MAX_MINUTES).contains(m))
}
