//! Full-width selectable list row with hover feedback.

use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// A clickable row: elevated with an accent border when selected, transparent with a
/// hover tint otherwise. Callers add layout, children and the click handler.
pub fn selectable_row(
    id: impl Into<ElementId>,
    selected: bool,
    selected_border: Rgba,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    div()
        .id(id)
        .w_full()
        .px(theme.spacing_sm)
        .py(theme.spacing_sm)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .border(theme.border_width_hairline)
        .when(selected, |s| s.bg(theme.bg_elevated).border_color(selected_border))
        .when(!selected, |s| {
            s.bg(theme.bg_transparent)
                .border_color(theme.bg_transparent)
                .hover(move |h| h.bg(hover_bg))
        })
}
