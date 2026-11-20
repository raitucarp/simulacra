//! The value types a post's symbol set is made of. All serialize as `snake_case` strings so
//! the stored JSON stays readable and stable.

use serde::{Deserialize, Serialize};

/// What kind of entry a post is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntrySymbol {
    /// Something to do; carries any [`EntryState`].
    Task,
    /// Something that happens at a time; open, done, scheduled or dropped.
    Event,
    /// A thought to keep; it has no state.
    Note,
}

impl EntrySymbol {
    pub const ALL: [Self; 3] = [Self::Task, Self::Event, Self::Note];
}

/// Where a task or event stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryState {
    /// Not yet handled.
    Open,
    /// Finished.
    Done,
    /// Carried over to another list or period.
    Migrated,
    /// Moved to a later date.
    Scheduled,
    /// No longer relevant.
    Dropped,
    /// Handed to someone else.
    Delegated,
}

impl EntryState {
    pub const ALL: [Self; 6] =
        [Self::Open, Self::Done, Self::Migrated, Self::Scheduled, Self::Dropped, Self::Delegated];

    /// Whether the entry is settled for good (done or dropped) rather than still pending
    /// somewhere (open, migrated, scheduled, delegated).
    pub fn is_closed(self) -> bool {
        matches!(self, Self::Done | Self::Dropped)
    }
}

/// A mark that draws attention to an entry, independent of its kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Signifier {
    Priority,
    Inspiration,
    Explore,
}

impl Signifier {
    pub const ALL: [Self; 3] = [Self::Priority, Self::Inspiration, Self::Explore];
}

/// A custom symbol from the icon library: the icon set and the icon's name inside it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SymbolRef {
    pub set: String,
    pub name: String,
}

impl SymbolRef {
    pub fn new(set: impl Into<String>, name: impl Into<String>) -> Self {
        Self { set: set.into(), name: name.into() }
    }
}
