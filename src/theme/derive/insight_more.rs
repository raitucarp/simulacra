//! Palette-independent sizes of the three lens views of the Insights page and the composer's
//! readability meter (sb F3).

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Concept timeline lanes (height, label column, date axis, the gap between period cells,
/// the peak ring), the shortest drag that counts as a brush, the height of detail lists, and
/// the composer meter's bar.
pub(super) fn apply_insight_more_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        insight_lane_height: px(22.0), insight_lane_label_width: px(148.0), insight_axis_height: px(18.0),
        insight_brush_min_width: px(6.0), insight_cell_gap: px(1.0), insight_list_max_height: px(320.0),
        insight_meter_width: px(64.0), insight_meter_height: px(4.0), insight_peak_ring_width: px(1.5),
    });
}
