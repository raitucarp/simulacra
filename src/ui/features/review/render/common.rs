//! Building blocks of the Review page: cards, titles, captions, tiles and progress bars.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;

/// A surface card holding one block of the page.
pub fn surface(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .p(theme.card_padding)
        .rounded(theme.radius_lg)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
}

/// An icon and a title in the display face.
pub fn block_title(icon: LucideIcon, color: Rgba, title: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .child(Icon::new(icon).size(theme.icon_size_sm).text_color(color))
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title))
}

/// Secondary explanatory text.
pub fn caption(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_muted).child(text)
}

/// A big number over its label.
pub fn count_tile(value: String, label: String, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .min_w(theme.review_count_tile_width)
        .flex_1()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.bg_elevated)
        .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(color).child(value))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label))
}

/// A thin track filled to `share` (0–1) in `color`.
pub fn progress_bar(share: f64, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .h(theme.review_progress_height)
        .rounded(theme.radius_full)
        .bg(theme.border_subtle)
        .overflow_hidden()
        .child(div().h_full().w(relative(share.clamp(0.0, 1.0) as f32)).rounded(theme.radius_full).bg(color))
}

/// A small colored square of a chart legend, with its label and count.
pub fn legend_item(color: Rgba, label: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .child(div().size(theme.review_swatch_size).rounded(theme.radius_xs).bg(color))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label))
}

/// A row of controls that wraps on narrow windows.
pub fn control_row(theme: &ThemeTokens) -> Div {
    div().flex().flex_wrap().items_center().gap(theme.control_gap)
}
