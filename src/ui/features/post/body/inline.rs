//! Inline pieces of the body flow: the prose text style, clickable links and trailing
//! punctuation kept with the element before it.

use super::OpenTextAction;
use crate::theme::ThemeTokens;
use gpui_kit::{div, relative, AnyElement, Div, InteractiveElement, IntoElement, MouseButton, ParentElement, Styled};

/// Makes an inline link dispatch `action(value)` on press without selecting the card.
pub(super) fn clickable(element: impl IntoElement, value: &str, action: Option<&OpenTextAction>) -> AnyElement {
    let Some(action) = action.cloned() else {
        return element.into_any_element();
    };
    let value = value.to_string();
    div()
        .child(element)
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            cx.stop_propagation();
            action(value.clone(), window, cx);
        })
        .into_any_element()
}

/// Base body prose text style.
pub(super) fn body_text(theme: &ThemeTokens) -> Div {
    div()
        .font_family(theme.font_family_body.clone())
        .text_size(theme.font_size_md)
        .text_color(theme.text_primary)
        .line_height(relative(theme.line_height_body))
}

/// Wraps an inline element with its trailing punctuation so they never wrap apart.
pub(super) fn with_trailing(element: AnyElement, trailing: &str, theme: &ThemeTokens) -> AnyElement {
    if trailing.is_empty() {
        return element;
    }
    div()
        .flex()
        .items_center()
        .gap_0()
        .child(element)
        .child(body_text(theme).child(trailing.to_string()))
        .into_any_element()
}
