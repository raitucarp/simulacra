//! What ticking one to-do does to the others of its post.
//!
//! - A parent is done only once every child is done or dropped (D7, the rule sub-task posts
//!   follow too); dropping it or moving it elsewhere does not wait.
//! - Reopening an item reopens the parents above it that were done, so a done parent never
//!   sits over an open child.
//! - Completing a repeating item (D8) brings its next occurrence round instead of closing it:
//!   it stays open, due on the rule's next day after today (or after its due day when that
//!   is later), counted from its first due day (or the day the post was written); its
//!   children start over open. A rule that cannot be read just closes the item.

use super::entries::TodoEntry;
use super::repeat::parse_repeat;
use super::state::TodoState;
use crate::domain::symbols::{unsettled_states, EntryState};
use jiff::civil::Date;

/// Why a tick was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TickRefusal {
    /// The item has this many children still to do.
    OpenChildren(usize),
}

/// The days a tick is read against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickDays {
    /// The day the post was written, which due phrases count from.
    pub written: Date,
    pub today: Date,
}

/// How many children of `entry` are not yet done or dropped.
pub fn open_children(entry: &TodoEntry, entries: &[TodoEntry]) -> usize {
    unsettled_states(entry.children.iter().filter_map(|child| entries.get(*child)).map(|child| child.state))
}

/// `ticks` after moving the `index`-th of `entries` to `to` (`entries` already reflect
/// `ticks`). Unknown indices leave the ticks as they are.
pub fn apply_tick(entries: &[TodoEntry], ticks: &TodoState, index: usize, to: EntryState, days: TickDays) -> Result<TodoState, TickRefusal> {
    let Some(entry) = entries.get(index) else { return Ok(ticks.clone()) };
    let open = open_children(entry, entries);
    if to == EntryState::Done && open > 0 {
        return Err(TickRefusal::OpenChildren(open));
    }
    let mut next = ticks.clone();
    if to == EntryState::Done && come_round(entry, entries, &mut next, days) {
        return Ok(next);
    }
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

/// Moves a repeating `entry` to its next occurrence in `ticks`; `false` when it does not repeat.
fn come_round(entry: &TodoEntry, entries: &[TodoEntry], ticks: &mut TodoState, days: TickDays) -> bool {
    let Some(rule) = entry.item.meta.repeat.as_deref().and_then(parse_repeat) else { return false };
    let anchor = entry.item.due_date(days.written).unwrap_or(days.written);
    let after = entry.due(days.written).map_or(days.today, |due| due.max(days.today));
    let Some(next_due) = rule.next_after(anchor, after) else { return false };
    let tick = ticks.tick_mut(entry.index, &entry.item);
    tick.state = Some(EntryState::Open);
    tick.due = Some(next_due);
    tick.completions += 1;
    for below in descendants(entry, entries) {
        ticks.tick_mut(below.index, &below.item).state = Some(EntryState::Open);
    }
    true
}

/// Every item nested under `entry`, at any depth.
fn descendants<'a>(entry: &TodoEntry, entries: &'a [TodoEntry]) -> Vec<&'a TodoEntry> {
    let mut out = Vec::new();
    for child in entry.children.iter().filter_map(|child| entries.get(*child)) {
        out.push(child);
        out.extend(descendants(child, entries));
    }
    out
}
