//! Scrollable random sparks (serendipitous thoughts) discovery feed.

mod item;

use super::section::{render_section_header, render_section_title};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::molecules::post_item::PostLookup;
use gpui_kit::base::StyledExt;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

/// Maximum number of recent posts scanned when resolving spark previews.
const MAX_GRAPH_SCAN_WINDOW: usize = 128;
/// Maximum number of thoughts rendered in the serendipitous discovery spark feed.
pub const MAX_SPARK_FEED_ITEMS: usize = 4;
const SPARK_SCROLL_ID: &str = "random-spark-scroll-feed";

fn render_scroll_badge(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_xs_plus)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .bg(theme.hover_bg)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .font_family(theme.font_family_ui.clone())
        .text_size(theme.font_size_label)
        .font_medium()
        .text_color(theme.text_muted)
        .child(div().w(theme.icon_size_sm).h(theme.icon_size_sm).child(Icon::new(LucideIcon::Dices)))
        .child(t!("sidebar.scroll_badge").to_string())
}

fn render_feed_header(theme: &ThemeTokens) -> Div {
    let title = render_section_title(
        LucideIcon::Sparkles,
        theme.accent_alt,
        t!("sidebar.sparks_feed_title").to_string(),
        theme,
    );
    render_section_header(title, render_scroll_badge(theme)).px(theme.spacing_md)
}

/// Renders a scrollable feed of random / serendipitous thoughts from the database.
pub fn render_random_spark_feed(posts: &[Post], theme: &ThemeTokens) -> impl IntoElement {
    let bounded_lookup = &posts[..posts.len().min(MAX_GRAPH_SCAN_WINDOW)];
    render_random_spark_feed_with_lookup(posts, bounded_lookup, theme)
}

/// Renders a scrollable feed of random / serendipitous thoughts using an O(1) `PostLookup`.
pub fn render_random_spark_feed_with_lookup<L: PostLookup>(
    posts: &[Post],
    lookup: L,
    theme: &ThemeTokens,
) -> Div {
    let spark_elements: Vec<Stateful<Div>> = posts
        .iter()
        .take(MAX_SPARK_FEED_ITEMS)
        .enumerate()
        .map(|(idx, post)| item::render_spark_item(idx, post, lookup, theme))
        .collect();

    div()
        .w_full()
        .flex_1()
        .min_h_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_sm_plus)
        .child(render_feed_header(theme))
        .child(
            div()
                .id(SPARK_SCROLL_ID)
                .w_full()
                .flex_1()
                .min_h_0()
                .overflow_y_scrollbar()
                .px(theme.spacing_md)
                .flex()
                .flex_col()
                .gap(theme.spacing_sm)
                .children(spark_elements),
        )
}
