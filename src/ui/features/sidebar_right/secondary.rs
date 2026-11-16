//! Secondary right sidebar featuring:
//! 1. Vector-sorted Dot Mind Graph with relational edges and no text labels
//! 2. Interactive Clickable Thought Cadence Heatmap Calendar
//! 3. Scrollable Random Sparks Discovery Feed
//!
//! Also re-exports the panel building blocks under the legacy
//! `ui::organisms::secondary_sidebar` path.

use std::rc::Rc;

pub use super::dot_graph::{
    render_obsidian_dot_graph, CANVAS_HEIGHT, CANVAS_PADDING, CANVAS_WIDTH, GOLDEN_ANGLE_RADIANS,
    MAX_GRAPH_SCAN_WINDOW, MAX_OBSIDIAN_GRAPH_NODES, NODE_SIZE_AGENT, NODE_SIZE_NEUTRAL,
    NODE_SIZE_USER,
};
pub use super::heatmap::{render_clickable_heatmap, ClickDayCallback};
pub use super::spark_feed::{
    render_random_spark_feed, render_random_spark_feed_with_lookup, MAX_SPARK_FEED_ITEMS,
};

use crate::domain::{GroundingDocument, Post};
use crate::theme::ThemeTokens;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;

const SECONDARY_SIDEBAR_ID: &str = "secondary-right-sidebar";

/// Renders the complete Secondary Right Sidebar.
pub fn render_secondary_sidebar(
    posts: &[Post],
    docs: &[GroundingDocument],
    selected_day: Option<usize>,
    on_click_day: impl Fn(usize, &mut Window, &mut App) + 'static,
    theme: &ThemeTokens,
) -> impl IntoElement {
    let on_click = Rc::new(on_click_day);

    div()
        .id(SECONDARY_SIDEBAR_ID)
        .w_full()
        .h_full()
        .overflow_y_scrollbar()
        .bg(theme.bg_sidebar)
        .flex()
        .flex_col()
        .gap_4()
        .p_3p5()
        // 1. Vector-placed Mind Graph without labels, with relational edges
        .child(render_obsidian_dot_graph(posts, docs, theme))
        // 2. Clickable GitHub-style Thought Cadence Activity Heatmap Calendar
        .child(render_clickable_heatmap(selected_day, on_click, theme))
        // 3. Scrollable Random Sparks Discovery Feed
        .child(render_random_spark_feed(posts, theme))
}
