//! Secondary right sidebar building blocks (mini mind graph, cadence heatmap, spark feed),
//! re-exported under the legacy `ui::organisms::secondary_sidebar` path.

pub use super::dot_graph::{render_mini_graph, OpenGraphCallback};
pub use super::heatmap::{render_clickable_heatmap, ClickDayCallback, HeatmapParams};
pub use super::spark_feed::{
    render_random_spark_feed, render_random_spark_feed_with_lookup, MAX_SPARK_FEED_ITEMS,
};
