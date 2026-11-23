//! N13: the "Still true?" view: the claims due for a check, the one due longest first, each
//! with when it fell due; a click opens it, where its strip offers Confirm, Revise and Retract.

use super::common::{empty_note, scroll_column};
use super::input::ViewInput;
use crate::domain::views::still_true::{due_claims, DueClaim};
use gpui_kit::*;
use rust_i18n::t;

/// How the view shows when a claim fell due.
const DATE_FORMAT: &str = "%Y-%m-%d";

pub(super) fn render_still_true(input: &ViewInput<'_>) -> AnyElement {
    let due = due_claims(input.posts.iter().copied(), jiff::Timestamp::now());
    if due.is_empty() {
        return empty_note("views.still_true.empty", input.theme);
    }
    scroll_column(input, "still-true").children(due.iter().map(|claim| render_claim(claim, input))).into_any_element()
}

fn render_claim(claim: &DueClaim, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let date = crate::time::local(claim.due).strftime(DATE_FORMAT).to_string();
    let when = div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("views.still_true.due", date = date).to_string());
    let text = div().flex_1().min_w_0().truncate().text_size(theme.font_size_sm).text_color(theme.text_primary).child(claim.text.clone());
    let hover = theme.hover_bg;
    div()
        .id(input.id(format!("still-true-{}", claim.post_id)))
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .hover(move |row| row.bg(hover))
        .on_click(input.opener(&claim.post_id))
        .child(text)
        .child(when)
}
