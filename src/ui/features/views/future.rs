//! The future log (W5): scheduled posts and dated to-dos from today on, by month. Each line
//! shows its day, a clock for a scheduled post or the to-do's state symbol, and its text; a
//! click opens the post it belongs to.

use super::common::{empty_note, locked_note, scroll_column, section_heading};
use super::input::ViewInput;
use crate::domain::symbols::{EntrySymbol, SymbolArt};
use crate::domain::views::future::{future_log, FutureEntry, FutureItem, FutureMonth};
use crate::ui::features::day_filter::month_label;
use crate::ui::features::symbols::render_symbol_art;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

pub(super) fn render_future(input: &ViewInput<'_>) -> AnyElement {
    let today = crate::time::local_now().date();
    let posts = input.posts.iter().chain(&input.scheduled).copied();
    let months = future_log(posts, today, crate::time::local_date);
    if months.is_empty() {
        return empty_note("views.future.empty", input.theme);
    }
    scroll_column(input, "future").children(months.iter().map(|month| render_month(month, input))).into_any_element()
}

fn render_month(month: &FutureMonth, input: &ViewInput<'_>) -> Div {
    let title = t!("views.calendar.month_title", month = month_label(month.month.month), year = month.month.year).to_string();
    div().flex().flex_col().gap(input.theme.spacing_3xs).child(section_heading(title, input.theme)).children(month.entries.iter().enumerate().map(|(i, entry)| render_entry(i, entry, input)))
}

fn render_entry(index: usize, entry: &FutureEntry, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let line = theme.font_size_sm * theme.line_height_body;
    let day = div().flex_none().w(theme.view_gutter_width).text_size(theme.font_size_xs).font_family(theme.font_family_mono.clone()).text_color(theme.text_muted).child(entry.day.day().to_string());
    let (mark, locked): (AnyElement, bool) = match entry.item {
        FutureItem::Post { locked } => (Icon::new(LucideIcon::CalendarClock).size(theme.symbol_outline_size).text_color(theme.accent_brand).into_any_element(), locked),
        FutureItem::Todo { state, .. } => (render_symbol_art(SymbolArt::Entry(EntrySymbol::Task, Some(state)), theme.symbol_outline_size).into_any_element(), false),
    };
    let text = match locked {
        true => locked_note(theme).into_any_element(),
        false => div().flex_1().min_w_0().text_size(theme.font_size_sm).line_height(line).text_color(theme.text_primary).child(entry.text.clone()).into_any_element(),
    };
    let hover = theme.hover_bg;
    div()
        .id(input.id(format!("future-{}-{index}-{}", entry.post_id, entry.day)))
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .hover(move |row| row.bg(hover))
        .on_click(input.opener(&entry.post_id))
        .child(day)
        .child(div().flex_none().h(line).flex().items_center().child(mark))
        .child(text)
}
