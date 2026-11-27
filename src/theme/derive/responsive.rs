//! Palette-independent sizes of the responsive layout: breakpoints, the narrowed and drawer
//! right sidebar, cadence heatmap cell bounds and Mind Graph label and mini-graph metrics.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Window-width breakpoints and the sizes that change with them.
pub(super) fn apply_responsive_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        // Below the medium width the layout is compact; from the wide width on it is wide
        layout_medium_min_width: px(900.0), layout_wide_min_width: px(1280.0),
        // The right panel opens by itself only from this width on (four columns need the room)
        right_panel_auto_min_width: px(1600.0),
        right_sidebar_medium_width: px(272.0), right_sidebar_medium_max: px(340.0),
        right_drawer_width: px(320.0),
        inspector_stacked_height: px(220.0), inspector_stacked_min: px(120.0), inspector_stacked_max: px(480.0),
        heatmap_cell_min: px(10.0), heatmap_cell_max: px(16.0), heatmap_month_label_width: px(26.0),
        graph_label_max_width: px(200.0), graph_mini_node_radius_max: px(6.0),
        graph_mini_aspect: 1.6,
    });
}
