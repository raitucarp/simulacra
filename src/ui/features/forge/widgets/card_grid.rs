//! [`CardGrid`]: selectable cells (option cards, swatches…) in rows of a fixed (or width-fitted)
//! number of columns, reachable as one tab stop. Arrow keys / Home / End move a cursor kept in keyed
//! element state, Enter / Space pick the cell under it, and the cursor cell shows the focus
//! ring while the grid has keyboard focus. Clicking picks and moves the cursor there.

use super::grid_cell::GridCell;
use super::grid_keys::{on_key, CellAction};
use super::ActionCallback;
use crate::ui::features::forge::logic::fit_columns;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

/// Focus handle, cursor and last measured width of one grid.
pub(super) struct GridFocus {
    handle: FocusHandle,
    pub(super) cursor: usize,
    width: Option<Pixels>,
}

/// A keyboard-navigable grid of [`GridCell`]s.
#[derive(IntoElement)]
pub struct CardGrid {
    id: ElementId,
    cells: Vec<GridCell>,
    columns: usize,
    gap: Pixels,
    headers: Vec<AnyElement>,
    centered: bool,
    /// Minimum cell width when the column count follows the grid's width.
    min_cell: Option<Pixels>,
}

impl CardGrid {
    /// `cells` in rows of `columns` (also the step of Up / Down), `gap` apart.
    pub fn new(id: impl Into<ElementId>, cells: Vec<GridCell>, columns: usize, gap: Pixels) -> Self {
        Self { id: id.into(), cells, columns: columns.max(1), gap, headers: Vec::new(), centered: false, min_cell: None }
    }

    /// One header per row, shown above it (rows past the list get none).
    pub fn row_headers(mut self, headers: Vec<AnyElement>) -> Self {
        self.headers = headers;
        self
    }

    /// Fits as many `min_cell`-wide columns as the measured width allows (Up / Down follow).
    pub fn auto_columns(mut self, min_cell: Pixels) -> Self {
        self.min_cell = Some(min_cell);
        self
    }

    /// Centers each row.
    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }
}

/// An invisible layer over the grid that records its width at prepaint (re-rendering on change).
fn width_probe(focus: Entity<GridFocus>) -> impl IntoElement {
    let record = move |bounds: Bounds<Pixels>, _: &mut Window, cx: &mut App| {
        if focus.read(cx).width != Some(bounds.size.width) {
            focus.update(cx, |state, cx| {
                state.width = Some(bounds.size.width);
                cx.notify();
            });
        }
    };
    canvas(record, |_, _, _, _| {}).absolute().top_0().left_0().size_full()
}

/// The cell at `at`, with its click moving the cursor there before picking. In a width-fitted
/// grid (`fill`) an option card widens to its column, so a row has no empty band on its right.
fn render_cell(cell: GridCell, at: usize, focused: bool, fill: bool, focus: &Entity<GridFocus>) -> Stateful<Div> {
    let (pick, cursor) = (cell.pick, focus.clone());
    let click: ActionCallback = Rc::new(move |window, cx| {
        cursor.update(cx, |state, _| state.cursor = at);
        pick(window, cx);
    });
    let element = (cell.build)(focused, click).when(fill && cell.stretches, |card| card.w_full());
    match cell.hover {
        Some(hover) => element.on_hover(move |hovered, window, cx| {
            if *hovered {
                hover(window, cx);
            }
        }),
        None => element,
    }
}

impl RenderOnce for CardGrid {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let initial = self.cells.iter().position(|cell| cell.selected).unwrap_or_default();
        let focus = window.use_keyed_state(self.id.clone(), cx, |_, cx| GridFocus { handle: cx.focus_handle().tab_stop(true), cursor: initial, width: None });
        let handle = focus.read(cx).handle.clone();
        let focused_at = (handle.is_focused(window) && window.last_input_was_keyboard()).then(|| focus.read(cx).cursor);
        let actions: Rc<[CellAction]> =
            self.cells.iter().map(|cell| CellAction { pick: cell.pick.clone(), hover: cell.hover.clone(), disabled: cell.disabled }).collect();
        let (gap, centered) = (self.gap, self.centered);
        let columns = match (self.min_cell, focus.read(cx).width) {
            (Some(min), Some(width)) => fit_columns(width.as_f32(), min.as_f32(), gap.as_f32()),
            _ => self.columns,
        };
        let fill = self.min_cell.is_some();
        let mut cells = self.cells.into_iter().enumerate().map(|(at, cell)| render_cell(cell, at, focused_at == Some(at), fill, &focus)).peekable();
        let (mut headers, mut rows) = (self.headers.into_iter(), Vec::new());
        while cells.peek().is_some() {
            let taken: Vec<Stateful<Div>> = cells.by_ref().take(columns).collect();
            // Equal slots (empty ones finish the last row) so the columns line up row to row.
            let empty = if fill { columns.saturating_sub(taken.len()) } else { 0 };
            let taken = taken.into_iter().map(|cell| if fill { div().flex_1().min_w_0().flex().child(cell).into_any_element() } else { cell.into_any_element() });
            let row = div().flex().gap(gap).when(centered, |row| row.justify_center()).children(taken).children((0..empty).map(|_| div().flex_1()));
            rows.push(div().flex().flex_col().gap(gap).children(headers.next()).child(row));
        }
        let key_focus = focus.clone();
        div()
            .id(self.id)
            .track_focus(&handle)
            .flex()
            .flex_col()
            .gap(gap)
            .on_key_down(move |event: &KeyDownEvent, window, cx| on_key(&key_focus, event, columns, &actions, window, cx))
            .children(rows)
            .when(self.min_cell.is_some(), |grid| grid.relative().child(width_probe(focus)))
    }
}
