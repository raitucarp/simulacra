//! The calendar view (W4): one month as a grid of weeks from the user's first weekday, each
//! day listing the start of the posts written on it (a click opens one). The header steps a
//! month back or forward, or back to this month.

use super::common::section_heading;
use super::input::ViewInput;
use crate::domain::views::calendar::{column_weekdays, month_grid, CalendarDay, YearMonth};
use crate::domain::views::rapid_log::one_line;
use crate::ui::features::day_filter::month_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use jiff::civil::Date;
use rust_i18n::t;

/// Posts listed in one day cell before "+N".
const POSTS_PER_DAY: usize = 3;

/// The month shown: the chosen one, else the month of the newest post, else this month.
pub fn shown_month(input: &ViewInput<'_>, today: Date) -> YearMonth {
    let newest = input.posts.iter().map(|post| post.created_at).max().map(crate::time::local_date);
    input.month.unwrap_or_else(|| YearMonth::of(newest.unwrap_or(today)))
}

/// The short name of `day` ("Mon").
fn weekday_name(day: jiff::civil::Weekday) -> String {
    let key = format!("journal.weekday_short.{}", day.to_monday_zero_offset());
    t!(key.as_str()).to_string()
}

pub(super) fn render_calendar(input: &ViewInput<'_>) -> AnyElement {
    let theme = input.theme;
    let today = crate::time::local_now().date();
    let month = shown_month(input, today);
    let first = input.state.preferences().general.first_weekday;
    let weeks = month_grid(month, first, input.posts.iter().copied(), crate::time::local_date);
    let names = column_weekdays(first).map(weekday_name);
    let header_row = div().flex().gap(theme.spacing_3xs).children(names.into_iter().map(|name| div().flex_1().min_w_0().truncate().child(section_heading(name, theme))));
    let rows = weeks.iter().map(|week| div().flex().gap(theme.spacing_3xs).children(week.iter().map(|day| render_day(day, today, input))));
    let grid = div().flex().flex_col().gap(theme.spacing_3xs).child(header_row).children(rows);
    let body = div().id(input.id("calendar")).flex_1().min_h_0().w_full().overflow_y_scroll().px(theme.panel_padding_compact).py(theme.spacing_xs).child(grid);
    div().flex_1().min_h_0().w_full().flex().flex_col().child(render_header(input, month, today)).child(body).into_any_element()
}

fn render_header(input: &ViewInput<'_>, month: YearMonth, today: Date) -> Div {
    let theme = input.theme;
    let step = |part: &str, icon: LucideIcon, key: &str, target: YearMonth| {
        let on_month = input.on_month.clone();
        Button::new(input.id(part)).ghost().xsmall().icon(Icon::new(icon)).tooltip(t!(key).to_string()).on_click(move |_, window, cx| on_month(target, window, cx))
    };
    let on_today = input.on_month.clone();
    let this_month = YearMonth::of(today);
    let today_button = Button::new(input.id("calendar-today")).ghost().xsmall().label(t!("views.calendar.today").to_string()).on_click(move |_, window, cx| on_today(this_month, window, cx));
    let title = t!("views.calendar.month_title", month = month_label(month.month), year = month.year).to_string();
    div()
        .flex()
        .items_center()
        .gap(theme.control_gap)
        .px(theme.panel_padding)
        .py(theme.spacing_2xs)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(step("calendar-previous", LucideIcon::ChevronLeft, "views.calendar.previous", month.shifted(-1)))
        .child(div().flex_1().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title))
        .child(step("calendar-next", LucideIcon::ChevronRight, "views.calendar.next", month.shifted(1)))
        .when(month != this_month, |header| header.child(today_button))
}

fn render_day(day: &CalendarDay, today: Date, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let number_color = match (day.date == today, day.in_month) {
        (true, _) => theme.accent_brand,
        (false, true) => theme.text_secondary,
        (false, false) => theme.text_muted,
    };
    let number = div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(number_color).child(day.date.day().to_string());
    let more = day.posts.len().saturating_sub(POSTS_PER_DAY);
    let snippets = day.posts.iter().take(POSTS_PER_DAY).filter_map(|id| input.state.get_post(id)).map(|post| render_snippet(post, input));
    div()
        .flex_1()
        .min_w_0()
        .min_h(theme.calendar_cell_min_height)
        .p(theme.spacing_3xs)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .when(day.in_month, |cell| cell.bg(theme.bg_card))
        .child(number)
        .children(snippets)
        .when(more > 0, |cell| cell.child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("views.calendar.more", count = more).to_string())))
}

fn render_snippet(post: &crate::domain::Post, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let hover = theme.hover_bg;
    let row = div().id(input.id(format!("calendar-post-{}", post.id))).truncate().rounded(theme.radius_xs).cursor_pointer().hover(move |row| row.bg(hover)).text_size(theme.font_size_2xs).text_color(theme.text_primary).on_click(input.opener(&post.id));
    match post.is_locked() {
        true => row.child(Icon::new(LucideIcon::LockKeyhole).xsmall().text_color(theme.text_muted)),
        false => row.child(one_line(post)),
    }
}
