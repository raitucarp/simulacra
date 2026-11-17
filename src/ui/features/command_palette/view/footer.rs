//! Key hints under the palette results.

use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::model::COMMAND_PREFIX;
use crate::ui::keymap::{render_chord, render_key_cap};
use gpui_kit::*;
use rust_i18n::t;

/// (chords, locale key of what they do), in reading order.
const HINTS: [(&[&str], &str); 4] = [
    (&["up", "down"], "palette.hint.move"),
    (&["enter"], "palette.hint.open"),
    (&["tab"], "palette.hint.groups"),
    (&["escape"], "palette.hint.close"),
];

pub(super) fn render_footer(theme: &ThemeTokens) -> Div {
    let hints = HINTS.iter().map(|(chords, key)| render_hint(chords, key, theme));
    let commands = div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .child(render_key_cap(COMMAND_PREFIX.to_string(), theme))
        .child(t!("palette.hint.commands").to_string());
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(theme.control_gap)
        .px(theme.card_padding)
        .py(theme.spacing_xs_plus)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_card)
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .children(hints)
        .child(div().flex_1())
        .child(commands)
}

fn render_hint(chords: &[&str], key: &str, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .children(chords.iter().map(|chord| render_chord(chord, theme)))
        .child(t!(key).to_string())
}
