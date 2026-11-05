//! Secondary right sidebar organism featuring:
//! 1. Vector-sorted Dot Mind Graph with relational edges and no text labels
//! 2. Interactive Clickable Thought Cadence Heatmap Calendar
//! 3. Scrollable Random Sparks Discovery Feed

use std::rc::Rc;

use crate::domain::{GroundingDocument, Post};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::molecules::post_item::render_post_body;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

pub type ClickDayCallback = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// Maximum number of thoughts rendered in the serendipitous discovery spark feed.
pub const MAX_SPARK_FEED_ITEMS: usize = 4;
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

/// Renders the clickable GitHub-style thought activity calendar heatmap.
pub fn render_clickable_heatmap(
    selected_day: Option<usize>,
    on_click_day: ClickDayCallback,
    theme: &ThemeTokens,
) -> impl IntoElement {
    // 7 rows (Days: Sun..Sat) x 15 columns (Weeks)
    let activity_levels: [u8; 105] = [
        0, 1, 0, 2, 3, 0, 1,
        1, 2, 0, 0, 4, 3, 2,
        0, 3, 4, 2, 1, 0, 2,
        2, 0, 1, 3, 2, 4, 1,
        3, 4, 2, 0, 1, 3, 2,
        1, 0, 3, 4, 2, 1, 0,
        0, 2, 3, 1, 4, 2, 3,
        2, 3, 0, 2, 3, 4, 1,
        4, 1, 2, 3, 0, 2, 4,
        1, 0, 4, 2, 3, 1, 2,
        3, 2, 1, 4, 2, 0, 3,
        2, 4, 3, 1, 2, 4, 1,
        0, 1, 2, 4, 3, 2, 0,
        3, 2, 4, 1, 0, 3, 2,
        1, 4, 2, 3, 2, 4, 3,
    ];

    let mut weeks = Vec::new();
    for week_idx in 0..15 {
        let mut days = Vec::new();
        for day_idx in 0..7 {
            let day_cell_id = week_idx * 7 + day_idx;
            let level = activity_levels[day_cell_id];
            let is_selected = selected_day == Some(day_cell_id);

            let cell_color = match level {
                0 => theme.bg_card,
                1 => theme.cadence_level_1,
                2 => theme.cadence_level_2,
                3 => theme.cadence_level_3,
                _ => theme.cadence_level_4,
            };

            let border_color = if is_selected {
                theme.accent_brand
            } else {
                theme.border
            };

            let on_click = Rc::clone(&on_click_day);
            days.push(
                div()
                    .id(ElementId::NamedInteger("cal-cell".into(), day_cell_id as u64))
                    .w(theme.cadence_cell_size)
                    .h(theme.cadence_cell_size)
                    .rounded_sm()
                    .bg(cell_color)
                    .border_1()
                    .border_color(border_color)
                    .cursor_pointer()
                    .when(is_selected, |s| s.border_2().border_color(theme.text_white))
                    .on_click(move |_, window, cx| {
                        on_click(day_cell_id, window, cx);
                    }),
            );
        }

        weeks.push(
            div()
                .flex()
                .flex_col()
                .gap(px(2.5))
                .children(days),
        );
    }

    let subtitle = match selected_day {
        Some(day) => format!("Day #{}: {} sparks", day + 1, activity_levels[day % 105]),
        None => "128 sparks / yr · Click day to inspect".to_string(),
    };

    div()
        .w_full()
        .pb_4()
        .border_b_1()
        .border_color(theme.border)
        .flex()
        .flex_col()
        .gap_2p5()
        // Header
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .w_4()
                                .h_4()
                                .text_color(theme.accent_success)
                                .child(Icon::new(LucideIcon::Calendar)),
                        )
                        .child(
                            div()
                                .font_family(theme.font_family_display.clone())
                                .text_xs()
                                .font_bold()
                                .text_color(theme.text_primary)
                                .child(t!("sidebar.thought_cadence_title").to_string()),
                        ),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_full()
                        .bg(theme.hover_bg)
                        .border_1()
                        .border_color(theme.border)
                        .font_family(theme.font_family_mono.clone())
                        .text_xs()
                        .font_medium()
                        .text_color(theme.accent_success)
                        .child("128 sparks/yr"),
                ),
        )
        // Subtitle Status
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_xs()
                .text_color(theme.text_muted)
                .child(subtitle),
        )
        // Calendar Grid
        .child(
            div()
                .w_full()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    // Day labels
                    div()
                        .flex()
                        .flex_col()
                        .justify_between()
                        .h(px(85.0))
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(div().child("M"))
                        .child(div().child("W"))
                        .child(div().child("F")),
                )
                .child(
                    div()
                        .flex()
                        .gap(px(2.5))
                        .children(weeks),
                ),
        )
        // Legend
        .child(
            div()
                .flex()
                .items_center()
                .justify_end()
                .gap_1p5()
                .pt_1()
                .text_xs()
                .text_color(theme.text_muted)
                .child("Less")
                .child(div().w(px(8.0)).h(px(8.0)).rounded_sm().bg(theme.bg_card).border_1().border_color(theme.border))
                .child(div().w(px(8.0)).h(px(8.0)).rounded_sm().bg(theme.cadence_level_1))
                .child(div().w(px(8.0)).h(px(8.0)).rounded_sm().bg(theme.cadence_level_2))
                .child(div().w(px(8.0)).h(px(8.0)).rounded_sm().bg(theme.cadence_level_3))
                .child(div().w(px(8.0)).h(px(8.0)).rounded_sm().bg(theme.cadence_level_4))
                .child("More"),
        )
}

