//! Small inline badges: tinted meta badges and monospace code tags.

use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Text, fill and border colors of a [`meta_badge`].
#[derive(Debug, Clone, Copy)]
pub struct BadgeColors {
    pub text: Rgba,
    pub bg: Rgba,
    pub border: Rgba,
}

impl BadgeColors {
    pub const fn new(text: Rgba, bg: Rgba, border: Rgba) -> Self {
        Self { text, bg, border }
    }
}

/// Rounded tinted badge with semibold UI-font text (e.g. `INTJ-A · Architect`).
pub fn meta_badge(text: &str, colors: BadgeColors, theme: &ThemeTokens) -> Div {
    div()
        .px(theme.spacing_sm)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .bg(colors.bg)
        .border(theme.border_width_hairline)
        .border_color(colors.border)
        .font_family(theme.font_family_ui.clone())
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(colors.text)
        .child(text.to_string())
}

/// Tiny card-surface monospace tag (e.g. an MBTI code or a `{{placeholder}}` token).
/// Callers add `.font_weight(...)` when the tag needs emphasis.
pub fn mono_tag(text: impl Into<SharedString>, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_xs)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(color)
        .child(text.into())
}
