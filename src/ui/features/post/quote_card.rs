//! Quoted thought card embed (or an unavailable notice when the quote is missing).

use super::body::render_post_body;
use super::consts::{META_SEPARATOR, QUOTE_UNAVAILABLE_PREFIX, QUOTE_UNAVAILABLE_SUFFIX, TIME_FORMAT};
use super::lookup::PostLookup;
use crate::domain::{Post, ProfileRole};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_avatar, render_role_badge};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, AnyElement, Div, FontWeight, InteractiveElement, IntoElement, ParentElement, Styled};

/// Quoted thought embed, resolving the quote through `lookup`.
pub(crate) fn render_quote_embed<L: PostLookup>(quote_id: &str, lookup: L, theme: &ThemeTokens) -> AnyElement {
    match lookup.find_post(quote_id) {
        Some(quoted) => render_quoted_card(quoted, lookup, theme).into_any_element(),
        None => render_quote_unavailable(quote_id, theme).into_any_element(),
    }
}

fn render_quoted_card<L: PostLookup>(quoted: &Post, lookup: L, theme: &ThemeTokens) -> Div {
    let (hover_bg, hover_border) = (theme.hover_bg, theme.border_strong);
    div()
        .w_full()
        .p(theme.spacing_md)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .hover(move |s| s.bg(hover_bg).border_color(hover_border))
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(render_quoted_meta(quoted, theme))
        .child(
            div()
                .font_family(theme.font_family_body.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.text_secondary)
                .child(render_post_body(&quoted.content, lookup, theme)),
        )
}

fn render_quoted_meta(quoted: &Post, theme: &ThemeTokens) -> Div {
    let quoted_time = quoted.created_at.format(TIME_FORMAT).to_string();
    let quoted_is_agent = matches!(quoted.author.role, ProfileRole::Agent { .. });
    let author = &quoted.author;
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(render_avatar(&author.avatar_initial, &author.role, theme.avatar_size_sm, theme))
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_primary)
                .child(author.name.clone()),
        )
        .child(small_meta(theme, theme.font_family_ui.clone()).child(author.handle.clone()))
        .when(quoted_is_agent, |this| this.child(render_role_badge(&author.role, theme)))
        .child(small_meta(theme, theme.font_family_mono.clone()).child(META_SEPARATOR))
        .child(small_meta(theme, theme.font_family_mono.clone()).child(quoted_time))
}

/// Muted extra-small metadata text in the given font family.
fn small_meta(theme: &ThemeTokens, family: String) -> Div {
    div()
        .font_family(family)
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
}

fn render_quote_unavailable(quote_id: &str, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .child(
            div()
                .w(theme.font_size_xs)
                .h(theme.font_size_xs)
                .text_color(theme.text_muted)
                .child(Icon::new(LucideIcon::Info)),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.text_muted)
                .child(format!("{QUOTE_UNAVAILABLE_PREFIX}{quote_id}{QUOTE_UNAVAILABLE_SUFFIX}")),
        )
}
