//! Post attachment embeds: local image and rich link preview card.

use super::consts::LINK_PREVIEW_ID_PREFIX;
use super::host::extract_host_domain;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, img, Div, FontWeight, InteractiveElement, ObjectFit, ParentElement, Stateful,
    StatefulInteractiveElement, Styled, StyledImage,
};
use std::path::PathBuf;

/// Local image attachment embed (zero synchronous filesystem syscalls).
pub(crate) fn render_image_embed(image_path: &str, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .max_h(theme.post_image_max_height)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .overflow_hidden()
        .child(
            img(PathBuf::from(image_path))
                .w_full()
                .h_full()
                .object_fit(ObjectFit::Cover),
        )
}

/// Rich link preview card opening the URL on click.
pub(crate) fn render_link_preview(post_id: &str, link_url: &str, theme: &ThemeTokens) -> Stateful<Div> {
    let url_to_open = link_url.to_string();
    let (hover_bg, hover_border) = (theme.hover_bg, theme.border_strong);
    div()
        .id(format!("{LINK_PREVIEW_ID_PREFIX}{post_id}"))
        .w_full()
        .p(theme.spacing_md)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .hover(move |s| s.bg(hover_bg).border_color(hover_border))
        .cursor_pointer()
        .on_click(move |_ev, _window, cx| {
            cx.open_url(&url_to_open);
        })
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(render_link_domain_row(link_url, theme))
        .child(
            div()
                .font_family(theme.font_family_ui.clone())
                .text_size(theme.font_size_sm)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_primary)
                .child(link_url.to_string()),
        )
}

fn render_link_domain_row(link_url: &str, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .child(
            div()
                .w(theme.font_size_xs)
                .h(theme.font_size_xs)
                .text_color(theme.text_muted)
                .child(Icon::new(LucideIcon::ExternalLink)),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.text_muted)
                .child(extract_host_domain(link_url)),
        )
}
