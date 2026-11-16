//! Threads-style action bar (Heart, Reply, Repost, Quote, Refs, Share).

use super::consts::{
    LIKE_BTN_ID_PREFIX, QUOTE_BTN_ID_PREFIX, REFS_BTN_ID_PREFIX, REPLY_BTN_ID_PREFIX,
    REPOST_BTN_ID_PREFIX, SHARE_BTN_ID_PREFIX,
};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, App, ClickEvent, Div, ElementId, FontWeight, InteractiveElement, IntoElement,
    ParentElement, Rgba, Stateful, StatefulInteractiveElement, Styled, Window,
};

pub type ActionClickCallback = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// Click handlers of the interactive action buttons.
pub(crate) struct PostActionHandlers {
    pub on_like: ActionClickCallback,
    pub on_reply: ActionClickCallback,
    pub on_repost: ActionClickCallback,
    pub on_quote: ActionClickCallback,
}

/// Atomic Threads-style action button helper with tokenized typography and hover states.
fn render_action_btn(
    id: impl Into<ElementId>,
    icon_name: LucideIcon,
    count: usize,
    is_active: bool,
    active_color: Rgba,
    theme: &ThemeTokens,
    on_click: Option<ActionClickCallback>,
) -> impl IntoElement {
    let base_color = if is_active { active_color } else { theme.text_muted };
    let hover_bg = theme.hover_bg;

    let mut btn = div()
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
        .child(Icon::new(icon_name).size(theme.font_size_md));

    if count > 0 {
        btn = btn.child(count.to_string());
    }

    if let Some(cb) = on_click {
        btn.on_click(move |e, w, cx| cb(e, w, cx))
    } else {
        btn
    }
}

/// Full action bar for `post` wired to `handlers`.
pub(crate) fn render_action_bar(post: &Post, theme: &ThemeTokens, handlers: PostActionHandlers) -> Div {
    let id = &post.id;
    let like_color = if post.is_liked { theme.accent_brand } else { theme.text_secondary };
    let repost_color = if post.is_reposted { theme.accent_success } else { theme.text_secondary };
    let has_refs = !post.linked_posts.is_empty();
    div()
        .w_full()
        .flex()
        .items_center()
        .gap(theme.spacing_lg)
        .pt(theme.spacing_xs)
        .child(render_action_btn(format!("{LIKE_BTN_ID_PREFIX}{id}"), LucideIcon::Heart, post.likes, post.is_liked, like_color, theme, Some(handlers.on_like)))
        // Reply Button -> opens ComposerModal in Reply mode
        .child(render_action_btn(format!("{REPLY_BTN_ID_PREFIX}{id}"), LucideIcon::MessageSquare, post.replies_count, false, theme.accent_brand, theme, Some(handlers.on_reply)))
        // Repost Button -> triggers instant repost
        .child(render_action_btn(format!("{REPOST_BTN_ID_PREFIX}{id}"), LucideIcon::Repeat, post.reposts_count, post.is_reposted, repost_color, theme, Some(handlers.on_repost)))
        // Quote Button -> opens ComposerModal in Quote mode
        .child(render_action_btn(format!("{QUOTE_BTN_ID_PREFIX}{id}"), LucideIcon::Edit, 0, false, theme.accent_dialectic, theme, Some(handlers.on_quote)))
        // Refs Button -> shows count of linked thoughts
        .child(render_action_btn(format!("{REFS_BTN_ID_PREFIX}{id}"), LucideIcon::GitBranch, post.linked_posts.len(), has_refs, theme.accent_user, theme, None))
        .child(render_share_btn(post, theme))
}

fn render_share_btn(post: &Post, theme: &ThemeTokens) -> Stateful<Div> {
    let hover_color = theme.text_primary;
    div()
        .id(format!("{SHARE_BTN_ID_PREFIX}{}", post.id))
        .flex()
        .items_center()
        .py(theme.spacing_2xs)
        .cursor_pointer()
        .text_color(theme.text_muted)
        .hover(move |s| s.text_color(hover_color))
        .child(Icon::new(LucideIcon::Share2).size(theme.font_size_md))
}
