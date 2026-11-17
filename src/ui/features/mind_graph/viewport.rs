//! Pure pan/zoom math mapping unit-square layout points to canvas pixels and back.

use crate::domain::graph::layout::{LayoutPoint, LAYOUT_CENTER};
use gpui_kit::{Bounds, Pixels};

/// Zoom limits and wheel sensitivity (zoom factor per scrolled pixel, exponential).
pub(super) const ZOOM_MIN: f32 = 0.25;
pub(super) const ZOOM_MAX: f32 = 8.0;
pub(super) const ZOOM_PER_PIXEL: f32 = 0.002;
/// Divisor turning a size into its half.
const HALF: f32 = 2.0;
/// Smallest drawable fit size, so a collapsed canvas never divides by zero.
const MIN_FIT: f32 = 1.0;
/// Smallest layout span fitted to the frame, so one node (or a line) is not blown up.
const MIN_EXTENT: f32 = 0.5;

/// How the layout is fitted to the frame: the inner padding (pixels) and the size of the box
/// around the nodes (layout units), so the nodes, not the unit square, fill the canvas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Fit {
    pub padding: f32,
    pub extent: (f32, f32),
}

impl Fit {
    pub(crate) fn new(padding: f32, extent: (f32, f32)) -> Self {
        Self { padding, extent }
    }
}

/// Canvas rectangle in window pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Frame {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl From<Bounds<Pixels>> for Frame {
    fn from(bounds: Bounds<Pixels>) -> Self {
        Self {
            x: bounds.origin.x.as_f32(),
            y: bounds.origin.y.as_f32(),
            width: bounds.size.width.as_f32(),
            height: bounds.size.height.as_f32(),
        }
    }
}

/// Pan offset (pixels) and zoom factor applied on top of the fitted layout.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Viewport {
    pub pan: (f32, f32),
    pub zoom: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self { pan: (0.0, 0.0), zoom: 1.0 }
    }
}

impl Viewport {
    /// Pixels per layout unit: the node box fitted inside the padded frame (aspect kept), zoomed.
    fn scale(&self, frame: &Frame, fit: Fit) -> f32 {
        let inner = |side: f32| (side - fit.padding * HALF).max(MIN_FIT);
        let (span_x, span_y) = (fit.extent.0.max(MIN_EXTENT), fit.extent.1.max(MIN_EXTENT));
        (inner(frame.width) / span_x).min(inner(frame.height) / span_y) * self.zoom
    }

    fn center(&self, frame: &Frame) -> (f32, f32) {
        (frame.x + frame.width / HALF + self.pan.0, frame.y + frame.height / HALF + self.pan.1)
    }

    /// Window position of a layout point.
    pub(crate) fn to_screen(self, frame: &Frame, fit: Fit, p: LayoutPoint) -> (f32, f32) {
        let scale = self.scale(frame, fit);
        let (cx, cy) = self.center(frame);
        (cx + (p.x - LAYOUT_CENTER) * scale, cy + (p.y - LAYOUT_CENTER) * scale)
    }

    /// Layout point under a window position.
    pub(crate) fn to_layout(self, frame: &Frame, fit: Fit, (sx, sy): (f32, f32)) -> LayoutPoint {
        let scale = self.scale(frame, fit);
        let (cx, cy) = self.center(frame);
        LayoutPoint { x: (sx - cx) / scale + LAYOUT_CENTER, y: (sy - cy) / scale + LAYOUT_CENTER }
    }

    /// Multiplies the zoom by `factor` (clamped) keeping the point under `cursor` fixed.
    pub(crate) fn zoom_at(&mut self, frame: &Frame, fit: Fit, cursor: (f32, f32), factor: f32) {
        let anchor = self.to_layout(frame, fit, cursor);
        self.zoom = (self.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
        let (sx, sy) = self.to_screen(frame, fit, anchor);
        self.pan.0 += cursor.0 - sx;
        self.pan.1 += cursor.1 - sy;
    }
}

/// Zoom factor for a vertical wheel delta in pixels (wheel up zooms in).
pub(super) fn wheel_factor(delta_y: f32) -> f32 {
    (delta_y * ZOOM_PER_PIXEL).exp()
}
