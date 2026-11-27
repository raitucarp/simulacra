//! N18: the open questions view: every question still open, the one asked longest ago first,
//! each with when it was asked; a click opens it. A locked one shows only its placeholder.

use super::common::{empty_note, locked_note, scroll_column};
use super::input::ViewInput;
use crate::domain::views::open_questions::{open_rows, OpenRow};
use gpui_kit::*;
use rust_i18n::t;


pub(super) fn render_open_questions(input: &ViewInput<'_>) -> AnyElement {
    let rows = open_rows(input.posts.iter().copied());
    if rows.is_empty() {
        return empty_note("views.open_questions.empty", input.theme);
    }
    scroll_column(input, "open-questions").children(rows.iter().map(|row| render_row(row, input))).into_any_element()
}

/// One open question as a clickable row (shared with the open questions page).
pub fn render_row(row: &OpenRow, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let date = crate::ui::format::date(crate::time::local(row.asked).date());
    let when = div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("views.open_questions.asked", date = date).to_string());
    let text = match &row.text {
        None => locked_note(theme).flex_1().into_any_element(),
        Some(text) => div().flex_1().min_w_0().truncate().text_size(theme.font_size_sm).text_color(theme.text_primary).child(text.clone()).into_any_element(),
    };
    let hover = theme.hover_bg;
    div()
        .id(input.id(format!("open-question-{}", row.post_id)))
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .hover(move |r| r.bg(hover))
        .on_click(input.opener(&row.post_id))
        .child(text)
        .child(when)
}
