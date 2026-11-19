//! "Their week": seven day rows of 24 hour cells, replayed through the real scheduler
//! (`simulate_week`). Hours with likely posts are violet (deeper for more posts), online hours
//! without one are tinted, offline hours (and whole days off) stay dark; each row ends with
//! the day's count.

use super::{week_summary, WeekSummary};
use crate::agent::schedule::{simulate_week, HourWindow, WeekPreview, DAYS_PER_WEEK, HEARTBEAT_INTERVAL, HOURS_PER_DAY};
use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::Weekday;
use crate::domain::ProfileId;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{tr, tr_count};
use gpui_kit::*;

/// Day-name keys, Monday first (`forge.ui.daily.day_*`).
const DAY_KEYS: [&str; DAYS_PER_WEEK] = [
    "forge.ui.daily.day_mon", "forge.ui.daily.day_tue", "forge.ui.daily.day_wed", "forge.ui.daily.day_thu",
    "forge.ui.daily.day_fri", "forge.ui.daily.day_sat", "forge.ui.daily.day_sun",
];
const OFF_KEY: &str = "forge.ui.daily.week_off";
const MANUAL_KEY: &str = "forge.ui.daily.week_manual";
const POSTS_KEY: &str = "forge.ui.daily.week_posts";
const LEGEND_KEY: &str = "forge.ui.daily.week_legend";
/// Seed of the preview's spontaneity rolls (any fixed value keeps the preview stable).
const PREVIEW_SEED: u64 = 0x5EED;

/// The simulated week of `cfg` for the agent with `id`.
fn preview_week(cfg: &AgentConfig, id: &ProfileId) -> WeekPreview {
    let tick = jiff::SignedDuration::try_from(HEARTBEAT_INTERVAL).unwrap_or_default();
    simulate_week(cfg, id, tick, PREVIEW_SEED)
}

/// Opacity of a cell with `posts` posts when the busiest hour has `busiest`.
fn post_color(posts: u16, busiest: u16, theme: &ThemeTokens) -> Rgba {
    let alphas = [theme.cadence_alpha_1, theme.cadence_alpha_2, theme.cadence_alpha_3, theme.cadence_alpha_4];
    let level = (usize::from(posts) * alphas.len()).div_ceil(usize::from(busiest.max(1))).clamp(1, alphas.len());
    Rgba { a: theme.accent_brand.a * alphas[level - 1], ..theme.accent_brand }
}

fn day_row(day: usize, week: &WeekPreview, online: Option<HourWindow>, theme: &ThemeTokens) -> Div {
    let busiest = week.busiest_hour();
    let cells = (0..HOURS_PER_DAY).map(|hour| {
        let posts = week.posts[day][usize::from(hour)];
        let color = match (posts, online.is_some_and(|window| window.contains(hour))) {
            (0, true) => theme.accent_brand_subtle,
            (0, false) => theme.bg_elevated,
            (n, _) => post_color(n, busiest, theme),
        };
        div().flex_1().h(theme.forge_week_cell_height).rounded(theme.radius_xs).bg(color)
    });
    let caption = |text: String| div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(text);
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .child(div().w(theme.heatmap_label_width).flex_shrink_0().child(caption(tr(DAY_KEYS[day]))))
        .child(div().flex_1().flex().gap(theme.spacing_3xs).children(cells))
        .child(div().w(theme.heatmap_label_width).flex_shrink_0().flex().justify_end().child(caption(week.day_total(day).to_string())))
}

/// The week strip for `cfg`, with its one-line summary.
pub(super) fn week_strip(cfg: &AgentConfig, id: &ProfileId, theme: &ThemeTokens) -> Div {
    let week = preview_week(cfg, id);
    let online = HourWindow::new(cfg.active_hours_start, cfg.active_hours_end);
    let summary = match week_summary(cfg, &week) {
        WeekSummary::Off => tr(OFF_KEY),
        WeekSummary::Manual => tr(MANUAL_KEY),
        WeekSummary::Posts(count) => tr_count(POSTS_KEY, count as usize, &[]),
    };
    let on_day = |day: usize| Weekday::ALL.get(day).is_some_and(|weekday| cfg.active_days.contains(*weekday));
    let rows: Vec<Div> = (0..DAYS_PER_WEEK).map(|day| day_row(day, &week, on_day(day).then_some(online), theme)).collect();
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(summary))
        .children(rows)
        .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr(LEGEND_KEY)))
}
