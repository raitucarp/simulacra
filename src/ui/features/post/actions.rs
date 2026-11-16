//! Threads-style action bar (Heart, Reply, Repost, Quote, Refs, Share).

use super::consts::{LIKE_BTN_ID_PREFIX, QUOTE_BTN_ID_PREFIX, REPLY_BTN_ID_PREFIX, REPOST_BTN_ID_PREFIX};
use super::controls::{render_refs_button, render_share_button, PostAction, PostCardHandlers, PostReference};
use crate::domain::{Post, Profile};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, App, ClickEvent, Div, ElementId, FontWeight, InteractiveElement, IntoElement,
    ParentElement, Rgba, StatefulInteractiveElement, Styled, Window,
};

pub type ActionClickCallback = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// Visual state of one counter button.
struct ActionLook {
    icon: LucideIcon,
    count: usize,
    is_active: bool,
    active_color: Rgba,
}

/// Atomic Threads-style action button helper with tokenized typography and hover states.
fn render_action_btn(
    id: impl Into<ElementId>,
    look: ActionLook,
    theme: &ThemeTokens,
    on_click: PostAction,
) -> impl IntoElement {
    let active_color = look.active_color;
    let base_color = if look.is_active { active_color } else { theme.text_muted };
    let hover_bg = theme.hover_bg;
    let btn = div()
        .id(id)
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_xs)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_xs)
        .font_weight(FontWeight::MEDIUM)
        .text_color(base_color)
        .hover(move |s| s.bg(hover_bg).text_color(active_color))
        .on_click(move |_, window, cx| on_click(window, cx))
        .child(Icon::new(look.icon).size(theme.font_size_md));
    if look.count > 0 {
        btn.child(look.count.to_string())
    } else {
        btn
    }
}

/// Full action bar for `post` wired to `handlers`; `refs` feeds the references menu.
pub(crate) fn render_action_bar(
    post: &Post,
    author: &Profile,
    refs: Vec<PostReference>,
    theme: &ThemeTokens,
    handlers: &PostCardHandlers,
) -> Div {
    let id = &post.id;
    let like_color = if post.is_liked { theme.accent_brand } else { theme.text_secondary };
    let repost_color = if post.is_reposted { theme.accent_success } else { theme.text_secondary };
    let like = ActionLook { icon: LucideIcon::Heart, count: post.likes, is_active: post.is_liked, active_color: like_color };
    let reply = ActionLook { icon: LucideIcon::MessageSquare, count: post.replies_count, is_active: false, active_color: theme.accent_brand };
    let repost = ActionLook { icon: LucideIcon::Repeat, count: post.reposts_count, is_active: post.is_reposted, active_color: repost_color };
    let quote = ActionLook { icon: LucideIcon::Edit, count: 0, is_active: false, active_color: theme.accent_dialectic };
    div()
        .w_full()
        .flex()
        .items_center()
        .gap(theme.spacing_lg)
        .pt(theme.spacing_xs)
        .child(render_action_btn(format!("{LIKE_BTN_ID_PREFIX}{id}"), like, theme, handlers.on_like.clone()))
        .child(render_action_btn(format!("{REPLY_BTN_ID_PREFIX}{id}"), reply, theme, handlers.on_reply.clone()))
        .child(render_action_btn(format!("{REPOST_BTN_ID_PREFIX}{id}"), repost, theme, handlers.on_repost.clone()))
        .child(render_action_btn(format!("{QUOTE_BTN_ID_PREFIX}{id}"), quote, theme, handlers.on_quote.clone()))
        .child(render_refs_button(post, refs, handlers.on_open_post.clone()))
        .child(render_share_button(post, author, theme))
}
