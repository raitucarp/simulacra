//! An option chip with its preview art: the option card on top, the label below, outlined in
//! the brand accent while chosen.

use crate::theme::ThemeTokens;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// One chip; `on_click` picks it.
pub fn art_chip(
    id: impl Into<ElementId>,
    art: String,
    label: String,
    selected: bool,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let hover = theme.hover_bg;
    div()
        .id(id)
        .w(theme.image_ideas_chip_width)
        .p(theme.spacing_xs)
        .flex()
        .flex_col()
        .items_center()
        .gap(theme.spacing_3xs)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .cursor_pointer()
        .map(|chip| {
            if selected {
                chip.bg(theme.accent_brand_subtle).border_color(theme.accent_brand).text_color(theme.text_primary)
            } else {
                chip.border_color(theme.border_subtle).text_color(theme.text_secondary).hover(move |style| style.bg(hover))
            }
        })
        .on_click(move |_, window, cx| on_click(window, cx))
        .child(Illustration::asset(art, IllustrationKind::TraitCard).width(theme.image_ideas_chip_art_width))
        .child(
            div()
                .w_full()
                .text_center()
                .text_size(theme.font_size_2xs)
                .font_weight(if selected { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                .truncate()
                .child(label),
        )
}
