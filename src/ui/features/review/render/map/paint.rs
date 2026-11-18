//! Painting connections: one stroke per connection (straight, or curved for missing ones),
//! and a filled arrowhead where a one-way connection meets its target's chip.

use super::geometry::{bend, chip_edge, stroke_ends, Frame};
use super::Stroke;
use crate::theme::ThemeTokens;
use gpui_kit::{point, px, Bounds, PathBuilder, Pixels, Point, Window};

/// Arrowhead length and half-width, as multiples of the stroke width.
const ARROW_LENGTH: f32 = 4.0;
const ARROW_HALF_WIDTH: f32 = 2.2;

fn at(origin: Point<Pixels>, (x, y): (f32, f32)) -> Point<Pixels> {
    point(origin.x + px(x), origin.y + px(y))
}

/// The three corners of an arrowhead whose tip is `tip`, pointing away from `from`.
fn arrowhead(from: (f32, f32), tip: (f32, f32), width: f32) -> [(f32, f32); 3] {
    let (dx, dy) = (tip.0 - from.0, tip.1 - from.1);
    let length = (dx * dx + dy * dy).sqrt().max(f32::EPSILON);
    let (ux, uy) = (dx / length, dy / length);
    let base = (tip.0 - ux * width * ARROW_LENGTH, tip.1 - uy * width * ARROW_LENGTH);
    let side = (-uy * width * ARROW_HALF_WIDTH, ux * width * ARROW_HALF_WIDTH);
    [tip, (base.0 + side.0, base.1 + side.1), (base.0 - side.0, base.1 - side.1)]
}

fn paint_stroke(stroke: &Stroke, (a, b): ((f32, f32), (f32, f32)), frame: &Frame, origin: Point<Pixels>, width: f32, window: &mut Window) {
    let control = bend(a, b, frame.bend);
    let mut line = PathBuilder::stroke(px(width));
    line.move_to(at(origin, a));
    if stroke.curved {
        line.curve_to(at(origin, b), at(origin, control));
    } else {
        line.line_to(at(origin, b));
    }
    if let Ok(path) = line.build() {
        window.paint_path(path, stroke.color);
    }
    if !stroke.arrow {
        return;
    }
    let from = if stroke.curved { control } else { a };
    let corners: Vec<Point<Pixels>> = arrowhead(from, chip_edge(frame, from, b), width).into_iter().map(|corner| at(origin, corner)).collect();
    let mut head = PathBuilder::fill();
    head.add_polygon(&corners, true);
    if let Ok(path) = head.build() {
        window.paint_path(path, stroke.color);
    }
}

/// Paints every stroke into `bounds` (the canvas).
pub fn paint_strokes(strokes: &[Stroke], positions: &[(f32, f32)], bounds: Bounds<Pixels>, theme: &ThemeTokens, window: &mut Window) {
    let frame = Frame::new(bounds.size.width, theme);
    let width = theme.review_map_edge_width.as_f32();
    for stroke in strokes {
        if let Some(ends) = stroke_ends(&frame, positions, stroke) {
            paint_stroke(stroke, ends, &frame, bounds.origin, width, window);
        }
    }
}
