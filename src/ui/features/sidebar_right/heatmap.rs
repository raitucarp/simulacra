//! Cadence heatmap of the active network: the most recent weeks that fit the sidebar (up to
//! a year), Sunday-first rows, square cells sized from the width. Hovering a day shows
//! "N posts on <date>"; clicking it filters the feed.

use std::rc::Rc;

use super::heatmap_chrome::{render_heatmap_header, render_legend, render_subtitle};
use super::heatmap_grid::CadenceGrid;
use super::section::render_divided_section;
use crate::domain::activity::ActivityHeatmap;
use crate::theme::ThemeTokens;
use jiff::civil::Date;
use gpui_kit::*;

/// Callback fired with the clicked calendar day.
pub type ClickDayCallback = Rc<dyn Fn(Date, &mut Window, &mut App) + 'static>;

/// Inputs of the cadence heatmap section.
pub struct HeatmapParams<'a> {
    pub heatmap: &'a ActivityHeatmap,
    /// Day the feed is currently filtered to, highlighted in the grid.
    pub selected: Option<Date>,
    pub on_click_day: ClickDayCallback,
}

/// Renders the clickable cadence heatmap section.
pub fn render_clickable_heatmap(params: HeatmapParams<'_>, theme: &ThemeTokens) -> impl IntoElement {
    let heatmap = params.heatmap;
    let grid = CadenceGrid {
        heatmap: heatmap.clone(),
        selected: params.selected,
        on_click_day: params.on_click_day,
        theme: theme.clone(),
    };
    render_divided_section(theme)
        .child(render_heatmap_header(heatmap.total(), theme))
        .child(render_subtitle(heatmap, params.selected, theme))
        .child(grid)
        .child(render_legend(theme))
}
