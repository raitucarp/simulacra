//! What ticking one to-do does to the others of its post.
//!
//! - A parent is done only once every child is done or dropped (D7, the rule sub-task posts
//!   follow too); dropping it or moving it elsewhere does not wait.
//! - Reopening an item reopens the parents above it that were done, so a done parent never
//!   sits over an open child.

use super::entries::TodoEntry;
use super::state::TodoState;
use crate::domain::symbols::{unsettled_states, EntryState};

/// Why a tick was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickRefusal {
    /// The item has this many children still to do.
    OpenChildren(usize),
}

/// How many children of `entry` are not yet done or dropped.
pub fn open_children(entry: &TodoEntry, entries: &[TodoEntry]) -> usize {
    unsettled_states(entry.children.iter().filter_map(|child| entries.get(*child)).map(|child| child.state))
}

/// `ticks` after moving the `index`-th of `entries` to `to` (`entries` already reflect
/// `ticks`). Unknown indices leave the ticks as they are.
pub fn apply_tick(entries: &[TodoEntry], ticks: &TodoState, index: usize, to: EntryState) -> Result<TodoState, TickRefusal> {
    let Some(entry) = entries.get(index) else { return Ok(ticks.clone()) };
    let open = open_children(entry, entries);
    if to == EntryState::Done && open > 0 {
        return Err(TickRefusal::OpenChildren(open));
    }
    let mut next = ticks.clone();
    next.tick_mut(index, &entry.item).state = Some(to);
    if !to.is_closed() {
        let mut parent = entry.parent;
        while let Some(above) = parent.and_then(|at| entries.get(at)) {
            if above.state == EntryState::Done {
                next.tick_mut(above.index, &above.item).state = Some(EntryState::Open);
            }
            parent = above.parent;
        }
    }
    Ok(next)
}
