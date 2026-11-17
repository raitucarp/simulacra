//! [`MultiSelectGrid`]: every option of a `TraitOption` as an illustrated card that toggles in
//! or out of an [`OptionSet`]. It is one tab stop (a [`CardGrid`]): arrow keys / Home / End
//! move the focus-ring cursor, Enter / Space toggle the card under it, clicking toggles too.
//! Chosen cards carry the brand ring and a check badge over their art.
//! [`trait_set_grid`] does the same for a runtime [`TraitSet`].

use super::card_grid::CardGrid;
use super::grid_cell::GridCell;
use super::option_card::OptionCardSpec;
use super::ValueCallback;
use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::OptionSet;
use crate::domain::TraitOption;
use crate::state::forge::{TraitSet, TraitSetValue, TraitSetVisitor};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{trait_description, trait_label};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::Illustration;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

/// Illustrated, toggleable cards for every option of `T`.
#[derive(IntoElement)]
pub struct MultiSelectGrid<T: TraitOption> {
    id: ElementId,
    selected: OptionSet<T>,
    columns: usize,
    card_width: Pixels,
    descriptions: bool,
    theme: ThemeTokens,
    on_change: ValueCallback<OptionSet<T>>,
}

impl<T: TraitOption> MultiSelectGrid<T> {
    /// A grid of `T::ALL` with every option of `selected` ringed; toggling a card calls
    /// `on_change` with the whole new set.
    pub fn new(
        id: impl Into<ElementId>,
        selected: &OptionSet<T>,
        theme: &ThemeTokens,
        on_change: impl Fn(OptionSet<T>, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            selected: selected.clone(),
            columns: super::GRID_COLUMNS,
            card_width: theme.forge_option_card_width,
            descriptions: false,
            theme: theme.clone(),
            on_change: Rc::new(on_change),
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

    /// Shows the short descriptions under the labels.
    pub fn with_descriptions(mut self) -> Self {
        self.descriptions = true;
        self
    }

    /// The option's art with a check badge in its corner when chosen.
    fn art(&self, option: T, chosen: bool) -> Div {
        let theme = &self.theme;
        let badge = div()
            .absolute()
            .top(theme.spacing_xs)
            .right(theme.spacing_xs)
            .p(theme.spacing_3xs)
            .rounded(theme.radius_full)
            .bg(theme.accent_brand)
            .text_color(theme.on_accent)
            .child(Icon::new(LucideIcon::Check).size(theme.font_size_xs));
        div().relative().child(Illustration::trait_card(option).width(self.card_width)).when(chosen, |art| art.child(badge))
    }

    fn cell(&self, at: usize, option: T) -> GridCell {
        let chosen = self.selected.contains(option);
        let art = self.art(option, chosen);
        let mut spec = OptionCardSpec::new((self.id.clone(), at.to_string()), art, trait_label(option), self.card_width);
        spec.description = self.descriptions.then(|| trait_description(option)).flatten();
        spec.selected = chosen;
        let (on_change, toggled) = (self.on_change.clone(), self.selected.toggled(option));
        GridCell::card(spec, &self.theme, Rc::new(move |window, cx| on_change(toggled.clone(), window, cx)))
    }
}

impl<T: TraitOption> RenderOnce for MultiSelectGrid<T> {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let cells = T::ALL.iter().enumerate().map(|(at, option)| self.cell(at, *option)).collect();
        CardGrid::new(self.id, cells, self.columns, self.theme.spacing_sm).auto_columns(self.card_width)
    }
}

/// Builds the grid once the concrete option type is known.
struct SetPick {
    id: SharedString,
    theme: ThemeTokens,
    columns: usize,
    card_width: Pixels,
    on_change: ValueCallback<TraitSetValue>,
}

impl TraitSetVisitor for SetPick {
    type Output = AnyElement;

    fn visit<T: TraitOption>(self, current: &OptionSet<T>, wrap: fn(OptionSet<T>) -> TraitSetValue) -> AnyElement {
        let on_change = self.on_change;
        MultiSelectGrid::new(self.id, current, &self.theme, move |set: OptionSet<T>, window, cx| on_change(wrap(set), window, cx))
            .columns(self.columns)
            .card_width(self.card_width)
            .into_any_element()
    }
}

/// Toggleable cards for every option of `field`, ringing those chosen on `cfg`, `columns` per
/// row and `card_width` wide; each toggle calls `on_change` with the field's new value.
pub fn trait_set_grid(
    id: SharedString,
    field: TraitSet,
    cfg: &AgentConfig,
    (columns, card_width): (usize, Pixels),
    theme: &ThemeTokens,
    on_change: ValueCallback<TraitSetValue>,
) -> AnyElement {
    field.read(cfg).visit(SetPick { id, theme: theme.clone(), columns, card_width, on_change })
}
