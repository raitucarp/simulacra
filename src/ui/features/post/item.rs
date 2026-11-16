//! Comprehensive Threads-style post item composition.

use super::actions::{render_action_bar, PostActionHandlers};
use super::attachments::{render_image_embed, render_link_preview};
use super::body::render_post_body;
use super::consts::POST_ITEM_ID_PREFIX;
use super::header::{render_author_row, render_repost_header};
use super::lookup::PostLookup;
use super::quote_card::render_quote_embed;
use super::threadline::{render_avatar_column, ThreadlineStyle};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, App, ClickEvent, Div, InteractiveElement, IntoElement, MouseButton, ParentElement, Rgba,
    Styled, Window,
};

pub type PostSelectCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Comprehensive Threads-style Post Molecule with tokenized layout, context-aware threadline,
/// attachment embeds (image & link), quote cards, repost header, and click-to-focus inspection.
#[allow(clippy::too_many_arguments)]
pub fn render_post_item<L: PostLookup>(
    post: &Post,
    lookup: L,
    theme: &ThemeTokens,
    is_focused: bool,
    threadline_style: ThreadlineStyle,
    on_select: impl Fn(&mut Window, &mut App) + 'static,
    on_like: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_reply: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_repost: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    on_quote: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let (effective_post, is_instant_repost) = resolve_effective_post(post, &lookup);
    let (bg_color, hover_bg) = surface_colors(is_focused, theme);
    let handlers = PostActionHandlers {
        on_like: Box::new(on_like),
        on_reply: Box::new(on_reply),
        on_repost: Box::new(on_repost),
        on_quote: Box::new(on_quote),
    };
    let has_repost_header = post.repost_id.is_some();

    div()
        .id(format!("{POST_ITEM_ID_PREFIX}{}", post.id))
        .w_full()
        .px(theme.spacing_lg)
        .bg(bg_color)
        .hover(move |s| s.bg(hover_bg))
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_default()
        .on_mouse_down(MouseButton::Left, move |_ev, window, cx| on_select(window, cx))
        .flex()
        .items_start()
        .gap(theme.spacing_md)
        .child(render_avatar_column(&effective_post.author, threadline_style, has_repost_header, theme))
        .child(render_content_column(post, effective_post, is_instant_repost, lookup, theme, handlers))
}

/// Card background and hover colors (focused cards stay elevated on hover).
fn surface_colors(is_focused: bool, theme: &ThemeTokens) -> (Rgba, Rgba) {
    if is_focused {
        (theme.bg_elevated, theme.bg_elevated)
    } else {
        (theme.bg_feed_post, theme.hover_bg)
    }
}

/// Resolves the instant-repost target when the post content is empty.
fn resolve_effective_post<'a, L: PostLookup>(post: &'a Post, lookup: &'a L) -> (&'a Post, bool) {
    if post.repost_id.is_none() || !post.content.trim().is_empty() {
        return (post, false);
    }
    let rep_id = post.repost_id.as_deref().unwrap_or("");
    match lookup.find_post(rep_id) {
        Some(target) => (target, true),
        None => (post, false),
    }
}

/// Right column: repost header, author info, body, attachments, quote, and action bar.
fn render_content_column<L: PostLookup>(
    post: &Post,
    effective: &Post,
    is_instant_repost: bool,
    lookup: L,
    theme: &ThemeTokens,
    handlers: PostActionHandlers,
) -> Div {
    let image_path = effective.image_path.as_deref().map(str::trim).filter(|p| !p.is_empty());
    div()
        .flex_1()
        .min_w_0()
        .py(theme.spacing_md)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .when(post.repost_id.is_some(), |this| this.child(render_repost_header(&post.author.name, theme)))
        .child(render_author_row(effective, is_instant_repost, theme))
        .child(
            div()
                .w_full()
                .min_w_0()
                .py(theme.spacing_2xs)
                .child(render_post_body(&effective.content, lookup, theme)),
        )
        .when_some(image_path, |this, path| this.child(render_image_embed(path, theme)))
        .when_some(effective.link_url.as_ref(), |this, url| {
            this.child(render_link_preview(effective.id.as_str(), url, theme))
        })
        .when_some(effective.quote_id.as_ref(), |this, quote_id| {
            this.child(render_quote_embed(quote_id.as_str(), lookup, theme))
        })
        .child(render_action_bar(effective, theme, handlers))
}
