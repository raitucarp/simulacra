//! Key-cap rendering shared by the palette, the menus and the shortcut sheet.

use super::display::chord_labels;
use crate::theme::ThemeTokens;
use gpui_kit::*;

/// One key cap.
pub fn render_key_cap(label: String, theme: &ThemeTokens) -> Div {
    div()
        .min_w(theme.key_cap_min_width)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .flex()
        .justify_center()
        .rounded(theme.radius_xs)
        .border(theme.border_width_hairline)
        .border_color(theme.border_strong)
        .bg(theme.bg_card)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_secondary)
        .child(label)
}

/// The caps of `chord` side by side.
pub fn render_chord(chord: &str, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_none()
        .items_center()
        .gap(theme.spacing_2xs)
        .children(chord_labels(chord).into_iter().map(|label| render_key_cap(label, theme)))
}

/// `chord` as plain text (`Ctrl+P`) for tooltips.
pub fn chord_text(chord: &str) -> String {
    chord_labels(chord).join(CHORD_TEXT_JOINER)
}

/// Joiner of the caps in [`chord_text`].
const CHORD_TEXT_JOINER: &str = "+";
