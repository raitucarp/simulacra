//! Post, document and tag node chips rendered in the Mind Graph cluster canvas.

use crate::domain::{GroundingDocument, Post};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::{div, Div, InteractiveElement, ParentElement, Stateful, Styled};

/// Posts longer than this many characters are truncated in their node excerpt.
const EXCERPT_MAX_CHARS: usize = 38;
/// Number of characters kept before the ellipsis when truncating.
const EXCERPT_KEEP_CHARS: usize = 35;
const HASH_PREFIX: char = '#';

/// Character-safe excerpt of a post body for its graph node.
fn excerpt(content: &str) -> String {
    if content.chars().count() > EXCERPT_MAX_CHARS {
        let truncated: String = content.chars().take(EXCERPT_KEEP_CHARS).collect();
        format!("{}...", truncated)
    } else {
        content.to_string()
    }
}

fn render_post_title(post: &Post, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(div().w_2().h_2().rounded_full().bg(theme.accent_agent))
        .child(div().text_xs().font_bold().text_color(theme.text_primary).child(post.id.to_string()))
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .child(format!("({} refs)", post.linked_posts.len())),
        )
}

pub(super) fn render_post_node(idx: usize, post: &Post, theme: &ThemeTokens) -> Stateful<Div> {
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
        .child(render_post_title(post, theme))
        .child(div().text_xs().text_color(theme.text_secondary).child(excerpt(&post.content)))
}

pub(super) fn render_doc_node(idx: usize, doc: &GroundingDocument, theme: &ThemeTokens) -> Stateful<Div> {
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
        .child(div().w_2().h_2().rounded_full().bg(theme.accent_user))
        .child(Icon::new(LucideIcon::FileText).size_3p5().text_color(theme.accent_user))
        .child(div().text_xs().font_medium().text_color(theme.text_primary).child(doc.title.clone()))
}

pub(super) fn render_tag_node(idx: usize, tag: &str, theme: &ThemeTokens) -> Stateful<Div> {
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
        .child(div().w_3().h_3().text_color(theme.accent_alt).child(Icon::new(LucideIcon::Hash)))
        .child(
            div()
                .text_xs()
                .font_medium()
                .text_color(theme.accent_alt)
                .child(tag.trim_start_matches(HASH_PREFIX).to_string()),
        )
}
