//! Delegated to-dos (D12): an item with `@name` that is still open (or marked delegated with
//! `[/]`) is handed to that agent, which replies to the post with how it is going. Each item
//! is asked about once; the tick remembers it, so neither a restart nor an edit asks again.

use super::entries::TodoEntry;
use crate::domain::symbols::EntryState;

/// One item handed to `delegate`, waiting for its progress reply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delegation {
    /// The item's place among the post's to-dos.
    pub index: usize,
    pub text: String,
    /// The name after `@`.
    pub delegate: String,
}

/// The items of `entries` handed to someone whose progress reply was not asked for yet.
pub fn awaiting_progress(entries: &[TodoEntry]) -> Vec<Delegation> {
    entries
        .iter()
        .filter(|entry| matches!(entry.state, EntryState::Open | EntryState::Delegated) && !entry.progress_asked)
        .filter_map(|entry| Some(Delegation { index: entry.index, text: entry.item.text.clone(), delegate: entry.item.meta.delegate.clone()? }))
        .collect()
}