/// Renders the Obsidian-style dot-only Mind Graph with vector-placed nodes and relational edges.
pub fn render_obsidian_dot_graph(
    posts: &[Post],
    _docs: &[GroundingDocument],
    theme: &ThemeTokens,
) -> impl IntoElement {
    let center_x = CANVAS_WIDTH / 2.0;
    let center_y = CANVAS_HEIGHT / 2.0;

    // Sample up to MAX_OBSIDIAN_GRAPH_NODES nodes within a bounded scan window, prioritizing connected thoughts
    let scan_slice = &posts[..posts.len().min(MAX_GRAPH_SCAN_WINDOW)];
    let sampled_posts: Vec<&Post> = if scan_slice.len() <= MAX_OBSIDIAN_GRAPH_NODES {
        scan_slice.iter().collect()
    } else {
        let mut sampled: Vec<&Post> = scan_slice
            .iter()
            .filter(|p| !p.linked_posts.is_empty() || p.reply_to_id.is_some())
            .take(MAX_OBSIDIAN_GRAPH_NODES)
            .collect();
        if sampled.len() < MAX_OBSIDIAN_GRAPH_NODES {
            for p in scan_slice {
                if sampled.len() >= MAX_OBSIDIAN_GRAPH_NODES {
                    break;
                }
                if !sampled.iter().any(|s| s.id == p.id) {
                    sampled.push(p);
                }
            }
        }
        sampled
    };

    // Calculate (x, y) coordinates for all post nodes distributed by golden angle spiral (vector simulation)
    let mut coords = Vec::new();
    let count = sampled_posts.len().max(1);

    for idx in 0..count {
        let golden_angle = (idx as f32) * GOLDEN_ANGLE_RADIANS;
        let r = 18.0 + ((idx as f32) * 11.5).min(52.0);
        let x = center_x + r * golden_angle.cos();
        let y = center_y + r * 0.72 * golden_angle.sin();
        coords.push((
            x.clamp(CANVAS_PADDING, CANVAS_WIDTH - CANVAS_PADDING),
            y.clamp(CANVAS_PADDING, CANVAS_HEIGHT - CANVAS_PADDING),
        ));
    }

    let mut edges = Vec::new();
    let mut total_edges_count = 0;

    let id_to_coord: std::collections::HashMap<&crate::domain::PostId, (f32, f32)> = sampled_posts
        .iter()
        .zip(coords.iter())
        .map(|(p, c)| (&p.id, *c))
        .collect();

    // Build relational edge connections between linked thoughts
    for (i, post) in sampled_posts.iter().enumerate() {
        if i >= coords.len() { break; }
        let (x1, y1) = coords[i];

        for linked_id in &post.linked_posts {
            if let Some(&(x2, y2)) = id_to_coord.get(linked_id) {
                if (x1, y1) != (x2, y2) {
                    total_edges_count += 1;
                    let min_x = x1.min(x2);
                    let max_x = x1.max(x2);
                    let min_y = y1.min(y2);
                    let max_y = y1.max(y2);

                    // Render orthogonal circuit/graph edge line
                    edges.push(
                        div()
                            .absolute()
                            .left(px(min_x))
                            .top(px(y1))
                            .w(px((max_x - min_x).max(1.0)))
                            .h(px(1.0))
                            .bg(theme.graph_edge),
                    );
                    edges.push(
                        div()
                            .absolute()
                            .left(px(x2))
                            .top(px(min_y))
                            .w(px(1.0))
                            .h(px((max_y - min_y).max(1.0)))
                            .bg(theme.graph_edge),
                    );
                }
            }
        }
    }

    // Render dot nodes WITHOUT text labels
    let mut node_elements = Vec::new();
    for (idx, post) in sampled_posts.iter().enumerate() {
        if idx >= coords.len() { break; }
        let (x, y) = coords[idx];
        let is_agent = matches!(post.author.role, crate::domain::ProfileRole::Agent { .. });

        let (node_color, node_size) = if is_agent {
            (theme.accent_agent, NODE_SIZE_AGENT)
        } else if !post.linked_posts.is_empty() {
            (theme.accent_user, NODE_SIZE_USER)
        } else {
            (theme.graph_node_neutral, NODE_SIZE_NEUTRAL)
        };

        node_elements.push(
            div()
                .absolute()
                .left(px(x - node_size / 2.0))
                .top(px(y - node_size / 2.0))
                .w(px(node_size))
                .h(px(node_size))
                .rounded_full()
                .bg(node_color)
                .border_1()
                .border_color(theme.border),
        );
    }

    div()
        .w_full()
        .pb_4()
        .border_b_1()
        .border_color(theme.border)
        .flex()
        .flex_col()
        .gap_2p5()
        // Header
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .w_4()
                                .h_4()
                                .text_color(theme.accent_agent)
                                .child(Icon::new(LucideIcon::Share2)),
                        )
                        .child(
                            div()
                                .font_family(theme.font_family_display.clone())
                                .text_xs()
                                .font_bold()
                                .text_color(theme.text_primary)
                                .child(t!("sidebar.mind_graph_title").to_string()),
                        ),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_md()
                        .bg(theme.hover_bg)
                        .border_1()
                        .border_color(theme.border)
                        .font_family(theme.font_family_mono.clone())
                        .text_xs()
                        .font_medium()
                        .text_color(theme.accent_agent)
                        .child(format!("{} nodes · {} edges", posts.len(), total_edges_count)),
                ),
        )
        // 2D Topological Canvas (Dots & Edges only, no text labels)
        .child(
            div()
                .w_full()
                .h(px(CANVAS_HEIGHT))
                .rounded_lg()
                .border_1()
                .border_color(theme.border)
                .bg(theme.bg_app)
                .relative()
                .overflow_hidden()
                // Render graph edges
                .children(edges)
                // Render dot nodes
                .children(node_elements),
        )
}

