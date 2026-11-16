//! Localized calendar labels for the cadence heatmap, and the dismissible
//! "Showing <date> ×" chip shown above the feed while it is narrowed to one day.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use chrono::{Datelike, NaiveDate};
use gpui_kit::component::Icon;
use gpui_kit::*;
use crate::i18n::plural_key;
use rust_i18n::t;

/// Translation keys of the abbreviated month names, January first.
const MONTH_KEYS: [&str; 12] = [
    "cadence.month_1", "cadence.month_2", "cadence.month_3", "cadence.month_4",
    "cadence.month_5", "cadence.month_6", "cadence.month_7", "cadence.month_8",
    "cadence.month_9", "cadence.month_10", "cadence.month_11", "cadence.month_12",
];

const CHIP_ID: &str = "feed-day-filter-chip";
const CLEAR_ID: &str = "feed-day-filter-clear";
/// Count-aware "N posts on <date>" label (`_one` / `_other` variants).
const DAY_POSTS_KEY: &str = "cadence.day_posts";

/// Callback clearing the day filter.
pub type ClearDayFilterCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Abbreviated, localized name of month `month` (`1..=12`).
pub fn month_label(month: u32) -> String {
    let idx = (month as usize).saturating_sub(1).min(MONTH_KEYS.len() - 1);
    t!(MONTH_KEYS[idx]).to_string()
}

/// Localized short date, e.g. "Nov 16, 2026".
pub fn format_day(date: NaiveDate) -> String {
    t!("cadence.date", month = month_label(date.month()), day = date.day(), year = date.year()).to_string()
}

/// "N posts on <date>" summary for a heatmap day.
pub fn day_summary(date: NaiveDate, count: u32) -> String {
    let date = format_day(date);
    if count == 0 {
        return t!("cadence.day_none", date = date).to_string();
    }
    let count = count as usize;
    let key = plural_key(DAY_POSTS_KEY, count);
    t!(key.as_str(), count = count, date = date).to_string()
}

/// Dismissible chip announcing the active day filter.
pub fn render_day_filter_chip(day: NaiveDate, on_clear: ClearDayFilterCallback, theme: &ThemeTokens) -> impl IntoElement {
    let clear = div()
        .id(CLEAR_ID)
        .w(theme.spacing_lg)
        .h(theme.spacing_lg)
        .flex()
        .items_center()
        .justify_center()
        .rounded(theme.radius_full)
        .cursor_pointer()
        .text_color(theme.text_muted)
        .hover(|s| s.bg(theme.hover_bg).text_color(theme.text_primary))
        .on_click(move |_, window, cx| on_clear(window, cx))
        .child(Icon::new(LucideIcon::X));
    div().w_full().px(theme.spacing_lg).py(theme.spacing_sm).flex().child(
        div()
            .id(CHIP_ID)
            .flex()
            .items_center()
            .gap(theme.spacing_sm)
            .pl(theme.spacing_md)
            .pr(theme.spacing_xs)
            .py(theme.spacing_2xs)
            .rounded(theme.radius_full)
            .border(theme.border_width_hairline)
            .border_color(theme.accent_success_border)
            .bg(theme.accent_success_subtle)
            .font_family(theme.font_family_ui.clone())
            .text_size(theme.font_size_xs)
            .text_color(theme.text_primary)
            .child(div().w(theme.spacing_md).h(theme.spacing_md).text_color(theme.accent_success).child(Icon::new(LucideIcon::Calendar)))
            .child(t!("cadence.showing", date = format_day(day)).to_string())
            .child(clear),
    )
}
