//! Where journal, mood, focus and capsule data may appear. Everything is private by default:
//! never in the stream, the semantic index, Insights or an agent's context, and never sent to
//! an AI unless the user clicks an AI action on that one entry. The only opt-in is word search
//! of journal entries in the command palette (Settings › Privacy & Data, off by default).

use super::entry::{newest_first, JournalEntry};

/// A place outside the Journal page that could show private data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    /// The stream, threads and the Inbox.
    Feed,
    /// The embedding index and search by meaning.
    SemanticSearch,
    /// The Insights page.
    Insights,
    /// What agents read before they write.
    AgentContext,
    /// Word search in the command palette.
    PaletteSearch,
    /// An AI action the user clicked on one entry.
    ExplicitAiAction,
}

impl Surface {
    pub const ALL: [Self; 6] = [Self::Feed, Self::SemanticSearch, Self::Insights, Self::AgentContext, Self::PaletteSearch, Self::ExplicitAiAction];
}

/// The user's privacy choices that concern the journal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct JournalPrivacy {
    /// Settings › Privacy & Data › "Include journal in search".
    pub include_in_search: bool,
}

/// Whether private records may reach `surface`; `explicit` is a click on that very entry.
pub fn may_reach(surface: Surface, privacy: JournalPrivacy, explicit: bool) -> bool {
    match surface {
        Surface::Feed | Surface::SemanticSearch | Surface::Insights | Surface::AgentContext => false,
        Surface::PaletteSearch => privacy.include_in_search,
        Surface::ExplicitAiAction => explicit,
    }
}

/// The entries the palette may search, newest first (none unless opted in).
pub fn palette_entries(entries: &[JournalEntry], privacy: JournalPrivacy) -> Vec<&JournalEntry> {
    if may_reach(Surface::PaletteSearch, privacy, false) { newest_first(entries) } else { Vec::new() }
}
