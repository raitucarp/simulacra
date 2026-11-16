//! Absolute-positioned edge lines and dot nodes painted on the dot graph canvas.

use super::layout::Point;
use super::{NODE_SIZE_AGENT, NODE_SIZE_NEUTRAL, NODE_SIZE_USER};
use crate::domain::{Post, ProfileRole};
use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Thickness of an orthogonal edge line.
const EDGE_THICKNESS: f32 = 1.0;
/// Minimum rendered edge length so degenerate edges stay visible.
const EDGE_MIN_LENGTH: f32 = 1.0;
/// Divisor centering a node on its coordinate.
const NODE_CENTER_DIVISOR: f32 = 2.0;

/// Renders an orthogonal circuit-style edge as a horizontal then a vertical line.
pub(super) fn render_edge_lines((from, to): (Point, Point), theme: &ThemeTokens) -> [Div; 2] {
    let ((x1, y1), (x2, y2)) = (from, to);
    let (min_x, max_x) = (x1.min(x2), x1.max(x2));
    let (min_y, max_y) = (y1.min(y2), y1.max(y2));
    [
        div()
            .absolute()
            .left(px(min_x))
            .top(px(y1))
            .w(px((max_x - min_x).max(EDGE_MIN_LENGTH)))
            .h(px(EDGE_THICKNESS))
            .bg(theme.graph_edge),
        div()
            .absolute()
            .left(px(x2))
            .top(px(min_y))
            .w(px(EDGE_THICKNESS))
            .h(px((max_y - min_y).max(EDGE_MIN_LENGTH)))
            .bg(theme.graph_edge),
    ]
}

fn node_style(post: &Post, theme: &ThemeTokens) -> (Rgba, f32) {
    if matches!(post.author.role, ProfileRole::Agent { .. }) {
        (theme.accent_agent, NODE_SIZE_AGENT)
    } else if !post.linked_posts.is_empty() {
        (theme.accent_user, NODE_SIZE_USER)
    } else {
        (theme.graph_node_neutral, NODE_SIZE_NEUTRAL)
    }
}

/// Renders a dot node WITHOUT a text label.
pub(super) fn render_node(post: &Post, (x, y): Point, theme: &ThemeTokens) -> Div {
    let (node_color, node_size) = node_style(post, theme);
    div()
        .absolute()
        .left(px(x - node_size / NODE_CENTER_DIVISOR))
        .top(px(y - node_size / NODE_CENTER_DIVISOR))
        .w(px(node_size))
        .h(px(node_size))
        .rounded_full()
        .bg(node_color)
        .border_1()
        .border_color(theme.border)
}
