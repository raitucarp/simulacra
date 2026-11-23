//! Comprehensive Threads-style post item composition.

use super::actions::render_action_bar;
use super::checklist::render_post_content;
use super::consts::POST_ITEM_ID_PREFIX;
use super::controls::{collect_references, PostCardHandlers};
use super::extras::render_post_extras;
use super::header::{render_author_row, render_repost_header};
use super::lookup::PostLookup;
use super::quote_card::render_quote_embed;
use super::surface::card_surface;
use super::threadline::{render_avatar_column, ThreadlineStyle};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::features::media::render_post_media;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, AnyElement, Div, InteractiveElement, IntoElement, MouseButton, ParentElement, Styled};

/// Comprehensive Threads-style Post Molecule with tokenized layout, context-aware threadline,
/// attachment embeds (image & link), quote cards, repost header, click-to-focus inspection,
/// and the overflow / references / share controls.
pub fn render_post_item<L: PostLookup>(
    post: &Post,
    lookup: L,
    theme: &ThemeTokens,
    is_focused: bool,
    threadline_style: ThreadlineStyle,
    handlers: PostCardHandlers,
) -> AnyElement {
    let (effective_post, is_instant_repost) = resolve_effective_post(post, &lookup);
    if let Some(row) = super::feeling_row::render_feeling_row(effective_post, lookup.author_of(effective_post), theme, is_focused, &handlers) {
        return row.into_any_element(); // M3: a feeling is one distinct line
    }
    let surface = card_surface(effective_post.extras.tint, is_focused, theme);
    let hover_bg = surface.hover;
    let has_repost_header = post.repost_id.is_some();
    let on_select = handlers.on_select.clone();

    div()
        .id(format!("{POST_ITEM_ID_PREFIX}{}", post.id))
        .w_full()
        .px(theme.panel_padding)
        .bg(surface.background)
        .hover(move |s| s.bg(hover_bg))
        .border_b(theme.border_width_hairline)
        .when(surface.edge.is_some(), |card| card.border_l(theme.border_width_threadline))
        .border_color(surface.rule)
        .cursor_default()
        .on_mouse_down(MouseButton::Left, move |_ev, window, cx| on_select(window, cx))
        .flex()
        .items_start()
        .gap(theme.spacing_md)
        .child(render_avatar_column(effective_post, lookup.author_of(effective_post), threadline_style, has_repost_header, theme))
        .child(render_content_column(post, effective_post, is_instant_repost, lookup, theme, handlers))
        .into_any_element()
}

/// Resolves the original thought an instant repost wraps (same rule as the action target).
fn resolve_effective_post<'a, L: PostLookup>(post: &'a Post, lookup: &'a L) -> (&'a Post, bool) {
    if !post.is_instant_repost() {
        return (post, false);
    }
    match lookup.find_post(post.action_target_id()) {
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
    handlers: PostCardHandlers,
) -> Div {
    let refs = collect_references(effective, lookup);
    div()
        .flex_1()
        .min_w_0()
        .py(theme.spacing_md)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .when(post.repost_id.is_some(), |this| this.child(render_repost_header(&lookup.author_of(post).name, theme)))
        .child(render_author_row(effective, lookup.author_of(effective), is_instant_repost, theme, &handlers))
        .children(super::garden_badge::render_garden_badge(effective, theme)) // N3
        .children(super::anchor_line::render_anchor_line(effective, lookup, theme)) // N2: the passage a margin reply answers
        .child(
            div()
                .w_full()
                .min_w_0()
                .py(theme.spacing_2xs)
                .child(match handlers.lock.as_ref().filter(|_| effective.is_locked()) {
                    Some(lock) => super::locked::render_locked_body(lock, lookup, theme, &handlers.links.scoped(effective.id.to_string())), // X4: the placeholder, or the text unlocked this session
                    None => render_post_content(effective, lookup, theme, &handlers.links.scoped(effective.id.to_string()), super::checklist::todo_toggle(handlers.data.as_ref())), // D5, D6: its checklist, ticked on the user's own posts
                }),
        )
        .children(render_post_extras(&effective.extras, theme, handlers.links.on_source.as_ref()))
        .children(render_post_media(effective, lookup, theme, handlers.links.on_media.as_ref(), handlers.links.on_relink.as_ref()))
        .when_some(effective.quote_id.as_ref(), |this, quote_id| {
            this.child(render_quote_embed(quote_id.as_str(), lookup, handlers.on_open_post.clone(), &handlers.links, theme))
        })
        .children(handlers.reactions.as_ref().and_then(|controls| crate::ui::features::reactions::render_reaction_bar(&controls.post_id, &controls.chips, &controls.on_event, theme)))
        .child(render_action_bar(effective, lookup.author_of(effective), refs, theme, &handlers))
}
