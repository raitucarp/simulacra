//! Where chips and edges go on the argument canvas, in pixels relative to the canvas: a node
//! at fraction `(fx, fy)` puts its chip's top-left corner that far across the free area (the
//! canvas less its inset and one chip), so every chip stays inside whatever width it gets.

use crate::theme::ThemeTokens;
use gpui_kit::Pixels;

const HALF: f32 = 0.5;

/// The canvas and chip sizes, in pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub width: f32,
    pub height: f32,
    pub chip_width: f32,
    pub chip_height: f32,
    pub inset: f32,
}

impl Frame {
    /// The frame of a canvas `width` wide holding `rows` rows of chips.
    pub fn new(width: Pixels, rows: usize, theme: &ThemeTokens) -> Self {
        let width = width.as_f32();
        let inset = theme.panel_padding_compact.as_f32();
        let chip_width = theme.thinking_chip_width.as_f32().min((width - inset * 2.0).max(0.0));
        Self { width, height: canvas_height(rows, theme).as_f32(), chip_width, chip_height: theme.thinking_chip_height.as_f32(), inset }
    }

    fn free(&self) -> (f32, f32) {
        ((self.width - self.chip_width - self.inset * 2.0).max(0.0), (self.height - self.chip_height - self.inset * 2.0).max(0.0))
    }
}

/// Height of a canvas with `rows` rows (one row at least).
pub fn canvas_height(rows: usize, theme: &ThemeTokens) -> Pixels {
    theme.thinking_row_height * rows.max(1) as f32 + theme.panel_padding_compact * 2.0
}

/// How many chips fit side by side in `width` (one at least).
pub fn columns_for(width: Pixels, theme: &ThemeTokens) -> usize {
    let slot = theme.thinking_chip_width + theme.control_gap;
    (((width - theme.panel_padding_compact * 2.0 + theme.control_gap) / slot).floor() as usize).max(1)
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

/// The control point bending an edge between chips of one row, so it does not run through
/// the chips between them.
pub fn bend(a: (f32, f32), b: (f32, f32), amount: f32) -> (f32, f32) {
    let mid = ((a.0 + b.0) * HALF, (a.1 + b.1) * HALF);
    (mid.0 - (b.1 - a.1) * amount, mid.1 + (b.0 - a.0) * amount)
}
