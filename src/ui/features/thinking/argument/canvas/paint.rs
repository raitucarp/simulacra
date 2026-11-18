//! Painting edges: one curve per edge (gently bent, more within a row), coloured by kind,
//! with a filled arrowhead where it meets its target's chip.

use super::geometry::{bend, chip_center, chip_edge, Frame};
use super::Stroke;
use crate::ui::features::insights::chart::paint::polygon;
use gpui_kit::{point, px, Bounds, PathBuilder, Pixels, Point, Window};

/// Arrowhead length and half-width, as multiples of the stroke width.
const ARROW_LENGTH: f32 = 4.0;
const ARROW_HALF_WIDTH: f32 = 2.2;
/// How far edges bend (share of their length), and how much more within one row.
const BEND: f32 = 0.08;
const SAME_ROW_BEND: f32 = 0.3;

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

fn paint_stroke(stroke: &Stroke, frame: &Frame, positions: &[(f32, f32)], origin: Point<Pixels>, width: f32, window: &mut Window) {
    let (Some(from), Some(to)) = (positions.get(stroke.from), positions.get(stroke.to)) else { return };
    let (a, b) = (chip_center(frame, *from), chip_center(frame, *to));
    let amount = if (from.1 - to.1).abs() < f32::EPSILON { SAME_ROW_BEND } else { BEND };
    let control = inside(frame, bend(a, b, amount)).unwrap_or_else(|| bend(b, a, amount));
    let (start, tip) = (chip_edge(frame, control, a), chip_edge(frame, control, b));
    let mut line = PathBuilder::stroke(px(width));
    line.move_to(at(origin, start));
    line.curve_to(at(origin, tip), at(origin, control));
    if let Ok(path) = line.build() {
        window.paint_path(path, stroke.colour);
    }
    let corners: Vec<(f32, f32)> = arrowhead(control, tip, width).iter().map(|(x, y)| (origin.x.as_f32() + x, origin.y.as_f32() + y)).collect();
    polygon(window, &corners, stroke.colour);
}

/// `control` when it lies inside the canvas (a bend may otherwise leave it and be cut).
fn inside(frame: &Frame, control: (f32, f32)) -> Option<(f32, f32)> {
    (frame.inset..=frame.height - frame.inset).contains(&control.1).then_some(control)
}

/// Paints every stroke into `bounds` (the canvas).
pub fn paint_strokes(strokes: &[Stroke], positions: &[(f32, f32)], frame: Frame, bounds: Bounds<Pixels>, width: f32, window: &mut Window) {
    strokes.iter().for_each(|stroke| paint_stroke(stroke, &frame, positions, bounds.origin, width, window));
}
