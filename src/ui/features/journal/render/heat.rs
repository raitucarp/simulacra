//! The heat strip of written days (words per day, week columns with Monday on top) and the
//! streak tiles beside it, worded as runs to enjoy rather than chains to keep.

use super::super::panel::JournalPanel;
use super::super::text::{day_label, tr};
use super::common::{block_title, caption, count_tile, surface};
use crate::domain::journal::calendar::{heat_strip, words_by_day};
use crate::domain::journal::streak::streaks;
use crate::domain::review::stats::HeatmapCell;
use crate::theme::color::with_alpha;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::*;
use rust_i18n::t;

const CELL_ID: &str = "journal-heat-cell";
/// Weeks shown in the compact and the wider layouts.
const WEEKS_COMPACT: usize = 12;
const WEEKS_WIDE: usize = 26;
const DAYS_PER_WEEK: usize = 7;

/// A cell's fill: empty days in the track colour, written days in the accent by level.
pub fn heat_colour(cell: &HeatmapCell, accent: Rgba, theme: &ThemeTokens) -> Rgba {
    let alphas = [theme.cadence_alpha_1, theme.cadence_alpha_2, theme.cadence_alpha_3, theme.cadence_alpha_4];
    match (cell.future, cell.level) {
        (true, _) => theme.bg_transparent,
        (_, 0) => theme.border_subtle,
        (_, level) => with_alpha(accent, alphas[(level as usize - 1).min(alphas.len() - 1)]),
    }
}

/// Week columns of square cells, each with a tooltip.
pub fn heat_columns(id: &'static str, weeks: Vec<Vec<HeatmapCell>>, fill: impl Fn(&HeatmapCell) -> Rgba, tip: impl Fn(&HeatmapCell) -> String, theme: &ThemeTokens) -> Div {
    let columns = weeks.into_iter().enumerate().map(|(w, week)| {
        let cells = week.into_iter().enumerate().map(|(d, cell)| {
            let text = tip(&cell);
            div()
                .id((id, w * DAYS_PER_WEEK + d))
                .size(theme.journal_heat_cell)
                .rounded(theme.radius_xs)
                .bg(fill(&cell))
                .tooltip(move |window, cx| Tooltip::new(text.clone()).build(window, cx))
        });
        div().flex().flex_col().gap(theme.journal_heat_gap).children(cells)
    });
    div().flex().gap(theme.journal_heat_gap).children(columns)
}

impl JournalPanel {
    pub(super) fn render_heat(&self, class: LayoutClass) -> Div {
        let theme = &self.theme;
        let weeks = if class.is_compact() { WEEKS_COMPACT } else { WEEKS_WIDE };
        let strip = heat_strip(&words_by_day(&self.data.book.entries), self.data.today, weeks);
        let tip = |cell: &HeatmapCell| t!("journal.heat.tip", day = day_label(cell.day), count = cell.count).to_string();
        let runs = streaks(self.data.book.written_days(), self.data.today);
        let tiles = div()
            .flex()
            .flex_wrap()
            .gap(theme.control_gap)
            .child(count_tile(runs.days.to_string(), tr("journal.streak.days"), theme.accent_brand, theme))
            .child(count_tile(runs.current.to_string(), tr("journal.streak.run"), theme.accent_success, theme))
            .child(count_tile(runs.longest.to_string(), tr("journal.streak.longest"), theme.accent_dialectic, theme));
        surface(theme)
            .child(block_title(LucideIcon::Calendar, theme.accent_brand, tr("journal.heat.title"), theme))
            .child(div().w_full().overflow_hidden().child(heat_columns(CELL_ID, strip, |cell| heat_colour(cell, theme.accent_brand, theme), tip, theme)))
            .child(tiles)
            .child(caption(tr("journal.streak.gentle"), theme))
    }
}
