//! Shared building blocks for settings panes (section labels, cards, rows, notes).

use super::super::consts::{TEXT_BODY, TEXT_CAPTION, TEXT_SMALL};
use crate::theme::ThemeTokens;
use gpui_kit::{div, px, Div, FontWeight, IntoElement, ParentElement, SharedString, Styled};

/// Uppercase muted bold section heading.
pub(super) fn section_label(theme: &ThemeTokens, text: &'static str) -> Div {
    div()
        .text_size(px(TEXT_BODY))
        .font_weight(FontWeight::BOLD)
        .text_color(theme.text_muted)
        .child(text)
}

/// Vertical section: a header element stacked above a body element.
pub(super) fn section(header: impl IntoElement, body: impl IntoElement) -> Div {
    div().flex().flex_col().gap_2().child(header).child(body)
}

/// Bordered card surface laid out as a vertical stack (caller adds the gap).
pub(super) fn card(theme: &ThemeTokens) -> Div {
    div()
        .p_3p5()
        .rounded_lg()
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border)
        .flex()
        .flex_col()
}

/// Horizontal row with content pushed to both ends.
pub(super) fn spaced_row() -> Div {
    div().flex().items_center().justify_between()
}

/// Small muted explanatory paragraph.
pub(super) fn muted_note(theme: &ThemeTokens, text: &'static str) -> Div {
    div().text_size(px(TEXT_SMALL)).text_color(theme.text_muted).child(text)
}

/// Primary body-sized text line.
pub(super) fn body_text(theme: &ThemeTokens, text: &'static str) -> Div {
    div().text_size(px(TEXT_BODY)).text_color(theme.text_primary).child(text)
}

/// Medium-weight title over a secondary subtitle in the given font family.
pub(super) fn title_with_subtitle(
    theme: &ThemeTokens,
    title: &'static str,
    subtitle: &'static str,
    subtitle_family: impl Into<SharedString>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .gap_0p5()
        .child(field_title(theme, title))
        .child(
            div()
                .text_size(px(TEXT_SMALL))
                .font_family(subtitle_family)
                .text_color(theme.text_secondary)
                .child(subtitle),
        )
}

/// Medium-weight primary field title.
pub(super) fn field_title(theme: &ThemeTokens, text: impl IntoElement) -> Div {
    div()
        .text_size(px(TEXT_BODY))
        .font_weight(FontWeight::MEDIUM)
        .text_color(theme.text_primary)
        .child(text)
}

/// Field title on the left with a muted caption hint on the right.
pub(super) fn field_header(theme: &ThemeTokens, title: impl IntoElement, hint: &'static str) -> Div {
    spaced_row()
        .child(field_title(theme, title))
        .child(div().text_size(px(TEXT_CAPTION)).text_color(theme.text_muted).child(hint))
}

/// Vertical field group (label + input + extras).
pub(super) fn field_group() -> Div {
    div().flex().flex_col().gap_1p5()
}
