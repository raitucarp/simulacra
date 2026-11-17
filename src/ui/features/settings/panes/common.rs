//! Shared building blocks for settings panes (section labels, cards, rows, notes).

use crate::theme::ThemeTokens;
use gpui_kit::{div, Div, FontWeight, IntoElement, ParentElement, SharedString, Styled};

/// Uppercase muted bold section heading.
pub(in crate::ui::features::settings) fn section_label(theme: &ThemeTokens, text: impl Into<SharedString>) -> Div {
    div()
        .text_size(theme.font_size_label)
        .font_weight(FontWeight::BOLD)
        .text_color(theme.text_muted)
        .child(text.into())
}

/// Vertical section: a header element stacked above a body element.
pub(in crate::ui::features::settings) fn section(header: impl IntoElement, body: impl IntoElement, theme: &ThemeTokens) -> Div {
    div().flex().flex_col().gap(theme.spacing_sm).child(header).child(body)
}

/// Bordered card surface laid out as a vertical stack (caller adds the gap).
pub(in crate::ui::features::settings) fn card(theme: &ThemeTokens) -> Div {
    div()
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .flex()
        .flex_col()
}

/// Horizontal row with content pushed to both ends.
pub(in crate::ui::features::settings) fn spaced_row() -> Div {
    div().flex().items_center().justify_between()
}

/// Small muted explanatory paragraph.
pub(in crate::ui::features::settings) fn muted_note(theme: &ThemeTokens, text: impl Into<SharedString>) -> Div {
    div().text_size(theme.font_size_hint).text_color(theme.text_muted).child(text.into())
}

/// Primary body-sized text line.
pub(in crate::ui::features::settings) fn body_text(theme: &ThemeTokens, text: impl Into<SharedString>) -> Div {
    div().text_size(theme.font_size_label).text_color(theme.text_primary).child(text.into())
}

/// Medium-weight title over a secondary subtitle in the given font family.
pub(in crate::ui::features::settings) fn title_with_subtitle(
    theme: &ThemeTokens,
    title: impl IntoElement,
    subtitle: impl Into<SharedString>,
    subtitle_family: impl Into<SharedString>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(field_title(theme, title))
        .child(
            div()
                .text_size(theme.font_size_hint)
                .font_family(subtitle_family)
                .text_color(theme.text_secondary)
                .child(subtitle.into()),
        )
}

/// Medium-weight primary field title.
pub(in crate::ui::features::settings) fn field_title(theme: &ThemeTokens, text: impl IntoElement) -> Div {
    div()
        .text_size(theme.font_size_label)
        .font_weight(FontWeight::MEDIUM)
        .text_color(theme.text_primary)
        .child(text)
}

/// Field title on the left with a muted caption hint on the right.
pub(in crate::ui::features::settings) fn field_header(theme: &ThemeTokens, title: impl IntoElement, hint: String) -> Div {
    spaced_row()
        .child(field_title(theme, title))
        .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(hint))
}

/// Vertical field group (label + input + extras).
pub(in crate::ui::features::settings) fn field_group(theme: &ThemeTokens) -> Div {
    div().flex().flex_col().gap(theme.spacing_xs_plus)
}
