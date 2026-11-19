//! The cadence grid itself: square day cells sized from the measured width (see
//! `heatmap_geometry`), weekday labels on their rows and non-overlapping month labels.

use super::heatmap::ClickDayCallback;
use super::heatmap_chrome::level_color;
use super::heatmap_geometry::{GridGeometry, PlacedLabel};
use crate::domain::activity::{ActivityHeatmap, HeatmapDay, DAYS_PER_WEEK, HEATMAP_WEEKS};
use crate::theme::ThemeTokens;
use crate::ui::components::measured_width;
use crate::ui::features::day_filter::{day_summary, month_label};
use jiff::civil::Date;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const GRID_ID: &str = "cadence-grid";
const CELL_ID_PREFIX: &str = "cal-cell";
/// Weekday rows (Sunday = 0) that carry a label, with their translation keys.
const LABELED_ROWS: [(usize, &str); 3] =
    [(1, "cadence.weekday_mon"), (3, "cadence.weekday_wed"), (5, "cadence.weekday_fri")];

/// The clickable grid of one year of days.
#[derive(IntoElement)]
pub(super) struct CadenceGrid {
    pub heatmap: ActivityHeatmap,
    pub selected: Option<Date>,
    pub on_click_day: ClickDayCallback,
    pub theme: ThemeTokens,
}

/// A small muted label, one line high, vertically centered in a `height` slot.
fn small_label(text: String, height: Pixels, theme: &ThemeTokens) -> Div {
    div()
        .h(height)
        .flex()
        .items_center()
        .overflow_hidden()
        .whitespace_nowrap()
        .text_size(theme.font_size_2xs)
        .line_height(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .child(text)
}

impl CadenceGrid {
    fn cell(&self, day: HeatmapDay, geometry: GridGeometry) -> Stateful<Div> {
        let theme = &self.theme;
        let fill = if self.selected == Some(day.date) { theme.text_primary } else { level_color(day.level, theme) };
        let on_click = Rc::clone(&self.on_click_day);
        let HeatmapDay { date, count, .. } = day;
        let index = self.heatmap.index_of(date).unwrap_or_default();
        div()
            .id(ElementId::NamedInteger(CELL_ID_PREFIX.into(), index as u64))
            .size(geometry.cell)
            .flex_none()
            .rounded(theme.heatmap_cell_radius)
            .bg(fill)
            .cursor_pointer()
            .hover(|s| s.bg(theme.text_secondary))
            .tooltip(move |window, cx| Tooltip::new(day_summary(date, count)).build(window, cx))
            .on_click(move |_, window, cx| on_click(date, window, cx))
    }

    /// One weekday row: its label, then that weekday of every shown week.
    fn row(&self, row: usize, geometry: GridGeometry) -> Div {
        let theme = &self.theme;
        let label = LABELED_ROWS.iter().find(|(r, _)| *r == row).map(|(_, key)| t!(*key).to_string());
        let cells = (geometry.first_week()..HEATMAP_WEEKS).map(|week| match self.heatmap.day(week, row) {
            Some(day) => self.cell(day, geometry).into_any_element(),
            None => div().size(geometry.cell).flex_none().into_any_element(),
        });
        div()
            .flex()
            .child(div().flex_none().w(theme.heatmap_label_width).when_some(label, |slot, text| slot.child(small_label(text, geometry.cell, theme))))
            .child(div().flex().gap(geometry.gap).children(cells))
    }

    /// Month names above the first column of their month.
    fn month_row(&self, labels: Vec<PlacedLabel>, geometry: GridGeometry) -> Div {
        let theme = &self.theme;
        let placed = labels.into_iter().map(|label| {
            let left = geometry.step() * label.column as f32;
            div().absolute().top_0().left(left).w(label.width).child(small_label(month_label(label.month), theme.heatmap_month_row_height, theme))
        });
        div()
            .flex()
            .h(theme.heatmap_month_row_height)
            .child(div().flex_none().w(theme.heatmap_label_width))
            .child(div().flex_1().relative().children(placed))
    }
}

impl RenderOnce for CadenceGrid {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (measured, probe) = measured_width(GRID_ID, window, cx);
        let theme = &self.theme;
        let width = measured.unwrap_or(theme.right_sidebar_min) - theme.heatmap_label_width;
        let geometry = GridGeometry::fit(width, theme.heatmap_cell_gap, (theme.heatmap_cell_min, theme.heatmap_cell_max));
        let labels = geometry.place_labels(&self.heatmap.month_labels(), theme.heatmap_month_label_width);
        let rows: Vec<Div> = (0..DAYS_PER_WEEK).map(|row| self.row(row, geometry)).collect();
        div()
            .relative()
            .w_full()
            .flex()
            .flex_col()
            .gap(geometry.gap)
            .child(probe)
            .child(self.month_row(labels, geometry))
            .children(rows)
    }
}
