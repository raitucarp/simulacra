//! The edit log of a post (V7): one entry per numbered version, newest edit first, with when
//! it was saved, the bump the edit made and its note. Versions without a number (vault
//! documents) have no log.

use super::{Bump, SemVer, Version, VersionKind};
use jiff::Timestamp;

/// One line of the log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub version: SemVer,
    pub edited_at: Option<Timestamp>,
    /// `None` for the original.
    pub bump: Option<Bump>,
    pub note: String,
    /// The version the post shows.
    pub current: bool,
}

/// The log of `versions` (any order), newest edit first; ties put the higher number first.
pub fn edit_log(versions: &[Version]) -> Vec<LogEntry> {
    let mut log: Vec<LogEntry> = versions
        .iter()
        .filter_map(|v| v.number.map(|version| LogEntry { version, edited_at: v.edited_at, bump: v.bump, note: v.note.clone(), current: v.kind == VersionKind::Current }))
        .collect();
    log.sort_by(|a, b| b.edited_at.cmp(&a.edited_at).then_with(|| b.version.cmp(&a.version)));
    log
}
