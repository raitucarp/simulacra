//! Interactive Obsidian-style Mind Graph 2D visualization view organism.

use crate::domain::{GroundingDocument, Post};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::{div, InteractiveElement, IntoElement, ParentElement, Styled};
use rust_i18n::t;
use std::collections::HashSet;

pub fn render_mind_graph_view(
    posts: &[Post],
    documents: &[GroundingDocument],
    theme: &ThemeTokens,
) -> impl IntoElement {
    let mut tags = HashSet::new();
    let mut total_links = 0;

    for post in posts {
        total_links += post.linked_posts.len();
        for word in post.content.split_whitespace() {
            if word.starts_with('#') && word.len() > 1 {
                tags.insert(word.to_string());
            }
        }
    }

    let total_nodes = posts.len() + documents.len() + tags.len();

    div()
        .id("mind-graph-scroll-container")
        .flex_1()
        .h_full()
        .overflow_y_scrollbar()
        .p(theme.spacing_xl)
        .bg(theme.bg_app)
        .flex()
        .flex_col()
        .gap(theme.spacing_xl)
        // Header
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(
                            div()
                                .font_family(theme.font_family_display.clone())
                                .text_size(theme.font_size_lg)
                                .font_weight(gpui_kit::FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(t!("graph.title").to_string()),
                        )
                        .child(
                            div()
                                .font_family(theme.font_family_ui.clone())
                                .text_size(theme.font_size_xs)
                                .text_color(theme.text_secondary)
                                .child(t!("graph.subtitle").to_string()),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(theme.spacing_sm)
                        .child(
                            div()
                                .px(theme.spacing_md)
                                .py(theme.spacing_xs)
                                .rounded(theme.radius_full)
                                .bg(theme.bg_card)
                                .border(theme.border_width_hairline)
                                .border_color(theme.border)
                                .font_family(theme.font_family_mono.clone())
                                .text_size(theme.font_size_xs)
                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                                .text_color(theme.accent_agent)
                                .child(t!("graph.nodes_count", count = total_nodes).to_string()),
                        )
                        .child(
                            div()
                                .px(theme.spacing_md)
                                .py(theme.spacing_xs)
                                .rounded(theme.radius_full)
                                .bg(theme.bg_card)
                                .border(theme.border_width_hairline)
                                .border_color(theme.border)
                                .font_family(theme.font_family_mono.clone())
                                .text_size(theme.font_size_xs)
                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                                .text_color(theme.accent_user)
                                .child(t!("graph.links_count", count = total_links).to_string()),
                        ),
                ),
        )
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
                // Legend
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(theme.spacing_xl)
                        .font_family(theme.font_family_ui.clone())
                        .text_size(theme.font_size_xs)
                        .text_color(theme.text_muted)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(theme.spacing_sm)
                                .child(
                                    div()
                                        .w(theme.spacing_sm)
                                        .h(theme.spacing_sm)
                                        .rounded(theme.radius_full)
                                        .bg(theme.accent_agent),
                                )
                                .child(t!("graph.legend_thoughts").to_string()),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(theme.spacing_sm)
                                .child(
                                    div()
                                        .w(theme.spacing_sm)
                                        .h(theme.spacing_sm)
                                        .rounded(theme.radius_full)
                                        .bg(theme.accent_user),
                                )
                                .child(t!("graph.legend_docs").to_string()),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(theme.spacing_sm)
                                .child(
                                    div()
                                        .w(theme.spacing_sm)
                                        .h(theme.spacing_sm)
                                        .rounded(theme.radius_full)
                                        .bg(theme.accent_alt),
                                )
                                .child(t!("graph.legend_tags").to_string()),
                        ),
                )
                // Graph Node Cluster Canvas
                .child(
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
                        // 1. Post Nodes
                        .children(posts.iter().enumerate().map(|(idx, post)| {
                            let excerpt = if post.content.chars().count() > 38 {
                                let truncated: String = post.content.chars().take(35).collect();
                                format!("{}...", truncated)
                            } else {
                                post.content.clone()
                            };

                            div()
                                .id(format!("graph-post-node-{}", idx))
                                .p_2p5()
                                .rounded_lg()
                                .border_1()
                                .border_color(theme.accent_agent)
                                .bg(theme.hover_bg)
                                .cursor_pointer()
                                .hover(move |s| s.bg(theme.bg_card))
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .w_2()
                                                .h_2()
                                                .rounded_full()
                                                .bg(theme.accent_agent),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_bold()
                                                .text_color(theme.text_primary)
                                                .child(post.id.to_string()),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(format!("({} refs)", post.linked_posts.len())),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_secondary)
                                        .child(excerpt),
                                )
                        }))
                        // 2. Document Nodes
                        .children(documents.iter().enumerate().map(|(idx, doc)| {
                            div()
                                .id(format!("graph-doc-node-{}", idx))
                                .p_2p5()
                                .rounded_lg()
                                .border_1()
                                .border_color(theme.accent_user)
                                .bg(theme.hover_bg)
                                .cursor_pointer()
                                .hover(move |s| s.bg(theme.bg_card))
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .w_2()
                                        .h_2()
                                        .rounded_full()
                                        .bg(theme.accent_user),
                                )
                                .child(Icon::new(LucideIcon::FileText).size_3p5().text_color(theme.accent_user))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(theme.text_primary)
                                        .child(doc.title.clone()),
                                )
                        }))
                        // 3. Tag Nodes
                        .children(tags.into_iter().enumerate().map(|(idx, tag)| {
                            div()
                                .id(format!("graph-tag-node-{}", idx))
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_3()
                                .py_1p5()
                                .rounded_full()
                                .border_1()
                                .border_color(theme.accent_alt)
                                .bg(theme.bg_card)
                                .cursor_pointer()
                                .hover(move |s| s.bg(theme.hover_bg))
                                .child(
                                    div()
                                        .w_3()
                                        .h_3()
                                        .text_color(theme.accent_alt)
                                        .child(Icon::new(LucideIcon::Hash)),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(theme.accent_alt)
                                        .child(tag.trim_start_matches('#').to_string()),
                                )
                        })),
                ),
        )
}
