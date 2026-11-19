//! The review heatmap: one column per week (Monday on top), as many recent weeks as fit the
//! measured width, cells clamped to the token range; with the current streak.

use super::super::super::panel::ReviewPanel;
use super::super::super::text::tr;
use super::super::common::{block_title, caption, surface};
use crate::domain::review::stats::{current_streak, review_heatmap, HeatmapCell};
use crate::domain::review::ReviewEntry;
use crate::i18n::t_count;
use crate::theme::ThemeTokens;
use crate::ui::components::measured_width;
use crate::ui::features::day_filter::format_day;
use crate::ui::icons::LucideIcon;
use chrono::NaiveDate;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::*;

const GRID_ID: &str = "review-heatmap";
const CELL_ID_PREFIX: &str = "review-heat-";
/// Most weeks shown (a year).
const MAX_WEEKS: usize = 53;
const DAYS_PER_WEEK: usize = 7;

/// Cell color of a level: the empty tone, or the brand accent at the level's opacity.
fn level_color(level: u8, theme: &ThemeTokens) -> Rgba {
    let opacity = match level {
        0 => return theme.border_subtle,
        1 => theme.cadence_alpha_1,
        2 => theme.cadence_alpha_2,
        3 => theme.cadence_alpha_3,
        _ => theme.cadence_alpha_4,
    };
    Rgba { a: theme.accent_brand.a * opacity, ..theme.accent_brand }
}

#[derive(IntoElement)]
struct ReviewHeatmap {
    log: Vec<ReviewEntry>,
    today: NaiveDate,
    theme: ThemeTokens,
}

impl ReviewHeatmap {
    fn cell(&self, index: usize, cell: HeatmapCell, size: Pixels) -> Stateful<Div> {
        let tip = format!("{}: {}", format_day(cell.day), t_count("review.stats.reviews", cell.count));
        let fill = if cell.future { self.theme.bg_transparent } else { level_color(cell.level, &self.theme) };
        div()
            .id(ElementId::NamedInteger(CELL_ID_PREFIX.into(), index as u64))
            .size(size)
            .rounded(self.theme.heatmap_cell_radius)
            .bg(fill)
            .tooltip(move |window, cx| Tooltip::new(tip.clone()).build(window, cx))
    }
}

impl RenderOnce for ReviewHeatmap {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (measured, probe) = measured_width(GRID_ID, window, cx);
        let theme = self.theme.clone();
        let (gap, min, max) = (theme.review_heatmap_gap, theme.review_heatmap_cell_min, theme.review_heatmap_cell_max);
        let width = measured.unwrap_or(theme.review_card_max_width);
        let weeks = (((width + gap) / (min + gap)).floor() as usize).clamp(1, MAX_WEEKS);
        let size = ((width - gap * (weeks as f32 - 1.0)) / weeks as f32).clamp(min, max);
        let columns = review_heatmap(&self.log, self.today, weeks).into_iter().enumerate().map(|(week, days)| {
            let cells = days.into_iter().enumerate().map(|(day, cell)| self.cell(week * DAYS_PER_WEEK + day, cell, size));
            div().flex().flex_col().gap(gap).children(cells)
        });
        let columns: Vec<Div> = columns.collect();
        div().relative().w_full().flex().justify_end().gap(gap).child(probe).children(columns)
    }
}

impl ReviewPanel {
    pub(super) fn heatmap_panel(&self) -> Div {
        let theme = &self.theme;
        let streak = current_streak(&self.data.log, self.data.today);
        let heatmap = ReviewHeatmap { log: self.data.log.clone(), today: self.data.today, theme: theme.clone() };
        surface(theme)
            .child(div().flex().items_center().justify_between().child(block_title(LucideIcon::Flame, theme.accent_brand, tr("review.stats.heatmap"), theme)).child(caption(t_count("review.stats.streak", streak), theme)))
            .child(heatmap)
            .child(caption(t_count("review.stats.total_reviews", self.data.log.len()), theme))
    }
}
