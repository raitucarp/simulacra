//! Keyboard handling of a [`CardGrid`](super::CardGrid): arrows, Home and End move the cursor
//! (previewing the cell it lands on), Enter / Space pick the cell under it.

use super::card_grid::GridFocus;
use super::focus_ring::is_activation;
use super::ActionCallback;
use crate::ui::features::forge::logic::{GridCursor, NavKey};
use gpui_kit::*;

/// What the key handler needs of each cell.
pub(super) struct CellAction {
    pub(super) pick: ActionCallback,
    pub(super) hover: Option<ActionCallback>,
    pub(super) disabled: bool,
}

/// Moves the cursor (previewing the cell it lands on) or picks the cell under it.
pub(super) fn on_key(focus: &Entity<GridFocus>, event: &KeyDownEvent, columns: usize, cells: &[CellAction], window: &mut Window, cx: &mut App) {
    let cursor = GridCursor::new(focus.read(cx).cursor, cells.len(), columns);
    let nav = NavKey::from_key(event.keystroke.key.as_str()).filter(|_| !event.keystroke.modifiers.modified());
    if let Some(nav) = nav {
        let moved = cursor.moved(nav).index;
        focus.update(cx, |state, cx| {
            state.cursor = moved;
            cx.notify();
        });
        if let Some(hover) = cells.get(moved).and_then(|cell| cell.hover.clone()) {
            hover(window, cx);
        }
        cx.stop_propagation();
    } else if is_activation(event) {
        if let Some(cell) = cells.get(cursor.index).filter(|cell| !cell.disabled) {
            (cell.pick.clone())(window, cx);
        }
        cx.stop_propagation();
    }
}
