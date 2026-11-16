//! GitHub-style cadence heatmap of the active network: 53 week columns ending today,
//! Sunday-first rows. Hovering a day shows "N posts on <date>"; clicking it filters the feed.

use std::rc::Rc;

use super::heatmap_chrome::{
    level_color, render_heatmap_header, render_legend, render_month_row, render_subtitle, render_weekday_label,
};
use super::section::render_divided_section;
use crate::domain::activity::{ActivityHeatmap, HeatmapDay, DAYS_PER_WEEK, HEATMAP_WEEKS};
use crate::theme::ThemeTokens;
use crate::ui::features::day_filter::day_summary;
use chrono::NaiveDate;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::*;

/// Callback fired with the clicked calendar day.
pub type ClickDayCallback = Rc<dyn Fn(NaiveDate, &mut Window, &mut App) + 'static>;

const CELL_ID_PREFIX: &str = "cal-cell";

/// Inputs of the cadence heatmap section.
pub struct HeatmapParams<'a> {
    pub heatmap: &'a ActivityHeatmap,
    /// Day the feed is currently filtered to, highlighted in the grid.
    pub selected: Option<NaiveDate>,
    pub on_click_day: ClickDayCallback,
}

/// Unstyled slot sized like one day cell (flexible width, token height).
pub(super) fn cell_slot(theme: &ThemeTokens) -> Div {
    div().flex_1().min_w_0().h(theme.heatmap_cell_height).rounded(theme.heatmap_cell_radius)
}

fn render_cell(day: HeatmapDay, idx: usize, params: &HeatmapParams<'_>, theme: &ThemeTokens) -> Stateful<Div> {
    let is_selected = params.selected == Some(day.date);
    let fill = if is_selected { theme.text_primary } else { level_color(day.level, theme) };
    let on_click = Rc::clone(&params.on_click_day);
    let HeatmapDay { date, count, .. } = day;
    cell_slot(theme)
        .id(ElementId::NamedInteger(CELL_ID_PREFIX.into(), idx as u64))
        .bg(fill)
        .cursor_pointer()
        .hover(|s| s.bg(theme.text_secondary))
        .tooltip(move |window, cx| Tooltip::new(day_summary(date, count)).build(window, cx))
        .on_click(move |_, window, cx| on_click(date, window, cx))
}

/// One weekday row: its label, then that weekday of every week (future days left blank).
fn render_row(row: usize, params: &HeatmapParams<'_>, theme: &ThemeTokens) -> Div {
    let cells = (0..HEATMAP_WEEKS).map(|week| match params.heatmap.day(week, row) {
        Some(day) => render_cell(day, week * DAYS_PER_WEEK + row, params, theme).into_any_element(),
        None => cell_slot(theme).into_any_element(),
    });
    div()
        .w_full()
        .flex()
        .items_center()
        .child(render_weekday_label(row, theme))
        .child(div().flex_1().min_w_0().flex().gap(theme.heatmap_cell_gap).children(cells))
}

/// Renders the clickable cadence heatmap section.
pub fn render_clickable_heatmap(params: HeatmapParams<'_>, theme: &ThemeTokens) -> impl IntoElement {
    let heatmap = params.heatmap;
    let rows = (0..DAYS_PER_WEEK).map(|row| render_row(row, &params, theme));
    let grid = div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.heatmap_cell_gap)
        .child(render_month_row(heatmap, theme))
        .children(rows);
    render_divided_section(theme)
        .child(render_heatmap_header(heatmap.total(), theme))
        .child(render_subtitle(heatmap, params.selected, theme))
        .child(grid)
        .child(render_legend(theme))
}
