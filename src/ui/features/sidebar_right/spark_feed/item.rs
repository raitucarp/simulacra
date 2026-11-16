//! A single card in the random sparks discovery feed.

use crate::domain::{Post, Profile};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::icons::LucideIcon;
use crate::ui::molecules::post_item::{render_post_body, PostLookup};
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const SPARK_ITEM_ID_PREFIX: &str = "spark-item";
const SPARK_TIME_FORMAT: &str = "%H:%M";

fn render_author(author: &Profile, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_1p5()
        .child(render_profile_avatar(author, theme.avatar_size_sm, theme))
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_xs()
                .font_bold()
                .text_color(theme.text_primary)
                .child(author.name.clone()),
        )
        .child(
            div()
                .font_family(theme.font_family_ui.clone())
                .text_xs()
                .text_color(theme.text_muted)
                .child(author.handle.clone()),
        )
}

fn render_item_header(post: &Post, author: &Profile, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .child(render_author(author, theme))
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_xs()
                .text_color(theme.text_muted)
                .child(post.created_at.format(SPARK_TIME_FORMAT).to_string()),
        )
}

/// Icon plus an optional count (hidden when zero).
fn render_stat(icon: LucideIcon, count: Option<String>) -> Div {
    div()
        .flex()
        .items_center()
        .gap_1()
        .child(Icon::new(icon).size_3())
        .when_some(count, |this, count| this.child(count))
}

fn render_stats(post: &Post, theme: &ThemeTokens) -> Div {
    let likes = (post.likes > 0).then(|| post.likes.to_string());
    let replies = (post.replies_count > 0).then(|| post.replies_count.to_string());
    let links = (!post.linked_posts.is_empty()).then(|| post.linked_posts.len().to_string());
    div()
        .flex()
        .items_center()
        .gap_3()
        .pt_0p5()
        .text_xs()
        .text_color(theme.text_muted)
        .child(render_stat(LucideIcon::Heart, likes))
        .child(render_stat(LucideIcon::MessageSquare, replies))
        .child(render_stat(LucideIcon::GitBranch, links))
}

/// Renders one spark card with author header, wikilink-aware body and engagement stats.
pub(super) fn render_spark_item<L: PostLookup>(
    idx: usize,
    post: &Post,
    lookup: L,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    div()
        .id(ElementId::NamedInteger(SPARK_ITEM_ID_PREFIX.into(), idx as u64))
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
        .child(render_item_header(post, lookup.author_of(post), theme))
        .child(div().w_full().child(render_post_body(&post.content, lookup, theme)))
        .child(render_stats(post, theme))
}
