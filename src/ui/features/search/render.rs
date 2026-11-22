//! The Search page: title, field, filter chips, the meaning note, then the results (or the
//! prompt, or why nothing was found). Arrow keys and Enter are caught before the field.

use super::chips::{active_chips, QUICK_FILTERS};
use super::rows::result_row;
use super::saved_row::saved_row;
use super::{SearchAction, SearchEvent, SearchView};
use crate::theme::ThemeTokens;
use crate::ui::components::chip::{choice_chip, chip_row};
use crate::ui::components::{art_state, ArtState};
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::component::input::Input;
use gpui_kit::*;
use crate::ui::features::data::tr;
use rust_i18n::t;

const ROOT_ID: &str = "search-page";
const ACTIVE_ID: &str = "search-filter";
const QUICK_ID: &str = "search-quick";
const VERSIONS_ID: &str = "search-versions";
/// The mark a removable chip ends with.
const REMOVE_MARK: &str = "×";

/// The event a navigation key asks for, if it is one.
fn key_event(key: &str) -> Option<SearchEvent> {
    match key {
        "down" => Some(SearchEvent::Step(true)),
        "up" => Some(SearchEvent::Step(false)),
        "enter" => Some(SearchEvent::Confirm),
        _ => None,
    }
}

fn chip(id: String, label: &str, selected: bool, event: SearchEvent, on_event: &SearchAction, theme: &ThemeTokens) -> Stateful<Div> {
    let on_event = on_event.clone();
    choice_chip(SharedString::from(id), label, selected, theme, move |window, cx| on_event(event.clone(), window, cx))
}

fn chips(view: &SearchView<'_>, theme: &ThemeTokens) -> Div {
    let active = active_chips(view.query).into_iter().enumerate().map(|(n, chip_of)| {
        let label = format!("{} {REMOVE_MARK}", chip_of.label);
        chip(format!("{ACTIVE_ID}-{n}"), &label, true, SearchEvent::RemoveFilter(chip_of.filter, chip_of.negated), &view.on_event, theme)
    });
    let quick = QUICK_FILTERS.iter().filter(|filter| !view.query.filters.contains(filter)).enumerate().map(|(n, filter)| {
        chip(format!("{QUICK_ID}-{n}"), &filter.to_text(), false, SearchEvent::ToggleFilter(filter.clone()), &view.on_event, theme)
    });
    let label = div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("search.page.filters").to_string());
    let versions = chip(VERSIONS_ID.to_string(), &t!("search.page.include_versions"), view.include_versions, SearchEvent::ToggleVersions, &view.on_event, theme); // Q6
    div().flex().items_center().flex_wrap().gap(theme.control_gap).child(label).child(chip_row(theme).children(active).children(quick).child(versions))
}

fn header(view: &SearchView<'_>, theme: &ThemeTokens) -> Div {
    let title = div().text_size(theme.font_size_xl).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!("search.page.title").to_string());
    let count = (!view.query.is_empty()).then(|| div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(crate::i18n::t_count("search.page.count", view.rows.len())));
    div().flex().items_baseline().gap(theme.control_gap).child(title).children(count)
}

fn field(view: &SearchView<'_>, theme: &ThemeTokens) -> Div {
    div().w_full().rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border).bg(theme.bg_card).p(theme.spacing_xs).child(Input::new(view.input).cleanable(true))
}

fn results(view: &SearchView<'_>, theme: &ThemeTokens) -> Div {
    let (title, hint) = match (view.query.is_empty(), view.rows.is_empty()) {
        (true, _) => ("search.page.prompt_title", "search.page.prompt_hint"),
        (false, true) => ("search.page.empty_title", "search.page.empty_hint"),
        (false, false) => {
            let rows = view.rows.iter().enumerate().map(|(n, row)| result_row(n, row, view.selected == Some(n), &view.on_event, theme));
            return div().flex().flex_col().children(rows);
        }
    };
    let art = Illustration::empty(EmptyKey::NoResults).width(theme.art_empty_width);
    div().py(theme.section_gap).flex().justify_center().child(art_state(ArtState::titled(art, tr(title), tr(hint)), theme))
}

/// The page.
pub fn render_search_page(view: SearchView<'_>, theme: &ThemeTokens) -> AnyElement {
    let note = view.note.map(|note| div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(note.to_string()));
    let column = div().w_full().max_w(theme.search_page_max_width).flex().flex_col().gap(theme.section_gap).child(header(&view, theme)).child(field(&view, theme)).child(chips(&view, theme)).children(saved_row(&view, theme)).children(note).child(results(&view, theme));
    let on_event = view.on_event.clone();
    div()
        .id(ROOT_ID)
        .size_full()
        .overflow_y_scroll()
        .bg(theme.bg_app)
        .flex()
        .justify_center()
        .items_start()
        .px(theme.panel_padding)
        .py(theme.panel_padding)
        .capture_key_down(move |event: &KeyDownEvent, window, cx| {
            if event.keystroke.modifiers.modified() {
                return;
            }
            if let Some(wanted) = key_event(event.keystroke.key.as_str()) {
                window.prevent_default();
                cx.stop_propagation();
                on_event(wanted, window, cx);
            }
        })
        .child(column)
        .into_any_element()
}

#[cfg(test)]
pub(super) fn key_event_for_tests(key: &str) -> Option<SearchEvent> {
    key_event(key)
}
