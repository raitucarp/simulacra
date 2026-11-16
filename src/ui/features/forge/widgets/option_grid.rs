//! [`IllustratedOptionGrid`]: every option of a `TraitOption` as an illustrated card, laid
//! out in rows of [`GRID_COLUMNS`] by a keyboard-navigable [`CardGrid`].

use super::card_grid::CardGrid;
use super::grid_cell::GridCell;
use super::option_card::OptionCardSpec;
use super::ValueCallback;
use crate::domain::TraitOption;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{trait_description, trait_label};
use crate::ui::illustration::Illustration;
use gpui_kit::*;
use std::rc::Rc;

/// Default number of cards per row.
pub const GRID_COLUMNS: usize = 3;

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

    fn cell(&self, at: usize, option: T) -> GridCell {
        let art = Illustration::trait_card(option).width(self.card_width);
        let mut spec = OptionCardSpec::new((self.id.clone(), at.to_string()), art, trait_label(option), self.card_width);
        spec.description = self.descriptions.then(|| trait_description(option)).flatten();
        spec.selected = self.selected.is_some_and(|chosen| chosen.key() == option.key());
        let on_select = self.on_select.clone();
        GridCell::card(spec, &self.theme, Rc::new(move |window, cx| on_select(option, window, cx)))
    }
}

impl<T: TraitOption> RenderOnce for IllustratedOptionGrid<T> {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let cells = T::ALL.iter().enumerate().map(|(at, option)| self.cell(at, *option)).collect();
        CardGrid::new(self.id, cells, self.columns, self.theme.spacing_sm)
    }
}
