//! Interactive Obsidian-style Mind Graph 2D visualization view.

mod header;
mod nodes;

use std::collections::HashSet;

use crate::domain::{GroundingDocument, Post};
use crate::theme::ThemeTokens;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::{div, Div, InteractiveElement, IntoElement, ParentElement, Rgba, Styled};
use rust_i18n::t;

const MIND_GRAPH_SCROLL_ID: &str = "mind-graph-scroll-container";
const HASH_PREFIX: char = '#';
/// Minimum word length (prefix included) for a `#tag` to count as a tag node.
const MIN_TAG_LEN: usize = 2;

/// Collects distinct `#tags` across posts and the total number of post links.
fn collect_tags_and_links(posts: &[Post]) -> (HashSet<String>, usize) {
    let mut tags = HashSet::new();
    let mut total_links = 0;
    for post in posts {
        total_links += post.linked_posts.len();
        for word in post.content.split_whitespace() {
            if word.starts_with(HASH_PREFIX) && word.len() >= MIN_TAG_LEN {
                tags.insert(word.to_string());
            }
        }
    }
    (tags, total_links)
}

fn render_legend_entry(color: Rgba, label: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(div().w(theme.spacing_sm).h(theme.spacing_sm).rounded(theme.radius_full).bg(color))
        .child(label)
}

fn render_legend(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xl)
        .font_family(theme.font_family_ui.clone())
        .text_size(theme.font_size_xs)
        .text_color(theme.text_muted)
        .child(render_legend_entry(theme.accent_agent, t!("graph.legend_thoughts").to_string(), theme))
        .child(render_legend_entry(theme.accent_user, t!("graph.legend_docs").to_string(), theme))
        .child(render_legend_entry(theme.accent_alt, t!("graph.legend_tags").to_string(), theme))
}

/// Graph node cluster canvas: post nodes, then document nodes, then tag nodes.
fn render_cluster(posts: &[Post], documents: &[GroundingDocument], tags: HashSet<String>, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .min_h(theme.graph_canvas_height)
        .p(theme.spacing_lg)
        .rounded(theme.radius_md)
        .border_1()
        .border_color(theme.border)
        .bg(theme.bg_app)
        .flex()
        .flex_wrap()
        .gap_3()
        .items_center()
        .children(posts.iter().enumerate().map(|(idx, post)| nodes::render_post_node(idx, post, theme)))
        .children(documents.iter().enumerate().map(|(idx, doc)| nodes::render_doc_node(idx, doc, theme)))
        .children(tags.into_iter().enumerate().map(|(idx, tag)| nodes::render_tag_node(idx, &tag, theme)))
}

pub fn render_mind_graph_view(
    posts: &[Post],
    documents: &[GroundingDocument],
    theme: &ThemeTokens,
) -> impl IntoElement {
    let (tags, total_links) = collect_tags_and_links(posts);
    let total_nodes = posts.len() + documents.len() + tags.len();

    div()
        .id(MIND_GRAPH_SCROLL_ID)
        .flex_1()
        .h_full()
        .overflow_y_scrollbar()
        .p(theme.spacing_xl)
        .bg(theme.bg_app)
        .flex()
        .flex_col()
        .gap(theme.spacing_xl)
        .child(header::render_header(total_nodes, total_links, theme))
        // 2D Topological Canvas Card
        .child(
            div()
                .w_full()
                .p(theme.spacing_xl)
                .rounded(theme.radius_lg)
                .border(theme.border_width_hairline)
                .border_color(theme.border)
                .bg(theme.bg_card)
                .flex()
                .flex_col()
                .gap(theme.spacing_lg)
                .child(render_legend(theme))
                .child(render_cluster(posts, documents, tags, theme)),
        )
}
