//! A task and its sub-tasks (S8): the replies to a task that are tasks or events themselves.
//! A parent task is done only once every child entry is settled (done or dropped); a child
//! that is still open, migrated, scheduled or delegated holds it back.

use super::kinds::{EntryState, EntrySymbol};
use super::PostSymbols;

/// How many of `children` still hold their parent back: tasks and events not yet settled.
pub fn unsettled_children<'a>(children: impl IntoIterator<Item = &'a PostSymbols>) -> usize {
    children.into_iter().filter(|child| is_unsettled(child)).count()
}

/// Whether `symbols` is a task or event that is not done or dropped.
pub fn is_unsettled(symbols: &PostSymbols) -> bool {
    matches!(symbols.entry, Some(EntrySymbol::Task | EntrySymbol::Event)) && !symbols.state.is_some_and(EntryState::is_closed)
}

/// Whether a parent with `children` may move to `to`: only `Done` waits for the children.
pub fn may_settle<'a>(to: EntryState, children: impl IntoIterator<Item = &'a PostSymbols>) -> bool {
    to != EntryState::Done || unsettled_children(children) == 0
}
