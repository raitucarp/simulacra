//! The scrolling body: a category's grouped cards, or the search results grouped by
//! category with their matches highlighted.

use super::super::catalog::{groups, SearchHit, SettingRow, SettingsGroup};
use super::super::panes::common::section_label;
use super::super::row::{render_row, RowContext, RowHighlight};
use super::super::tabs::SettingsTab;
use super::embedded::EmbeddedPanes;
use crate::theme::ThemeTokens;
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use super::super::text::tr;
use rust_i18n::t;

/// The bordered card of a group, its rows divided by hairlines.
fn group_card(theme: &ThemeTokens, rows: Vec<AnyElement>) -> Div {
    let divided = rows.into_iter().enumerate().map(|(index, row)| {
        div().w_full().when(index > 0, |row| row.border_t(theme.border_width_hairline).border_color(theme.border_subtle)).child(row)
    });
    div()
        .w_full()
        .px(theme.card_padding)
        .py(theme.spacing_xs)
        .rounded(theme.radius_lg)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .flex()
        .flex_col()
        .children(divided)
}

/// A titled group of `tab`.
fn render_group(ctx: &RowContext, tab: SettingsTab, group: &SettingsGroup, panes: &mut EmbeddedPanes) -> AnyElement {
    let theme = ctx.theme;
    let rows: Vec<SettingRow> = group.rows.iter().copied().filter(|row| row.listed_in_category()).collect();
    if let [SettingRow::Embedded(pane)] = rows.as_slice() {
        if EmbeddedPanes::is_block(*pane) {
            return panes.take(theme, *pane).unwrap_or_else(|| div().into_any_element());
        }
    }
    let items = rows
        .into_iter()
        .filter_map(|row| match row {
            SettingRow::Embedded(pane) => panes.take(theme, pane),
            _ => Some(render_row(ctx, row, &RowHighlight::default()).into_any_element()),
        })
        .collect();
    let title = tr(&group.title_key(tab));
    div().flex().flex_col().gap(theme.spacing_sm).child(section_label(theme, title)).child(group_card(theme, items)).into_any_element()
}

/// Every group of `tab`.
pub(in crate::ui::features::settings) fn render_category(ctx: &RowContext, tab: SettingsTab, mut panes: EmbeddedPanes) -> Div {
    let cards: Vec<AnyElement> = groups(tab).iter().map(|group| render_group(ctx, tab, group, &mut panes)).collect();
    div().w_full().flex().flex_col().gap(ctx.theme.section_gap).children(cards)
}

/// The search results, grouped by category in rail order.
pub(in crate::ui::features::settings) fn render_search(ctx: &RowContext, hits: &[SearchHit]) -> Div {
    let theme = ctx.theme;
    if hits.is_empty() {
        return no_results(theme);
    }
    let sections = SettingsTab::ALL.into_iter().filter_map(|tab| {
        let rows: Vec<AnyElement> =
            hits.iter().filter(|hit| hit.tab == tab).map(|hit| render_row(ctx, hit.row, &RowHighlight::of(hit)).into_any_element()).collect();
        (!rows.is_empty()).then(|| div().flex().flex_col().gap(theme.spacing_sm).child(section_heading(theme, tab)).child(group_card(theme, rows)))
    });
    div().w_full().flex().flex_col().gap(theme.section_gap).children(sections)
}

/// A category's icon and name above its results.
fn section_heading(theme: &ThemeTokens, tab: SettingsTab) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .child(Icon::new(tab.icon()).size(theme.icon_size_sm).text_color(theme.accent_brand_text))
        .child(section_label(theme, tab.title()))
}

/// The empty state of a search without matches.
fn no_results(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .py(theme.section_gap)
        .flex()
        .flex_col()
        .items_center()
        .gap(theme.stack_gap)
        .child(Illustration::empty(EmptyKey::NoResults).width(theme.settings_empty_art_width))
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!("prefs.ui.no_results").to_string()))
        .child(div().text_size(theme.font_size_hint).text_color(theme.text_muted).child(t!("prefs.ui.no_results_hint").to_string()))
}
