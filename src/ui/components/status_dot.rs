//! Tiny status indicators: a filled dot and a bold glyph bullet.

use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Filled-circle glyph used as an inline "active" marker.
pub const STATUS_GLYPH: &str = "●";

/// Small filled circle sized by the extra-small spacing token.
pub fn status_dot(color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .w(theme.spacing_xs)
        .h(theme.spacing_xs)
        .rounded(theme.radius_full)
        .bg(color)
}

/// Bold text glyph marker (`●`) in the given color.
pub fn status_glyph(color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::BOLD)
        .text_color(color)
        .child(STATUS_GLYPH)
}
