//! The floating tooltip of canvas charts: a title and detail lines, placed beside the
//! pointer and flipped so it stays inside the chart.

use crate::theme::ThemeTokens;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::*;

/// Gap between the pointer and the tooltip.
const POINTER_GAP: f32 = 12.0;

/// Top-left corner (relative to the frame) of a `width` × `height` tooltip for a pointer at
/// window position `pointer`: right of and below it, flipped left or up near the edges.
pub fn placement(frame: &Frame, pointer: (f32, f32), (width, height): (f32, f32)) -> (f32, f32) {
    let (x, y) = (pointer.0 - frame.x, pointer.1 - frame.y);
    let left = if x + POINTER_GAP + width <= frame.width { x + POINTER_GAP } else { (x - POINTER_GAP - width).max(0.0) };
    let top = if y + POINTER_GAP + height <= frame.height { y + POINTER_GAP } else { (y - POINTER_GAP - height).max(0.0) };
    (left, top)
}

/// The tooltip card.
pub fn tooltip_card(title: String, lines: Vec<String>, theme: &ThemeTokens) -> Div {
    let detail = lines.into_iter().map(|line| div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(line));
    div()
        .w(theme.insights_tooltip_width)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.border_strong)
        .shadow(vec![BoxShadow::new(Pixels::ZERO, theme.shadow_offset_y, theme.shadow.into()).blur_radius(theme.shadow_blur)])
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title))
        .children(detail)
}

/// The tooltip placed over a chart of `frame` (call inside the chart's relative container).
/// `height` is an estimate used only to decide whether to flip upwards.
pub fn floating_tooltip(frame: &Frame, pointer: (f32, f32), card: Div, height: f32, theme: &ThemeTokens) -> Div {
    let (left, top) = placement(frame, pointer, (theme.insights_tooltip_width.as_f32(), height));
    div().absolute().left(px(left)).top(px(top)).child(card)
}
