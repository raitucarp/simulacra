//! The "Entry" submenu of the user's own task or event (S7): one item per state the entry
//! may move to (done, migrate, schedule, drop, delegate, or reopen), each a patch version of
//! the post.

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use crate::domain::symbols::{can_transition, state_key, EntryState};
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuSpec, MenuText, Submenu};

/// The menu icon of the action moving an entry to `state`.
const fn state_icon(state: EntryState) -> LucideIcon {
    match state {
        EntryState::Open => LucideIcon::RotateCcw,
        EntryState::Done => LucideIcon::Check,
        EntryState::Migrated => LucideIcon::ChevronRight,
        EntryState::Scheduled => LucideIcon::ChevronLeft,
        EntryState::Dropped => LucideIcon::X,
        EntryState::Delegated => LucideIcon::Send,
    }
}

/// The states the post's entry may move to now, in their usual order.
pub fn next_states(controls: &DataControls) -> Vec<EntryState> {
    let (Some(entry), Some(from)) = (controls.symbols.entry, controls.symbols.state) else { return Vec::new() };
    EntryState::ALL.into_iter().filter(|to| can_transition(entry, from, *to)).collect()
}

/// The "Entry" submenu of the moves the post's entry allows (`None` without any).
pub(super) fn entry_submenu(controls: &DataControls) -> Option<Submenu> {
    let states = next_states(controls);
    if !controls.own || states.is_empty() {
        return None;
    }
    let moves = states.into_iter().map(|to| {
        let label = MenuText::key(format!("symbols.action.{}", state_key(to)));
        item(format!("{}{}", ids::ENTRY_PREFIX, state_key(to)), label, state_icon(to), DataEvent::SetEntryState(controls.post_id.clone(), to), &controls.on_event)
    });
    Some(Submenu::new(ids::ENTRY, MenuText::key("symbols.menu.entry"), MenuSpec::of(moves)).icon(LucideIcon::ListChecks))
}
