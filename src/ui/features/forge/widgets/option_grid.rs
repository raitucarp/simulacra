//! [`IllustratedOptionGrid`]: every option of a `TraitOption` as an illustrated card, laid
//! out in rows of [`GRID_COLUMNS`]. Arrow keys move a focus cursor (kept in keyed element
//! state), Enter / Space picks; clicking picks and focuses the grid.

use super::option_card::{option_card, OptionCardSpec};
use super::ValueCallback;
use crate::domain::TraitOption;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::{GridCursor, NavKey, ACTIVATE_KEYS};
use crate::ui::features::forge::text::{trait_description, trait_label};
use crate::ui::illustration::Illustration;
use gpui_kit::*;
use std::rc::Rc;

/// Default number of cards per row.
pub const GRID_COLUMNS: usize = 3;

/// Focus handle and cursor of one grid.
struct GridFocus {
    handle: FocusHandle,
    cursor: usize,
}

/// Illustrated cards for every option of `T`.
#[derive(IntoElement)]
pub struct IllustratedOptionGrid<T: TraitOption> {
    id: ElementId,
    selected: Option<T>,
    columns: usize,
    card_width: Pixels,
    descriptions: bool,
    theme: ThemeTokens,
    on_select: ValueCallback<T>,
}

impl<T: TraitOption> IllustratedOptionGrid<T> {
    /// A grid of `T::ALL` with `selected` ringed; picking a card calls `on_select`.
    pub fn new(
        id: impl Into<ElementId>,
        selected: Option<T>,
        theme: &ThemeTokens,
        on_select: impl Fn(T, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            selected,
            columns: GRID_COLUMNS,
            card_width: theme.forge_option_card_width,
            descriptions: true,
            theme: theme.clone(),
            on_select: Rc::new(on_select),
        }
    }

    /// Cards per row (also the step of Up / Down).
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    /// Width of each card.
    pub fn card_width(mut self, width: Pixels) -> Self {
        self.card_width = width;
        self
    }

    /// Hides the short descriptions under the labels.
    pub fn compact(mut self) -> Self {
        self.descriptions = false;
        self
    }

    fn selected_index(&self) -> usize {
        let key = self.selected.map(|option| option.key());
        T::ALL.iter().position(|option| Some(option.key()) == key).unwrap_or_default()
    }

    fn card(&self, at: usize, option: T, focused: bool, focus: &Entity<GridFocus>) -> Stateful<Div> {
        let art = Illustration::trait_card(option).width(self.card_width);
        let mut spec = OptionCardSpec::new((self.id.clone(), at.to_string()), art, trait_label(option), self.card_width);
        spec.description = self.descriptions.then(|| trait_description(option)).flatten();
        spec.selected = self.selected.is_some_and(|chosen| chosen.key() == option.key());
        spec.focused = focused;
        let (on_select, focus) = (self.on_select.clone(), focus.clone());
        option_card(spec, &self.theme, move |window, cx| {
            focus.update(cx, |state, _| state.cursor = at);
            let handle = focus.read(cx).handle.clone();
            handle.focus(window, cx);
            on_select(option, window, cx);
        })
    }

    fn rows(&self, focus: &Entity<GridFocus>, focused_at: Option<usize>) -> Vec<Div> {
        T::ALL
            .chunks(self.columns)
            .enumerate()
            .map(|(row, options)| {
                let cards: Vec<Stateful<Div>> = options
                    .iter()
                    .enumerate()
                    .map(|(col, option)| {
                        let at = row * self.columns + col;
                        self.card(at, *option, focused_at == Some(at), focus)
                    })
                    .collect();
                div().flex().gap(self.theme.spacing_sm).children(cards)
            })
            .collect()
    }
}

/// Moves the cursor or picks the option under it for one key press.
fn on_key<T: TraitOption>(focus: &Entity<GridFocus>, key: &str, columns: usize, on_select: &ValueCallback<T>, window: &mut Window, cx: &mut App) {
    let cursor = GridCursor::new(focus.read(cx).cursor, T::ALL.len(), columns);
    if let Some(nav) = NavKey::from_key(key) {
        let moved = cursor.moved(nav).index;
        focus.update(cx, |state, cx| {
            state.cursor = moved;
            cx.notify();
        });
    } else if ACTIVATE_KEYS.contains(&key) {
        if let Some(option) = T::ALL.get(cursor.index) {
            on_select(*option, window, cx);
        }
    }
}

impl<T: TraitOption> RenderOnce for IllustratedOptionGrid<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let initial = self.selected_index();
        let focus = window.use_keyed_state(self.id.clone(), cx, |_, cx| GridFocus { handle: cx.focus_handle(), cursor: initial });
        let handle = focus.read(cx).handle.clone();
        let focused_at = handle.is_focused(window).then(|| focus.read(cx).cursor);
        let rows = self.rows(&focus, focused_at);
        let (on_select, columns, key_focus) = (self.on_select.clone(), self.columns, focus.clone());
        div()
            .id(self.id.clone())
            .track_focus(&handle)
            .flex()
            .flex_col()
            .gap(self.theme.spacing_sm)
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                on_key(&key_focus, event.keystroke.key.as_str(), columns, &on_select, window, cx);
            })
            .children(rows)
    }
}
