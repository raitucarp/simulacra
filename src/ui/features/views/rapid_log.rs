//! The rapid log (W2): bullets grouped by day, a post's symbol as its bullet and its
//! signifiers before it; a click opens the post.

use super::common::{empty_note, locked_note, scroll_column, section_heading};
use super::input::ViewInput;
use crate::domain::views::rapid_log::{rapid_log, LogDay, LogEntry};
use crate::ui::features::journal::text::day_label;
use crate::ui::features::symbols::{render_glyph, render_symbol_art};
use gpui_kit::*;

const PLAIN_BULLET: &str = "•";

/// The log of the input's posts, or why it is empty.
pub(super) fn render_rapid_log(input: &ViewInput<'_>) -> AnyElement {
    let days = rapid_log(input.posts.iter().copied(), crate::time::local_date);
    if days.is_empty() {
        return empty_note("views.rapid_log.empty", input.theme);
    }
    scroll_column(input, "rapid-log").children(days.iter().map(|day| render_day(day, input))).into_any_element()
}

fn render_day(day: &LogDay, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let entries = day.entries.iter().map(|entry| render_entry(entry, input));
    div().flex().flex_col().gap(theme.spacing_3xs).child(section_heading(day_label(day.day), theme)).children(entries)
}

fn render_entry(entry: &LogEntry, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let line = theme.font_size_sm * theme.line_height_body;
    let marks = entry.marks.iter().map(|art| render_symbol_art(*art, theme.symbol_badge_size));
    let bullet = match &entry.bullet {
        Some(glyph) => render_glyph(glyph, theme.symbol_outline_size, theme),
        None => div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(PLAIN_BULLET).into_any_element(),
    };
    let gutter = div().flex_none().w(theme.view_gutter_width).h(line).flex().items_center().justify_end().gap(theme.spacing_3xs).children(marks).child(bullet);
    let text = match entry.locked {
        true => locked_note(theme).into_any_element(),
        false => div().flex_1().min_w_0().text_size(theme.font_size_sm).line_height(line).text_color(theme.text_primary).child(entry.text.clone()).into_any_element(),
    };
    let hover = theme.hover_bg;
    div()
        .id(input.id(format!("log-{}", entry.id)))
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .hover(move |row| row.bg(hover))
        .on_click(input.opener(&entry.id))
        .child(gutter)
        .child(text)
}
