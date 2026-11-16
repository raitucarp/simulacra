//! Tiny monospace step buttons (e.g. the hour steppers of the forge).

use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Tiny bordered monospace button (e.g. `-5`, `+1`).
pub fn mini_step_button(
    id: impl Into<ElementId>,
    label: &str,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    div()
        .id(id)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_xs)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, w, cx| on_click(w, cx))
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_secondary)
        .child(label.to_string())
}
