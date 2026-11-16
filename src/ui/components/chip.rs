//! Selectable choice chip and the wrapping row that lays chips out.

use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// A pill-shaped selectable option; highlighted in brand accent when selected.
pub fn choice_chip(
    id: impl Into<ElementId>,
    label: &str,
    is_selected: bool,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    div()
        .id(id)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .border(theme.border_width_hairline)
        .when(is_selected, |s| {
            s.bg(theme.accent_brand_subtle)
                .border_color(theme.accent_brand)
                .text_color(theme.text_primary)
        })
        .when(!is_selected, |s| {
            s.bg(theme.bg_app)
                .border_color(theme.border_subtle)
                .text_color(theme.text_secondary)
                .hover(move |h| h.bg(hover_bg))
        })
        .on_click(move |_, w, cx| on_click(w, cx))
        .text_size(theme.font_size_xs)
        .font_weight(if is_selected { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
        .child(label.to_string())
}

/// Wrapping horizontal row for chips, separated by the extra-small spacing token.
pub fn chip_row(theme: &ThemeTokens) -> Div {
    div().flex().flex_wrap().gap(theme.spacing_xs)
}
