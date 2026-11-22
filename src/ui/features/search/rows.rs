//! One result row: the author, the day and how it was found, then the passage with its
//! matching words marked. The selected row is filled; a click opens the post.

use super::{SearchAction, SearchEvent, SearchRow};
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const ROW_ID: &str = "search-row";

fn byline(row: &SearchRow, theme: &ThemeTokens) -> Div {
    let muted = |text: String| div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(text);
    let author = div().min_w_0().truncate().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(row.author.clone());
    let meaning = row.by_meaning.then(|| muted(t!("search.page.by_meaning").to_string()));
    div().flex().items_baseline().gap(theme.control_gap).child(author).child(muted(row.handle.clone())).child(muted(row.day.clone())).children(meaning)
}

fn passage(row: &SearchRow, theme: &ThemeTokens) -> Div {
    let style = HighlightStyle { color: Some(theme.accent_brand.into()), font_weight: Some(FontWeight::SEMIBOLD), ..Default::default() };
    let highlights = row.snippet.highlights.iter().cloned().map(move |range| (range, style));
    div().text_size(theme.font_size_sm).text_color(theme.text_secondary).child(StyledText::new(row.snippet.text.clone()).with_highlights(highlights))
}

/// The row at `index`.
pub(super) fn result_row(index: usize, row: &SearchRow, selected: bool, on_event: &SearchAction, theme: &ThemeTokens) -> Stateful<Div> {
    let (on_event, id, hover) = (on_event.clone(), row.post_id.clone(), theme.hover_bg);
    div()
        .id((ROW_ID, index))
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .when(selected, |row| row.bg(theme.accent_brand_subtle))
        .when(!selected, |row| row.hover(move |style| style.bg(hover)))
        .child(byline(row, theme))
        .child(passage(row, theme))
        .on_click(move |_, window, cx| on_event(SearchEvent::Open(id.clone()), window, cx))
}
