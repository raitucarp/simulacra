//! Small layout pieces shared by the forge screens: headings, cards, labeled controls.

use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::field_label;
use crate::ui::features::forge::widgets::ExplainerButton;
use gpui_kit::*;

/// Large bold title.
pub(super) fn title(text: impl Into<SharedString>, theme: &ThemeTokens) -> Div {
    div()
        .font_family(theme.font_family_display.clone())
        .text_size(theme.font_size_xl)
        .font_weight(FontWeight::BOLD)
        .text_color(theme.text_primary)
        .child(text.into())
}

/// Section heading inside a panel.
pub(super) fn heading(text: impl Into<SharedString>, theme: &ThemeTokens) -> Div {
    div()
        .text_size(theme.font_size_md)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.text_primary)
        .child(text.into())
}

/// Muted explanatory text.
pub(super) fn muted(text: impl Into<SharedString>, theme: &ThemeTokens) -> Div {
    div()
        .text_size(theme.font_size_sm)
        .text_color(theme.text_muted)
        .line_height(relative(theme.line_height_body))
        .child(text.into())
}

/// A rounded card surface.
pub(super) fn card(theme: &ThemeTokens) -> Div {
    div()
        .p(theme.card_padding)
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .rounded(theme.radius_xl)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
}

/// A field caption with its "?" explainer when one exists.
pub(super) fn field_caption(path: &str, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .child(
            div()
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_secondary)
                .child(field_label(path)),
        )
        .children(ExplainerButton::for_field(path, theme))
}

/// A field caption above a control.
pub(super) fn labeled(path: &str, control: impl IntoElement, theme: &ThemeTokens) -> Div {
    div().flex().flex_col().gap(theme.spacing_xs).child(field_caption(path, theme)).child(control)
}

/// Small colored message line (validation feedback, notices).
pub(super) fn message(text: impl Into<SharedString>, color: Rgba, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).text_color(color).child(text.into())
}
