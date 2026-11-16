//! Form field label plus single-line input / multi-line textarea wrappers.

use crate::theme::ThemeTokens;
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::*;

/// Small semibold caption placed above a form control.
pub fn field_label(label: &str, theme: &ThemeTokens) -> Div {
    div()
        .text_size(theme.font_size_xs)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.text_secondary)
        .child(label.to_string())
}

/// Vertical group: a [`field_label`] above an arbitrary control body.
pub fn labeled_group(label: &str, body: impl IntoElement, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(field_label(label, theme))
        .child(body)
}

/// Labeled, borderless single-line text input inside a tokenized frame.
pub fn labeled_input(label: &str, input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    labeled_frame(label, theme).child(
        field_frame(theme)
            .py(theme.spacing_3xs)
            .child(Input::new(input).bordered(false).cleanable(true)),
    )
}

/// Labeled, borderless multi-line textarea inside a tokenized frame.
pub fn labeled_textarea(
    label: &str,
    textarea: &Entity<TextareaState>,
    theme: &ThemeTokens,
) -> Div {
    labeled_frame(label, theme).child(
        field_frame(theme)
            .py(theme.spacing_2xs)
            .child(Textarea::new(textarea).bordered(false)),
    )
}

fn labeled_frame(label: &str, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(field_label(label, theme))
}

fn field_frame(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_app)
        .px(theme.spacing_xs)
}
