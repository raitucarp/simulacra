//! Palette-independent sizes of the Insights page (sb D2): chart plot heights, the canvases
//! of the network, embedding map, diagrams and radial charts, bar and axis geometry, node and
//! point radii, legend swatches, tooltips, ranked lists and the UpSet matrix.

use crate::theme::ThemeTokens;
use gpui_kit::px;

/// Every Insights size token.
pub(super) fn apply_insights_layout(t: &mut ThemeTokens) {
    set_tokens!(t {
        insights_page_max_width: px(1320.0), insights_tile_width: px(150.0), insights_chart_height: px(168.0),
        insights_spark_height: px(56.0), insights_axis_width: px(34.0), insights_bar_gap: px(2.0),
        insights_bar_max_width: px(30.0), insights_bar_min_height: px(2.0), insights_swatch_size: px(10.0),
        insights_tooltip_width: px(260.0), insights_radial_min: px(220.0), insights_radial_max: px(340.0),
        insights_canvas_height: px(520.0), insights_canvas_min_height: px(320.0), insights_side_width: px(300.0),
        insights_rank_label_width: px(168.0), insights_rank_value_width: px(64.0), insights_rank_bar_height: px(6.0),
        insights_node_radius_min: px(2.5), insights_node_radius_max: px(11.0), insights_point_radius: px(3.5),
        insights_edge_width: px(1.0), insights_label_max_width: px(150.0), insights_upset_cell: px(40.0),
        insights_upset_dot: px(9.0), insights_upset_label_width: px(140.0), insights_tree_height: px(220.0),
        insights_venn_height: px(300.0), insights_diagram_height: px(380.0), insights_filter_width: px(200.0),
    });
}
