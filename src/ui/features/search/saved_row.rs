//! The saved searches row (Q5): one chip per saved search (a click runs it, the mark next to
//! it forgets it), and "Save search" while the query is new.

use super::{SearchAction, SearchEvent, SearchView};
use crate::theme::ThemeTokens;
use crate::ui::components::chip::{choice_chip, chip_row};
use gpui_kit::*;
use rust_i18n::t;

const SAVED_ID: &str = "search-saved";
const FORGET_ID: &str = "search-forget";
const SAVE_ID: &str = "search-save";
/// The mark that forgets a saved search.
const FORGET_MARK: &str = "×";

fn event_chip(id: SharedString, label: &str, selected: bool, event: SearchEvent, on_event: &SearchAction, theme: &ThemeTokens) -> Stateful<Div> {
    let on_event = on_event.clone();
    choice_chip(id, label, selected, theme, move |window, cx| on_event(event.clone(), window, cx))
}

fn forget(n: usize, query: &str, on_event: &SearchAction, theme: &ThemeTokens) -> Stateful<Div> {
    let (on_event, query, hover) = (on_event.clone(), query.to_string(), theme.text_primary);
    div()
        .id(SharedString::from(format!("{FORGET_ID}-{n}")))
        .px(theme.spacing_3xs)
        .cursor_pointer()
        .text_size(theme.font_size_xs)
        .text_color(theme.text_muted)
        .hover(move |style| style.text_color(hover))
        .child(FORGET_MARK)
        .on_click(move |_, window, cx| on_event(SearchEvent::ForgetSaved(query.clone()), window, cx))
}

/// The row; `None` with nothing saved and nothing to save.
pub(super) fn saved_row(view: &SearchView<'_>, theme: &ThemeTokens) -> Option<Div> {
    let current = view.query.to_text();
    let can_save = !view.query.is_empty() && !view.saved.contains(&current);
    if view.saved.items.is_empty() && !can_save {
        return None;
    }
    let saved = view.saved.items.iter().enumerate().map(|(n, item)| {
        let open = event_chip(SharedString::from(format!("{SAVED_ID}-{n}")), &item.name, item.query == current, SearchEvent::OpenSaved(item.query.clone()), &view.on_event, theme);
        div().flex().items_center().child(open).child(forget(n, &item.query, &view.on_event, theme))
    });
    let save = can_save.then(|| event_chip(SharedString::from(SAVE_ID), &t!("search.page.save"), false, SearchEvent::SaveQuery, &view.on_event, theme));
    let label = div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("search.page.saved").to_string());
    Some(div().flex().items_center().flex_wrap().gap(theme.control_gap).child(label).child(chip_row(theme).children(saved).children(save)))
}
