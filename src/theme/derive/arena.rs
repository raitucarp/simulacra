//! Palette-independent sizes of the Arena page and the profile's Beliefs tab (sb F1): the
//! page measure and its setup column, rubric bars, the lab grid's columns, saved lists, the
//! belief sparkline (size, stroke, end dot) and credence bars.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Every Arena and Beliefs size token.
pub(super) fn apply_arena_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        arena_page_max_width: px(1180.0), arena_side_width: px(440.0), arena_score_label_width: px(112.0),
        arena_score_bar_width: px(132.0), arena_bar_height: px(6.0), arena_grid_label_width: px(210.0),
        arena_grid_cell_width: px(190.0), arena_saved_max_height: px(300.0), arena_row_label_width: px(96.0),
        belief_spark_width: px(112.0), belief_spark_height: px(28.0), belief_spark_stroke: px(1.5),
        belief_spark_dot: px(2.5), belief_credence_width: px(88.0),
    });
}
