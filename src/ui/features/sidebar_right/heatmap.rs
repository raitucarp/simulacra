//! Clickable GitHub-style thought cadence heatmap calendar.

use std::rc::Rc;

use super::heatmap_chrome::{render_day_labels, render_heatmap_header, render_legend, render_subtitle};
use super::section::render_divided_section;
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

pub type ClickDayCallback = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// Number of week columns in the calendar grid.
pub(super) const WEEKS: usize = 15;
/// Number of day rows (Sun..Sat) in each week column.
pub(super) const DAYS_PER_WEEK: usize = 7;
/// Total number of day cells in the grid.
pub(super) const CELL_COUNT: usize = WEEKS * DAYS_PER_WEEK;
/// Gap between calendar cells and week columns.
pub(super) const CELL_GAP: f32 = 2.5;
const CELL_ID_PREFIX: &str = "cal-cell";

/// Static activity levels (0..=4) for every day cell, laid out week by week.
#[rustfmt::skip]
pub(super) const ACTIVITY_LEVELS: [u8; CELL_COUNT] = [
    0, 1, 0, 2, 3, 0, 1,
    1, 2, 0, 0, 4, 3, 2,
    0, 3, 4, 2, 1, 0, 2,
    2, 0, 1, 3, 2, 4, 1,
    3, 4, 2, 0, 1, 3, 2,
    1, 0, 3, 4, 2, 1, 0,
    0, 2, 3, 1, 4, 2, 3,
    2, 3, 0, 2, 3, 4, 1,
    4, 1, 2, 3, 0, 2, 4,
    1, 0, 4, 2, 3, 1, 2,
    3, 2, 1, 4, 2, 0, 3,
    2, 4, 3, 1, 2, 4, 1,
    0, 1, 2, 4, 3, 2, 0,
    3, 2, 4, 1, 0, 3, 2,
    1, 4, 2, 3, 2, 4, 3,
];

fn level_color(level: u8, theme: &ThemeTokens) -> Rgba {
    match level {
        0 => theme.bg_card,
        1 => theme.cadence_level_1,
        2 => theme.cadence_level_2,
        3 => theme.cadence_level_3,
        _ => theme.cadence_level_4,
    }
}

fn render_day_cell(
    day_cell_id: usize,
    selected_day: Option<usize>,
    on_click_day: &ClickDayCallback,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let is_selected = selected_day == Some(day_cell_id);
    let border_color = if is_selected { theme.accent_brand } else { theme.border };
    let on_click = Rc::clone(on_click_day);
    div()
        .id(ElementId::NamedInteger(CELL_ID_PREFIX.into(), day_cell_id as u64))
        .w(theme.cadence_cell_size)
        .h(theme.cadence_cell_size)
        .rounded_sm()
        .bg(level_color(ACTIVITY_LEVELS[day_cell_id], theme))
        .border_1()
        .border_color(border_color)
        .cursor_pointer()
        .when(is_selected, |s| s.border_2().border_color(theme.text_white))
        .on_click(move |_, window, cx| {
            on_click(day_cell_id, window, cx);
        })
}

fn render_week_column(
    week_idx: usize,
    selected_day: Option<usize>,
    on_click_day: &ClickDayCallback,
    theme: &ThemeTokens,
) -> Div {
    let days = (0..DAYS_PER_WEEK).map(|day_idx| {
        render_day_cell(week_idx * DAYS_PER_WEEK + day_idx, selected_day, on_click_day, theme)
    });
    div().flex().flex_col().gap(px(CELL_GAP)).children(days)
}

/// Renders the clickable GitHub-style thought activity calendar heatmap.
pub fn render_clickable_heatmap(
    selected_day: Option<usize>,
    on_click_day: ClickDayCallback,
    theme: &ThemeTokens,
) -> impl IntoElement {
    let weeks: Vec<Div> = (0..WEEKS)
        .map(|week_idx| render_week_column(week_idx, selected_day, &on_click_day, theme))
        .collect();

    render_divided_section(theme)
        .child(render_heatmap_header(theme))
        .child(render_subtitle(selected_day, theme))
        .child(
            div()
                .w_full()
                .flex()
                .items_center()
                .gap_2()
                .child(render_day_labels(theme))
                .child(div().flex().gap(px(CELL_GAP)).children(weeks)),
        )
        .child(render_legend(theme))
}
