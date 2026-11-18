//! Where chips and connections go, in pixels relative to the canvas: a point at fraction
//! `(fx, fy)` puts its chip's top-left corner that far across the free area (the canvas
//! less its inset and one chip), so every chip stays inside whatever width the map gets.

use super::Stroke;
use crate::theme::ThemeTokens;
use gpui_kit::Pixels;

const HALF: f32 = 0.5;

/// The canvas size and the chip box, in pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub width: f32,
    pub height: f32,
    pub chip_width: f32,
    pub chip_height: f32,
    pub inset: f32,
    pub label_width: f32,
    /// How far a curved stroke bends (share of its length).
    pub bend: f32,
}

impl Frame {
    pub fn new(width: Pixels, theme: &ThemeTokens) -> Self {
        let width = width.as_f32();
        let inset = theme.panel_padding_compact.as_f32();
        let chip_width = theme.review_map_point_width.as_f32().min((width - inset * 2.0) * HALF).max(0.0);
        Self { width, height: theme.review_map_height.as_f32(), chip_width, chip_height: theme.review_map_point_height.as_f32(), inset, label_width: theme.review_map_label_width.as_f32(), bend: theme.graph_curve_bend }
    }

    fn free(&self) -> (f32, f32) {
        ((self.width - self.chip_width - self.inset * 2.0).max(0.0), (self.height - self.chip_height - self.inset * 2.0).max(0.0))
    }
}

/// The top-left corner of the chip at `position`.
pub fn chip_origin(frame: &Frame, (fx, fy): (f32, f32)) -> (f32, f32) {
    let (free_x, free_y) = frame.free();
    (frame.inset + fx * free_x, frame.inset + fy * free_y)
}

/// The centre of the chip at `position`.
pub fn chip_center(frame: &Frame, position: (f32, f32)) -> (f32, f32) {
    let (x, y) = chip_origin(frame, position);
    (x + frame.chip_width * HALF, y + frame.chip_height * HALF)
}

/// Where a line from `from` towards the centre `to` meets the edge of the chip at `to`.
pub fn chip_edge(frame: &Frame, from: (f32, f32), to: (f32, f32)) -> (f32, f32) {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let reach_x = if dx.abs() > f32::EPSILON { frame.chip_width * HALF / dx.abs() } else { f32::INFINITY };
    let reach_y = if dy.abs() > f32::EPSILON { frame.chip_height * HALF / dy.abs() } else { f32::INFINITY };
    let back = reach_x.min(reach_y).min(1.0);
    (to.0 - dx * back, to.1 - dy * back)
}

/// The centres of a stroke's two chips.
pub fn stroke_ends(frame: &Frame, positions: &[(f32, f32)], stroke: &Stroke) -> Option<((f32, f32), (f32, f32))> {
    Some((chip_center(frame, *positions.get(stroke.from)?), chip_center(frame, *positions.get(stroke.to)?)))
}

/// The bend control point of a curved stroke from `a` to `b`.
pub fn bend(a: (f32, f32), b: (f32, f32), amount: f32) -> (f32, f32) {
    let mid = ((a.0 + b.0) * HALF, (a.1 + b.1) * HALF);
    (mid.0 - (b.1 - a.1) * amount, mid.1 + (b.0 - a.0) * amount)
}

/// The middle of a stroke (of its curve, for a curved one).
pub fn stroke_middle(frame: &Frame, a: (f32, f32), b: (f32, f32), curved: bool) -> (f32, f32) {
    let straight = ((a.0 + b.0) * HALF, (a.1 + b.1) * HALF);
    if !curved {
        return straight;
    }
    let control = bend(a, b, frame.bend);
    ((straight.0 + control.0) * HALF, (straight.1 + control.1) * HALF)
}

/// The top-left corner of a stroke's relation label (centred on the stroke's middle).
pub fn label_origin(frame: &Frame, positions: &[(f32, f32)], stroke: &Stroke) -> Option<(f32, f32)> {
    let (a, b) = stroke_ends(frame, positions, stroke)?;
    let middle = stroke_middle(frame, a, b, stroke.curved);
    Some((middle.0 - frame.label_width * HALF, middle.1 - frame.inset * HALF))
}
