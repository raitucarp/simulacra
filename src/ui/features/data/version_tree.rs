//! The graph lanes beside the history sheet's versions (V5): one column per lane, a line
//! where a line of versions runs, a dot on the row's own version (the current one in the
//! brand colour) and a connector where a branch joins the version it was edited from.

use crate::domain::data::history::{lane_cell, LaneCell, LaneRow};
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// A line segment: drawn in `colour` when `on`, else an empty spacer of the same size.
fn segment(on: bool, colour: Rgba) -> Div {
    div().when(on, |s| s.bg(colour))
}

/// One lane of one row.
fn lane(cell: LaneCell, current: bool, theme: &ThemeTokens) -> Div {
    let line = theme.border;
    let dot_colour = if current { theme.accent_brand } else { theme.text_muted };
    let stroke = theme.border_width_threadline;
    let vertical = |on: bool| segment(on, line).flex_1().w(stroke);
    let horizontal = |on: bool| segment(on, line).flex_1().h(stroke);
    let middle = if cell.dot {
        div().flex_none().size(theme.history_lane_dot).rounded(theme.radius_full).bg(dot_colour)
    } else {
        segment(cell.up && cell.down, line).flex_none().w(stroke).h(theme.history_lane_dot)
    };
    let cross = div().w_full().flex_none().flex().items_center().child(horizontal(cell.left)).child(middle).child(horizontal(cell.right));
    div().w(theme.history_lane_width).flex_none().flex().flex_col().items_center().child(vertical(cell.up)).child(cross).child(vertical(cell.down))
}

/// The lanes of `row`, `count` columns wide.
pub(super) fn render_lanes(row: &LaneRow, count: usize, current: bool, theme: &ThemeTokens) -> Div {
    div().flex_none().flex().children((0..count).map(|at| lane(lane_cell(row, at), current && at == row.lane, theme)))
}
