//! Secondary right sidebar building blocks (dot mind graph, cadence heatmap, spark feed),
//! re-exported under the legacy `ui::organisms::secondary_sidebar` path.

pub use super::dot_graph::{
    render_obsidian_dot_graph, CANVAS_HEIGHT, CANVAS_PADDING, CANVAS_WIDTH, GOLDEN_ANGLE_RADIANS,
    MAX_GRAPH_SCAN_WINDOW, MAX_OBSIDIAN_GRAPH_NODES, NODE_SIZE_AGENT, NODE_SIZE_NEUTRAL,
    NODE_SIZE_USER,
};
pub use super::heatmap::{render_clickable_heatmap, ClickDayCallback};
pub use super::spark_feed::{
    render_random_spark_feed, render_random_spark_feed_with_lookup, MAX_SPARK_FEED_ITEMS,
};
