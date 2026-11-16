//! Shared chrome for the sidebar's divided sections (grounding sources, members).

use crate::theme::ThemeTokens;
use gpui_kit::base::StyledExt;
use gpui_kit::{div, Div, ParentElement, Rgba, Styled};

/// Hairline-divided vertical section container.
pub(super) fn section_container(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .pt(theme.spacing_sm)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
}

/// Section title on the left with a monospaced count on the right.
pub(super) fn section_header(
    title: String,
    count: usize,
    count_color: Rgba,
    theme: &ThemeTokens,
) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .px(theme.spacing_xs)
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .font_bold()
                .text_color(theme.text_muted)
                .child(title),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_2xs)
                .text_color(count_color)
                .child(format!("{count}")),
        )
}
