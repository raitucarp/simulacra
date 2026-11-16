//! Titled card surface that groups related form controls.

use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Renders a card with a display-font title and muted subtitle header.
/// Callers append the card body with `.child(...)`.
pub fn section_card(title: &str, subtitle: &str, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .p(theme.spacing_md)
        .rounded(theme.radius_lg)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .child(section_header(title, subtitle, theme))
}

fn section_header(title: &str, subtitle: &str, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .pb(theme.spacing_xs)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child(title.to_string()),
        )
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .text_color(theme.text_muted)
                .child(subtitle.to_string()),
        )
}

/// Horizontal row whose children share the width (pair with [`fill`]).
pub fn split_row(gap: Pixels) -> Div {
    div().flex().gap(gap)
}

/// Wraps `child` in a cell that grows to fill its share of a [`split_row`].
pub fn fill(child: impl IntoElement) -> Div {
    div().flex_1().child(child)
}

/// Vertical stack of cards separated by the medium spacing token.
pub fn card_stack(theme: &ThemeTokens) -> Div {
    div().flex().flex_col().gap(theme.spacing_md)
}
