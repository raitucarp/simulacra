//! What the Journal page asks the shell to do (the shell owns and stores the records).

use crate::domain::journal::focus::FocusLink;
use crate::domain::journal::{FocusSession, JournalEntry, MoodCheckin, RecordId, TimeCapsule};

/// A request from the Journal page.
#[derive(Debug, Clone, PartialEq)]
pub enum JournalEvent {
    SaveEntry(JournalEntry),
    DeleteEntry(RecordId),
    SaveCheckin(MoodCheckin),
    DeleteCheckin(RecordId),
    SaveSession(FocusSession),
    SaveCapsule(TimeCapsule),
    DeleteCapsule(RecordId),
    /// Open the composer with this text; the user picks the persona as usual.
    TurnIntoPost(String),
    /// Open quick capture: a distraction goes to the Inbox, the session keeps running.
    CaptureDistraction,
    /// Focus mode switched: the shell hides or shows the feed and side panels.
    FocusMode(bool),
    /// Open a `[[target]]` named in an entry.
    OpenWikilink(String),
    /// Open what a focus session works on.
    OpenLink(FocusLink),
}
