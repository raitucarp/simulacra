//! The two column charts: items due in the next thirty days, and weekly retention against
//! the desired retention.

use super::super::super::panel::ReviewPanel;
use super::super::super::text::{percent, tr};
use super::super::common::{block_title, caption, surface};
use super::bars::BarChart;
use crate::domain::review::stats::{due_forecast, retention_by_week, FORECAST_DAYS, RETENTION_WEEKS};
use crate::i18n::t_count;
use crate::ui::features::day_filter::format_day;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

const FORECAST_ID: &str = "review-forecast";
const RETENTION_ID: &str = "review-retention";
/// A full retention bar.
const FULL_SHARE: f64 = 1.0;
const MIN_FORECAST_MAX: f64 = 1.0;

impl ReviewPanel {
    pub(super) fn forecast_panel(&self) -> Div {
        let theme = &self.theme;
        let counts = due_forecast(&self.data.items, self.data.today, FORECAST_DAYS);
        let total: usize = counts.iter().sum();
        let max = counts.iter().copied().max().unwrap_or_default() as f64;
        let tips = counts.iter().enumerate().map(|(day, count)| {
            let when = if day == 0 { tr("review.due.today") } else { t_count("review.due.in_days", day) };
            format!("{when}: {}", t_count("review.stats.items", *count))
        });
        let chart = BarChart {
            id: FORECAST_ID,
            values: counts.iter().map(|count| Some(*count as f64)).collect(),
            max: max.max(MIN_FORECAST_MAX),
            color: theme.accent_brand,
            tips: tips.collect(),
            axis: [tr("review.due.today"), t_count("review.due.in_days", FORECAST_DAYS / 2), t_count("review.due.in_days", FORECAST_DAYS - 1)],
            target: None,
            theme: theme.clone(),
        };
        let average = t!("review.stats.forecast_caption", total = total, average = format!("{:.1}", total as f64 / FORECAST_DAYS as f64)).to_string();
        surface(theme).child(block_title(LucideIcon::Calendar, theme.accent_brand, tr("review.stats.forecast"), theme)).child(chart).child(caption(average, theme))
    }

    pub(super) fn retention_panel(&self) -> Div {
        let theme = &self.theme;
        let weeks = retention_by_week(&self.data.log, self.data.today, RETENTION_WEEKS);
        let reviews: usize = weeks.iter().map(|week| week.reviews).sum();
        let recalled: f64 = weeks.iter().filter_map(|week| week.retention.map(|share| share * week.reviews as f64)).sum();
        let target = self.data.policy.desired_retention;
        let tips = weeks.iter().map(|week| {
            let share = week.retention.map_or_else(|| tr("review.none"), percent);
            t!("review.stats.week_tip", week = format_day(week.start), share = share, count = week.reviews).to_string()
        });
        let chart = BarChart {
            id: RETENTION_ID,
            values: weeks.iter().map(|week| week.retention).collect(),
            max: FULL_SHARE,
            color: theme.accent_success,
            tips: tips.collect(),
            axis: [t_count("review.stats.weeks_ago", RETENTION_WEEKS - 1), t_count("review.stats.weeks_ago", RETENTION_WEEKS / 2), tr("review.stats.this_week")],
            target: Some(target),
            theme: theme.clone(),
        };
        let summary = if reviews == 0 {
            tr("review.stats.retention_empty")
        } else {
            t!("review.stats.retention_caption", actual = percent(recalled / reviews as f64), target = percent(target), count = reviews).to_string()
        };
        surface(theme).child(block_title(LucideIcon::Target, theme.accent_success, tr("review.stats.retention"), theme)).child(chart).child(caption(summary, theme))
    }
}
