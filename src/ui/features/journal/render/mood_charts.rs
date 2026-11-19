//! Mood over time: a heat strip of each day's mean mood (pleasant days in the success
//! accent, unpleasant ones in the danger accent, stronger with the feeling), and columns of
//! mean energy by weekday and of check-ins by hour (the Insights chart kit).

use super::super::panel::JournalPanel;
use super::super::text::{day_label, tr};
use super::common::{block_title, caption, surface};
use super::heat::heat_columns;
use crate::domain::journal::mood::summary::{by_hour, by_weekday, daily_means, MoodMean};
use crate::domain::review::stats::{heatmap_level, HeatmapCell, HEATMAP_LEVELS};
use crate::theme::color::with_alpha;
use crate::ui::features::insights::chart::columns::{Column, ColumnChart};
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use jiff::civil::Date;
use gpui_kit::*;
use rust_i18n::t;
use std::collections::BTreeMap;

const HEAT_ID: &str = "journal-mood-heat";
const WEEKDAY_ID: &str = "journal-mood-weekday";
const HOUR_ID: &str = "journal-mood-hour";
const WEEKS_COMPACT: usize = 12;
const WEEKS_WIDE: usize = 26;
/// Valence magnitude mapped onto the heat levels.
const LEVEL_SCALE: f32 = 100.0;

impl JournalPanel {
    pub(super) fn render_mood_charts(&self, class: LayoutClass) -> Div {
        let theme = &self.theme;
        let checkins = &self.data.book.checkins;
        let weeks = if class.is_compact() { WEEKS_COMPACT } else { WEEKS_WIDE };
        let means = daily_means(checkins);
        let counts: BTreeMap<Date, usize> = means.iter().map(|(d, m)| (*d, m.count)).collect();
        let strip = crate::domain::journal::calendar::heat_strip(&counts, self.data.today, weeks);
        let fill = |cell: &HeatmapCell| self.mood_fill(cell, &means);
        let tip = |cell: &HeatmapCell| match means.get(&cell.day) {
            Some(m) => t!("journal.mood.day_tip", day = day_label(cell.day), mood = format!("{:+.2}", m.valence), energy = format!("{:.1}", m.energy), count = m.count).to_string(),
            None => day_label(cell.day),
        };
        surface(theme)
            .child(block_title(LucideIcon::Activity, theme.accent_brand, tr("journal.mood.over_time"), theme))
            .child(div().w_full().overflow_hidden().child(heat_columns(HEAT_ID, strip, fill, tip, theme)))
            .child(caption(tr("journal.mood.heat_hint"), theme))
            .child(block_title(LucideIcon::Calendar, theme.accent_success, tr("journal.mood.by_weekday"), theme))
            .child(self.weekday_chart(&by_weekday(checkins)))
            .child(block_title(LucideIcon::Clock, theme.info, tr("journal.mood.by_hour"), theme))
            .child(self.hour_chart(&by_hour(checkins)))
    }

    fn mood_fill(&self, cell: &HeatmapCell, means: &BTreeMap<Date, MoodMean>) -> Rgba {
        let theme = &self.theme;
        let Some(mean) = means.get(&cell.day).filter(|_| !cell.future) else { return super::heat::heat_colour(&HeatmapCell { level: 0, ..*cell }, theme.accent_brand, theme) };
        let level = heatmap_level((mean.valence.abs() * LEVEL_SCALE) as usize, LEVEL_SCALE as usize).clamp(1, HEATMAP_LEVELS);
        let alphas = [theme.cadence_alpha_1, theme.cadence_alpha_2, theme.cadence_alpha_3, theme.cadence_alpha_4];
        let accent = if mean.valence >= 0.0 { theme.accent_success } else { theme.accent_danger };
        with_alpha(accent, alphas[level as usize - 1])
    }

    fn weekday_chart(&self, means: &[MoodMean]) -> ColumnChart {
        let theme = &self.theme;
        let columns = means.iter().enumerate().map(|(i, m)| Column {
            segments: vec![(f64::from(m.energy), theme.accent_success)],
            tip: t!("journal.mood.weekday_tip", day = tr(&format!("journal.weekday.{i}")), energy = format!("{:.1}", m.energy), count = m.count).to_string(),
        });
        let labels = (0..means.len()).map(|i| tr(&format!("journal.weekday_short.{i}"))).collect();
        ColumnChart { id: WEEKDAY_ID.into(), columns: columns.collect(), labels, height: theme.journal_chart_height, on_click: None, theme: theme.clone() }
    }

    fn hour_chart(&self, means: &[MoodMean]) -> ColumnChart {
        let theme = &self.theme;
        let columns = means.iter().enumerate().map(|(hour, m)| Column {
            segments: vec![(m.count as f64, theme.info)],
            tip: t!("journal.mood.hour_tip", hour = format!("{hour:02}:00"), energy = format!("{:.1}", m.energy), count = m.count).to_string(),
        });
        let labels = (0..means.len()).map(|hour| format!("{hour:02}")).collect();
        ColumnChart { id: HOUR_ID.into(), columns: columns.collect(), labels, height: theme.journal_chart_height, on_click: None, theme: theme.clone() }
    }
}
