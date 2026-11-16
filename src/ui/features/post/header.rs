//! Post header rows: repost banner and author identity/metadata row.

use super::consts::{META_SEPARATOR, ORIGINAL_MARKER_LABEL, REPOSTED_SUFFIX, TIME_FORMAT};
use super::controls::{render_overflow_menu, PostCardHandlers};
use crate::domain::{Post, ProfileRole};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_role_badge;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, FontWeight, ParentElement, Styled};

/// "<name> reposted" banner shown above a reposted thought.
pub(crate) fn render_repost_header(reposter_name: &str, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .pb(theme.spacing_2xs)
        .child(
            div()
                .w(theme.font_size_xs)
                .h(theme.font_size_xs)
                .text_color(theme.text_muted)
                .child(Icon::new(LucideIcon::Repeat)),
        )
        .child(
            div()
                .font_family(theme.font_family_ui.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.text_muted)
                .child(format!("{reposter_name} {REPOSTED_SUFFIX}")),
        )
}

/// Top row: author name, handle, role badge, timestamp, and the "…" overflow menu.
pub(crate) fn render_author_row(
    post: &Post,
    is_instant_repost: bool,
    theme: &ThemeTokens,
    handlers: &PostCardHandlers,
) -> Div {
    div()
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .child(render_author_meta(post, is_instant_repost, theme))
        .child(render_overflow_menu(post, handlers))
}

fn render_author_meta(post: &Post, is_instant_repost: bool, theme: &ThemeTokens) -> Div {
    let is_agent = matches!(post.author.role, ProfileRole::Agent { .. });
    let time_str = post.created_at.format(TIME_FORMAT).to_string();
    div()
        .flex()
        .items_center()
        .flex_wrap()
        .gap(theme.spacing_sm)
        // Author Display Name (prominent, crisp white)
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_size(theme.font_size_sm)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_primary)
                .child(post.author.name.clone()),
        )
        // Author Handle (secondary muted)
        .child(
            div()
                .font_family(theme.font_family_ui.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::NORMAL)
                .text_color(theme.text_muted)
                .child(post.author.handle.clone()),
        )
        .when(is_agent, |this| this.child(render_role_badge(&post.author.role, theme)))
        .child(mono_meta(theme, theme.font_size_xs).child(format!("{META_SEPARATOR} {time_str}")))
        .when(is_instant_repost, |this| {
            this.child(mono_meta(theme, theme.font_size_2xs).text_color(theme.accent_agent).child(ORIGINAL_MARKER_LABEL))
        })
}

/// Muted monospace metadata text at the given size.
fn mono_meta(theme: &ThemeTokens, size: gpui_kit::Pixels) -> Div {
    div()
        .font_family(theme.font_family_mono.clone())
        .text_size(size)
        .text_color(theme.text_muted)
}
