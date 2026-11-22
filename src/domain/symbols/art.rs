//! The in-house art of the core symbols (S3): one template per entry state (a task's dot,
//! cross and arrows, an event's ring), one for notes and one per signifier, under
//! `illustrations/symbols/` (see `assets/illustrations/STYLE.md`).

use super::kinds::{EntryState, EntrySymbol, Signifier};
use super::transitions::allowed_states;

/// Folder of the symbol templates, below the asset root.
pub const SYMBOL_ART_DIR: &str = "illustrations/symbols";

/// One core symbol drawing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolArt {
    /// An entry in a state (`None` only for notes, which have no state).
    Entry(EntrySymbol, Option<EntryState>),
    Signifier(Signifier),
}

impl SymbolArt {
    /// Every drawing: each entry in each of its allowed states, the note, every signifier.
    pub fn all() -> Vec<Self> {
        let entries = EntrySymbol::ALL.into_iter().flat_map(|entry| {
            let states = allowed_states(entry);
            let none = states.is_empty().then_some(Self::Entry(entry, None));
            states.iter().map(move |state| Self::Entry(entry, Some(*state))).chain(none)
        });
        entries.chain(Signifier::ALL.map(Self::Signifier)).collect()
    }

    /// The template's file stem (`task_open`, `note`, `priority`).
    pub fn key(self) -> String {
        match self {
            Self::Entry(entry, Some(state)) => format!("{}_{}", entry_key(entry), state_key(state)),
            Self::Entry(entry, None) => entry_key(entry).to_string(),
            Self::Signifier(signifier) => signifier_key(signifier).to_string(),
        }
    }

    /// The template's asset path (`illustrations/symbols/<key>.svg`).
    pub fn path(self) -> String {
        format!("{SYMBOL_ART_DIR}/{}.svg", self.key())
    }
}

/// Stable key of an entry kind (also its stored name).
pub fn entry_key(entry: EntrySymbol) -> &'static str {
    match entry {
        EntrySymbol::Task => "task",
        EntrySymbol::Event => "event",
        EntrySymbol::Note => "note",
    }
}

/// Stable key of a state (also its stored name).
pub fn state_key(state: EntryState) -> &'static str {
    match state {
        EntryState::Open => "open",
        EntryState::Done => "done",
        EntryState::Migrated => "migrated",
        EntryState::Scheduled => "scheduled",
        EntryState::Dropped => "dropped",
        EntryState::Delegated => "delegated",
    }
}

/// Stable key of a signifier (also its stored name).
pub fn signifier_key(signifier: Signifier) -> &'static str {
    match signifier {
        Signifier::Priority => "priority",
        Signifier::Inspiration => "inspiration",
        Signifier::Explore => "explore",
    }
}
