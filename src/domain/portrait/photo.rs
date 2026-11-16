//! Circular crop of an imported avatar photo, stored in integer per-mille units so it is
//! exact, hashable and `Eq` (profiles compare configs by value).

use serde::{Deserialize, Serialize};

/// Per-mille units in one whole (1000 ‰ = 1.0).
pub const PER_MILLE: i32 = 1000;
/// Zoom bounds: 1× (photo just covers the circle) to 4×.
pub const ZOOM_MIN: u16 = 1000;
pub const ZOOM_MAX: u16 = 4000;
/// Zoom change of one slider step / scroll notch.
pub const ZOOM_STEP: u16 = 100;

/// Zoom and pan of a photo inside the avatar circle.
///
/// - `zoom`: photo box side relative to the circle diameter, in ‰ (`1000..=4000`).
/// - `offset_x` / `offset_y`: shift of the photo centre from the circle centre, in ‰ of the
///   diameter; limited so the photo always covers the circle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct PhotoCrop {
    pub zoom: u16,
    pub offset_x: i16,
    pub offset_y: i16,
}

impl Default for PhotoCrop {
    fn default() -> Self {
        Self { zoom: ZOOM_MIN, offset_x: 0, offset_y: 0 }
    }
}

/// Where to draw the photo box for a circle of a given diameter (same units as the diameter,
/// relative to the circle's top-left corner).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CropFrame {
    pub x: f32,
    pub y: f32,
    pub side: f32,
}

impl PhotoCrop {
    /// A crop with every field clamped into its valid range.
    pub fn new(zoom: u16, offset_x: i16, offset_y: i16) -> Self {
        Self { zoom, offset_x, offset_y }.clamped()
    }

    /// This crop with zoom in `ZOOM_MIN..=ZOOM_MAX` and offsets that keep the circle covered.
    pub fn clamped(self) -> Self {
        let zoom = self.zoom.clamp(ZOOM_MIN, ZOOM_MAX);
        let limit = max_offset(zoom);
        let clamp = |v: i16| (v as i32).clamp(-limit, limit) as i16;
        Self { zoom, offset_x: clamp(self.offset_x), offset_y: clamp(self.offset_y) }
    }

    /// This crop zoomed by `steps` notches (negative zooms out), re-clamped.
    pub fn zoomed(self, steps: i32) -> Self {
        let zoom = (self.zoom as i32 + steps * ZOOM_STEP as i32).clamp(ZOOM_MIN as i32, ZOOM_MAX as i32);
        Self { zoom: zoom as u16, ..self }.clamped()
    }

    /// This crop panned by (`dx`, `dy`) ‰ of the diameter, re-clamped.
    pub fn panned(self, dx: i32, dy: i32) -> Self {
        let shift = |v: i16, d: i32| (v as i32 + d).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        Self { offset_x: shift(self.offset_x, dx), offset_y: shift(self.offset_y, dy), ..self }.clamped()
    }

    /// The photo box for a circle of `diameter`.
    pub fn frame(self, diameter: f32) -> CropFrame {
        let crop = self.clamped();
        let unit = diameter / PER_MILLE as f32;
        let side = crop.zoom as f32 * unit;
        let origin = (diameter - side) / 2.0;
        CropFrame {
            x: origin + crop.offset_x as f32 * unit,
            y: origin + crop.offset_y as f32 * unit,
            side,
        }
    }
}

/// Largest offset (‰) that still keeps a photo of `zoom` covering the circle.
fn max_offset(zoom: u16) -> i32 {
    (zoom as i32 - PER_MILLE) / 2
}
