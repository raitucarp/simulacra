//! Palette-independent sizes of the thinking tools (sb E1).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Argument canvas chips, rows, edges and the statement list's role column; the credence
/// popover; the reliability diagram and its dots; the atomicity sheet and its score ring.
pub(super) fn apply_thinking_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        thinking_chip_width: px(176.0), thinking_chip_height: px(60.0), thinking_row_height: px(84.0), thinking_edge_width: px(1.5),
        thinking_handle_width: px(88.0), credence_popover_width: px(300.0),
        calibration_chart_size: px(360.0), calibration_dot_min: px(3.5), calibration_dot_max: px(9.0),
        atomic_sheet_width: px(560.0), atomic_sheet_max_height: px(600.0), atomic_score_size: px(56.0),
    });
}
