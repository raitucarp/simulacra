//! Hue replacement in HSL, for colors a palette has no slot for (a violet derived from the
//! palette's info blue keeps that blue's saturation and lightness).

use gpui_kit::Rgba;

/// Degrees in a full turn of the hue circle.
const FULL_TURN: f32 = 360.0;
/// Degrees per sixth of the circle (one RGB sector).
const SECTOR: f32 = 60.0;

/// `color` with its HSL hue set to `hue` degrees; saturation, lightness and alpha stay.
pub fn with_hue(color: Rgba, hue: f32) -> Rgba {
    let (max, min) = (color.r.max(color.g).max(color.b), color.r.min(color.g).min(color.b));
    let lightness = (max + min) / 2.0;
    let chroma = max - min;
    let h = hue.rem_euclid(FULL_TURN) / SECTOR;
    let x = chroma * (1.0 - (h % 2.0 - 1.0).abs());
    let (r, g, b) = match h as u32 {
        0 => (chroma, x, 0.0),
        1 => (x, chroma, 0.0),
        2 => (0.0, chroma, x),
        3 => (0.0, x, chroma),
        4 => (x, 0.0, chroma),
        _ => (chroma, 0.0, x),
    };
    let m = lightness - chroma / 2.0;
    Rgba { r: r + m, g: g + m, b: b + m, a: color.a }
}
