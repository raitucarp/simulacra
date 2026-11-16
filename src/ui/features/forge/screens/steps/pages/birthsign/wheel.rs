//! The Enneagram wheel: the circle, the inner triangle and hexad drawn on a canvas, and nine
//! clickable numbered points around it (the chosen core point filled in brand violet). Each
//! point is a tab stop picked with Enter / Space.

use super::enneagram::{wheel_offset, HEXAD, POINTS, TRIANGLE};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::widgets::{KeyboardStop, ValueCallback};
use std::rc::Rc;
use gpui_kit::*;

const WHEEL_ID: &str = "forge-enneagram-wheel";
const POINT_ID_PREFIX: &str = "forge-enneagram-point";
/// Half of a length.
const HALF: f32 = 0.5;

/// Centre and radius (to the point centres) of a wheel of `size` with points of `dot`.
fn layout(size: Pixels, dot: Pixels) -> (f32, f32) {
    (size.as_f32() * HALF, (size.as_f32() - dot.as_f32()) * HALF)
}

/// Where `point`'s centre sits inside a wheel of `size` with points of `dot`.
fn point_center(point: u8, size: Pixels, dot: Pixels) -> (f32, f32) {
    let (center, radius) = layout(size, dot);
    let (dx, dy) = wheel_offset(point);
    (center + dx * radius, center + dy * radius)
}

/// Strokes the triangle and the hexad of the Enneagram figure inside `bounds`.
fn paint_figure(bounds: Bounds<Pixels>, theme: &ThemeTokens, window: &mut Window) {
    let size = theme.forge_wheel_size;
    let dot = theme.forge_wheel_dot_size;
    let at = |point: u8| {
        let (x, y) = point_center(point, size, dot);
        bounds.origin + gpui_kit::point(px(x), px(y))
    };
    let mut figure = PathBuilder::stroke(theme.border_width_threadline);
    for path in [&TRIANGLE[..], &HEXAD[..]] {
        let mut points = path.iter().copied();
        if let Some(first) = points.next() {
            figure.move_to(at(first));
            points.for_each(|point| figure.line_to(at(point)));
        }
    }
    if let Ok(path) = figure.build() {
        window.paint_path(path, theme.border_strong);
    }
}

/// One numbered point; clicking it calls `on_pick` with its number.
fn point_button(point: u8, chosen: bool, theme: &ThemeTokens, on_pick: ValueCallback<u8>) -> Stateful<Div> {
    let dot = theme.forge_wheel_dot_size;
    let (x, y) = point_center(point, theme.forge_wheel_size, dot);
    let corner = dot.as_f32() * HALF;
    let (bg, fg, ring) = if chosen {
        (theme.accent_brand, theme.on_accent, theme.accent_brand)
    } else {
        (theme.bg_card, theme.text_primary, theme.border_strong)
    };
    let hover_ring = theme.accent_brand;
    let pick = on_pick.clone();
    div()
        .id((POINT_ID_PREFIX, usize::from(point)))
        .absolute()
        .left(px(x - corner))
        .top(px(y - corner))
        .size(dot)
        .flex()
        .items_center()
        .justify_center()
        .rounded(theme.radius_full)
        .border(theme.border_width_threadline)
        .border_color(ring)
        .bg(bg)
        .text_color(fg)
        .text_size(theme.font_size_sm)
        .font_weight(FontWeight::BOLD)
        .cursor_pointer()
        .hover(move |style| style.border_color(hover_ring))
        .on_click(move |_, window, cx| on_pick(point, window, cx))
        .keyboard_stop(theme, Rc::new(move |window, cx| pick(point, window, cx)))
        .child(point.to_string())
}

/// The wheel with `chosen` (1..=9) highlighted; picking a point calls `on_pick`.
pub(super) fn enneagram_wheel(chosen: u8, theme: &ThemeTokens, on_pick: ValueCallback<u8>) -> Stateful<Div> {
    let size = theme.forge_wheel_size;
    let inset = theme.forge_wheel_dot_size * HALF;
    let painter = theme.clone();
    let figure = canvas(|_, _, _| {}, move |bounds, (), window, _| paint_figure(bounds, &painter, window)).absolute().top_0().left_0().size_full();
    let ring = div()
        .absolute()
        .top(inset)
        .left(inset)
        .size(size - theme.forge_wheel_dot_size)
        .rounded(theme.radius_full)
        .border(theme.border_width_threadline)
        .border_color(theme.border_subtle);
    let points: Vec<Stateful<Div>> = (1..=POINTS).map(|point| point_button(point, point == chosen, theme, on_pick.clone())).collect();
    div().id(WHEEL_ID).relative().size(size).flex_shrink_0().child(ring).child(figure).children(points)
}
