//! Obsidian-style dot-only Mind Graph with vector-placed nodes and relational edges.

mod elements;
mod layout;

use super::section::{render_divided_section, render_mono_badge, render_section_header, render_section_title};
use crate::domain::{GroundingDocument, Post};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

/// Maximum number of sampled thought nodes rendered in the Obsidian 2D topological graph.
pub const MAX_OBSIDIAN_GRAPH_NODES: usize = 16;
/// Maximum scan window when sampling connected nodes in the Obsidian 2D topological graph.
pub const MAX_GRAPH_SCAN_WINDOW: usize = 128;
pub const CANVAS_WIDTH: f32 = 246.0;
pub const CANVAS_HEIGHT: f32 = 140.0;
pub const GOLDEN_ANGLE_RADIANS: f32 = 2.3999632;
pub const CANVAS_PADDING: f32 = 14.0;
pub const NODE_SIZE_AGENT: f32 = 7.0;
pub const NODE_SIZE_USER: f32 = 6.0;
pub const NODE_SIZE_NEUTRAL: f32 = 5.0;

fn render_graph_header(node_count: usize, edge_count: usize, theme: &ThemeTokens) -> Div {
    render_section_header(
        render_section_title(
            LucideIcon::Share2,
            theme.accent_agent,
            t!("sidebar.mind_graph_title").to_string(),
            theme,
        ),
        render_mono_badge(
            format!("{} nodes · {} edges", node_count, edge_count),
            theme.accent_agent,
            theme,
        )
        .rounded_md(),
    )
}

/// 2D topological canvas (dots & edges only, no text labels).
fn render_canvas(edges: Vec<Div>, nodes: Vec<Div>, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .h(px(CANVAS_HEIGHT))
        .rounded_lg()
        .border_1()
        .border_color(theme.border)
        .bg(theme.bg_app)
        .relative()
        .overflow_hidden()
        .children(edges)
        .children(nodes)
}

/// Renders the Obsidian-style dot-only Mind Graph with vector-placed nodes and relational edges.
pub fn render_obsidian_dot_graph(
    posts: &[Post],
    _docs: &[GroundingDocument],
    theme: &ThemeTokens,
) -> impl IntoElement {
    let sampled_posts = layout::sample_posts(posts);
    let coords = layout::spiral_coords(sampled_posts.len());
    let segments = layout::edge_segments(&sampled_posts, &coords);
    let edges: Vec<Div> = segments
        .iter()
        .flat_map(|segment| elements::render_edge_lines(*segment, theme))
        .collect();
    let nodes: Vec<Div> = sampled_posts
        .iter()
        .zip(coords.iter())
        .map(|(post, point)| elements::render_node(post, *point, theme))
        .collect();

    render_divided_section(theme)
        .child(render_graph_header(posts.len(), segments.len(), theme))
        .child(render_canvas(edges, nodes, theme))
}
