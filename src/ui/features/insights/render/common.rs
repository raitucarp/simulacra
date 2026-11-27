//! Building blocks of the Insights page: chart cards, titles, key-number tiles, option chips,
//! grids that pair charts on wider pages, and the loading and empty states.

use super::super::panel::InsightsPanel;
use super::super::state::PageState;
use super::Room;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// The translation of `key`.
pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

/// A card holding one chart.
pub fn card(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .p(theme.card_padding)
        .rounded(theme.radius_lg)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
}

/// A card's icon and title, with an optional control on the right.
pub fn card_title(icon: LucideIcon, title: String, trailing: Option<AnyElement>, theme: &ThemeTokens) -> Div {
    let heading = div()
        .flex_1()
        .min_w_0()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .child(Icon::new(icon).size(theme.icon_size_sm).text_color(theme.accent_brand_text))
        .child(div().truncate().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title));
    div().w_full().flex().flex_wrap().items_center().gap(theme.control_gap).child(heading).children(trailing)
}

/// A key number over its label.
pub fn tile(value: String, label: String, colour: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .min_w(theme.insights_tile_width)
        .flex_1()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(colour).child(value))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label))
}

/// A grid of `columns` equal tile columns (no stretched orphan tile on the last row).
pub fn tile_row(columns: u16, theme: &ThemeTokens) -> Div {
    div().w_full().grid().grid_cols(columns.max(1)).gap(theme.control_gap)
}

/// Columns for `count` tiles: all in one row when the page is wide, else rows of three
/// (or two when three does not divide them).
pub fn tile_columns(room: Room, count: usize) -> u16 {
    match room {
        Room::Wide => count as u16,
        _ if count.is_multiple_of(3) => 3,
        _ => 2,
    }
}

/// Charts side by side when the page has room, stacked otherwise. Side by side, the two cards
/// share the taller one's height (each cell is a row, which stretches its card), so the pair
/// ends on one line instead of a ragged edge.
pub fn pair(room: Room, first: impl IntoElement, second: impl IntoElement, theme: &ThemeTokens) -> Div {
    let cell = |child: AnyElement| div().flex_1().min_w_0().flex().child(child);
    let row = div().w_full().flex().gap(theme.section_gap);
    let row = if room == Room::Narrow { row.flex_col() } else { row };
    row.child(cell(first.into_any_element())).child(cell(second.into_any_element()))
}

/// A small selectable option (filters and chart choices), reachable by keyboard.
pub fn option_chip(id: ElementId, label: String, selected: bool, theme: &ThemeTokens, on_pick: Rc<dyn Fn(&mut Window, &mut App)>) -> Stateful<Div> {
    let pick = Rc::clone(&on_pick);
    crate::ui::components::choice_chip(id, &label, selected, theme, move |window, cx| pick(window, cx)).keyboard_stop(theme, on_pick)
}

/// A labelled row of option chips.
pub fn option_group(label: String, chips: Vec<Stateful<Div>>, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(theme.spacing_xs)
        .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(label))
        .children(chips)
}

/// A chip or button action.
pub type Pick = Rc<dyn Fn(&mut Window, &mut App)>;

impl InsightsPanel {
    /// A callback applying `change` to the page state.
    pub(super) fn pick(&self, cx: &mut Context<Self>, change: impl Fn(&mut PageState) + 'static) -> Pick {
        let entity = cx.entity().downgrade();
        let change = Rc::new(change);
        Rc::new(move |_, cx| {
            let change = Rc::clone(&change);
            let _ = entity.update(cx, |panel, cx| panel.update_state(cx, |state| change(state)));
        })
    }

    /// An illustrated empty state.
    fn empty_state(&self, key: EmptyKey, title: String, hint: String) -> Div {
        let theme = &self.theme;
        div()
            .w_full()
            .py(theme.section_gap)
            .flex()
            .flex_col()
            .items_center()
            .gap(theme.stack_gap)
            .child(Illustration::empty(key).width(theme.review_empty_art_width))
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title))
            .child(div().max_w(theme.review_card_max_width).text_center().text_size(theme.font_size_xs).text_color(theme.text_muted).child(hint))
    }

    pub(super) fn render_empty(&self) -> Div {
        self.empty_state(EmptyKey::NoResults, tr("insights.empty.title"), tr("insights.empty.hint"))
    }

    pub(super) fn render_loading(&self) -> Div {
        self.empty_state(EmptyKey::GraphEmpty, tr("insights.loading.title"), tr("insights.loading.hint"))
    }

    /// An empty state inside one view (not enough material for this chart).
    pub(super) fn view_empty(&self, title_key: &str, hint_key: &str) -> Div {
        self.empty_state(EmptyKey::GraphEmpty, tr(title_key), tr(hint_key))
    }
}
