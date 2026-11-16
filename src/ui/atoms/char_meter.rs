//! Atomic character counter meter with threshold color changes.

use crate::domain::MAX_POST_CHARS;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use rust_i18n::t;

/// Remaining-character margin at which the meter switches to the warning color.
const CHAR_WARNING_MARGIN: usize = 20;

/// Renders a character meter indicator (e.g. `120 / 280`).
pub fn render_char_meter(current: usize, theme: &ThemeTokens) -> impl IntoElement {
    let color = if current > MAX_POST_CHARS {
        theme.accent_danger // Danger red for exceeded character limit
    } else if current >= MAX_POST_CHARS - CHAR_WARNING_MARGIN {
        theme.accent_alt // Amber warning near character limit
    } else {
        theme.text_muted
    };

    div()
        .text_size(theme.font_size_label)
        .text_color(color)
        .child(t!("composer.char_counter", count = current, max = MAX_POST_CHARS).to_string())
}
