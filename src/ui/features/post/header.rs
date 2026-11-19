//! Post header rows: repost banner and author identity/metadata row.

use super::consts::{META_SEPARATOR, TIME_FORMAT};
use super::controls::{render_learn_badge, render_overflow_menu, PostCardHandlers};
use crate::domain::{Post, Profile, ProfileRole};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_role_badge;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, FontWeight, ParentElement, Styled};
use rust_i18n::t;

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
                .child(t!("post.reposted_by", name = reposter_name).to_string()),
        )
}

/// Top row: author name, handle, role badge, timestamp, and the "…" overflow menu.
pub(crate) fn render_author_row(
    post: &Post,
    author: &Profile,
    is_instant_repost: bool,
    theme: &ThemeTokens,
    handlers: &PostCardHandlers,
) -> Div {
    div()
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_xs)
        .child(render_author_meta(post, author, is_instant_repost, theme))
        .children(handlers.data.as_ref().and_then(|data| crate::ui::features::data::render_edited_mark(data, theme))) // sb F2
        .children(handlers.learn.status.map(|status| render_learn_badge(status, theme)))
        .child(div().flex_none().child(render_overflow_menu(post, handlers)))
}

fn render_author_meta(post: &Post, author: &Profile, is_instant_repost: bool, theme: &ThemeTokens) -> Div {
    let is_agent = matches!(author.role, ProfileRole::Agent { .. });
    let time_str = post.created_at.format(TIME_FORMAT).to_string();
    // Wraps between whole items (the badge and time move to a second line in a narrow
    // column); a name or handle too long for a line of its own ends in an ellipsis.
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_wrap()
        .items_center()
        .gap_x(theme.spacing_sm)
        .gap_y(theme.spacing_2xs)
        .child(identity_text(author.name.clone(), theme).font_family(theme.font_family_display.clone()).text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary))
        .child(identity_text(author.handle.clone(), theme).text_size(theme.font_size_xs).font_weight(FontWeight::NORMAL).text_color(theme.text_muted))
        .when(is_agent, |this| this.child(div().flex_none().child(render_role_badge(&author.role, theme))))
        .child(mono_meta(theme, theme.font_size_xs).child(format!("{META_SEPARATOR} {time_str}")))
        .when(is_instant_repost, |this| {
            this.child(mono_meta(theme, theme.font_size_2xs).text_color(theme.accent_agent).child(t!("post.original_marker").to_string()))
        })
}

/// The author's name or handle: one line, cut with an ellipsis when it cannot fit.
fn identity_text(text: String, theme: &ThemeTokens) -> Div {
    div().min_w_0().truncate().font_family(theme.font_family_ui.clone()).child(text)
}

/// Muted monospace metadata text at the given size.
fn mono_meta(theme: &ThemeTokens, size: gpui_kit::Pixels) -> Div {
    div()
        .flex_shrink_0()
        .whitespace_nowrap()
        .font_family(theme.font_family_mono.clone())
        .text_size(size)
        .text_color(theme.text_muted)
}