/// Renders a scrollable feed of random / serendipitous thoughts from the database.
pub fn render_random_spark_feed(posts: &[Post], theme: &ThemeTokens) -> impl IntoElement {
    let bounded_lookup = &posts[..posts.len().min(MAX_GRAPH_SCAN_WINDOW)];
    render_random_spark_feed_with_lookup(posts, bounded_lookup, theme)
}

/// Renders a scrollable feed of random / serendipitous thoughts using an O(1) `PostLookup`.
pub fn render_random_spark_feed_with_lookup<L: crate::ui::molecules::post_item::PostLookup>(
    posts: &[Post],
    lookup: L,
    theme: &ThemeTokens,
) -> Div {
    let mut spark_elements = Vec::with_capacity(MAX_SPARK_FEED_ITEMS);

    for (idx, post) in posts.iter().take(MAX_SPARK_FEED_ITEMS).enumerate() {
        let time_str = post.created_at.format("%H:%M").to_string();

        spark_elements.push(
            div()
                .id(ElementId::NamedInteger("spark-item".into(), idx as u64))
                .w_full()
                .p_2p5()
                .rounded_lg()
                .border_1()
                .border_color(theme.border)
                .bg(theme.bg_app)
                .cursor_default()
                .hover(move |s| s.bg(theme.hover_bg))
                .flex()
                .flex_col()
                .gap_1p5()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .child(render_profile_avatar(&post.author, theme.avatar_size_sm, theme))
                                .child(
                                    div()
                                        .font_family(theme.font_family_display.clone())
                                        .text_xs()
                                        .font_bold()
                                        .text_color(theme.text_primary)
                                        .child(post.author.name.clone()),
                                )
                                .child(
                                    div()
                                        .font_family(theme.font_family_ui.clone())
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(post.author.handle.clone()),
                                ),
                        )
                        .child(
                            div()
                                .font_family(theme.font_family_mono.clone())
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(time_str),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .child(render_post_body(&post.content, lookup, theme)),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .pt_0p5()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(Icon::new(LucideIcon::Heart).size_3())
                                .when(post.likes > 0, |this| this.child(post.likes.to_string())),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(Icon::new(LucideIcon::MessageSquare).size_3())
                                .when(post.replies_count > 0, |this| this.child(post.replies_count.to_string())),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(Icon::new(LucideIcon::GitBranch).size_3())
                                .when(!post.linked_posts.is_empty(), |this| this.child(post.linked_posts.len().to_string())),
                        ),
                ),
        );
    }

    div()
        .w_full()
        .flex_1()
        .min_h_0()
        .flex()
        .flex_col()
        .gap_2p5()
        // Header
        .child(
            div()
                .px(theme.spacing_md)
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .w_4()
                                .h_4()
                                .text_color(theme.accent_alt)
                                .child(Icon::new(LucideIcon::Sparkles)),
                        )
                        .child(
                            div()
                                .font_family(theme.font_family_display.clone())
                                .text_xs()
                                .font_bold()
                                .text_color(theme.text_primary)
                                .child(t!("sidebar.sparks_feed_title").to_string()),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_1p5()
                        .py_0p5()
                        .rounded_md()
                        .bg(theme.hover_bg)
                        .border_1()
                        .border_color(theme.border)
                        .font_family(theme.font_family_ui.clone())
                        .text_xs()
                        .font_medium()
                        .text_color(theme.text_muted)
                        .child(
                            div()
                                .w_3()
                                .h_3()
                                .child(Icon::new(LucideIcon::Dices)),
                        )
                        .child("Scroll"),
                ),
        )
        // Scrollable Feed List (Flex-1 fills remaining space to bottom)
        .child(
            div()
                .id("random-spark-scroll-feed")
                .w_full()
                .flex_1()
                .min_h_0()
                .overflow_y_scrollbar()
                .px(theme.spacing_md)
                .flex()
                .flex_col()
                .gap_2()
                .children(spark_elements),
        )
}

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
        .id("secondary-right-sidebar")
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
