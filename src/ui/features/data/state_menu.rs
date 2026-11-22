//! The entry items of the user's own task or event in its "…" menu (S7): one item per state
//! the entry may move to (done, migrate, schedule, drop, delegate, or reopen), each a patch
//! version of the post.

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::text::tr;
use crate::domain::symbols::{can_transition, state_key, EntryState};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::component::Icon;
use gpui_kit::*;

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

/// `menu` with an "Entry" submenu of the moves the post's entry allows (unchanged without).
pub(super) fn with_state_items(menu: PopupMenu, controls: &DataControls, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let states = next_states(controls);
    if states.is_empty() {
        return menu;
    }
    let controls = controls.clone();
    let icon = Some(Icon::new(LucideIcon::ListChecks));
    menu.submenu_with_icon(icon, tr("symbols.menu.entry"), window, cx, move |submenu, _, _| {
        states.iter().fold(submenu, |submenu, to| {
            let label = tr(&format!("symbols.action.{}", state_key(*to)));
            submenu.item(item(label, state_icon(*to), DataEvent::SetEntryState(controls.post_id.clone(), *to), &controls.on_event))
        })
    })
}
