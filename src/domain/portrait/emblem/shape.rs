//! Emblem silhouettes (one per Enneagram core type) as SVG path data.

use serde::{Deserialize, Serialize};
use std::f32::consts::{FRAC_PI_2, TAU};
use std::fmt::Write;

/// Inner / outer radius ratio of star silhouettes.
const STAR_INNER_RATIO: f32 = 0.55;
/// Points of the five- and eight-pointed stars.
const STAR_POINTS: usize = 5;
const BURST_POINTS: usize = 8;
/// Shield: half-width of the top edge, and where the curved flanks start (fractions of r).
const SHIELD_HALF_WIDTH: f32 = 0.8;
const SHIELD_SHOULDER: f32 = 0.7;

/// Outer silhouette of an emblem.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmblemShape {
    Square,
    #[default]
    Circle,
    Star,
    Diamond,
    Hexagon,
    Shield,
    Burst,
    Triangle,
    Octagon,
}

impl EmblemShape {
    /// Shapes indexed by Enneagram core type 1–9.
    pub const BY_CORE: [EmblemShape; 9] = [
        Self::Square, Self::Circle, Self::Star, Self::Diamond, Self::Hexagon,
        Self::Shield, Self::Burst, Self::Triangle, Self::Octagon,
    ];

    /// The silhouette of Enneagram core type `core` (1–9; anything else is a circle).
    pub fn for_core(core: u8) -> Self {
        (core as usize).checked_sub(1).and_then(|i| Self::BY_CORE.get(i).copied()).unwrap_or_default()
    }

    /// Closed SVG path of this silhouette centred on (`cx`, `cy`) with outer radius `r`.
    pub fn outline(self, cx: f32, cy: f32, r: f32) -> String {
        let star = |points: usize| star(cx, cy, r, r * STAR_INNER_RATIO, points);
        match self {
            Self::Circle => format!("M{:.1} {cy:.1}a{r:.1} {r:.1} 0 1 0 {:.1} 0a{r:.1} {r:.1} 0 1 0 {:.1} 0Z", cx - r, 2.0 * r, -2.0 * r),
            Self::Square => polygon(cx, cy, r, 4, TAU / 8.0),
            Self::Diamond => polygon(cx, cy, r, 4, 0.0),
            Self::Hexagon => polygon(cx, cy, r, 6, 0.0),
            Self::Triangle => polygon(cx, cy, r, 3, 0.0),
            Self::Octagon => polygon(cx, cy, r, 8, TAU / 16.0),
            Self::Star => star(STAR_POINTS),
            Self::Burst => star(BURST_POINTS),
            Self::Shield => shield(cx, cy, r),
        }
    }
}

/// Point at `radius` and `angle` (radians clockwise from 12 o'clock) around the centre.
fn point(cx: f32, cy: f32, radius: f32, angle: f32) -> (f32, f32) {
    let theta = angle - FRAC_PI_2;
    (cx + radius * theta.cos(), cy + radius * theta.sin())
}

/// Closed path through `points`.
fn path(points: impl Iterator<Item = (f32, f32)>) -> String {
    let mut out = String::new();
    for (i, (x, y)) in points.enumerate() {
        let _ = write!(out, "{}{x:.1} {y:.1}", if i == 0 { "M" } else { "L" });
    }
    out.push('Z');
    out
}

/// Regular polygon with `sides` vertices, the first at `start` radians from 12 o'clock.
fn polygon(cx: f32, cy: f32, r: f32, sides: usize, start: f32) -> String {
    path((0..sides).map(|i| point(cx, cy, r, start + TAU * i as f32 / sides as f32)))
}

/// Star alternating between `outer` and `inner` radii, a tip at 12 o'clock.
fn star(cx: f32, cy: f32, outer: f32, inner: f32, points: usize) -> String {
    let steps = points * 2;
    path((0..steps).map(|i| {
        let radius = if i % 2 == 0 { outer } else { inner };
        point(cx, cy, radius, TAU * i as f32 / steps as f32)
    }))
}

/// Heraldic shield: flat top, straight sides, curved flanks meeting in a point.
fn shield(cx: f32, cy: f32, r: f32) -> String {
    let (w, top, shoulder) = (r * SHIELD_HALF_WIDTH, cy - r * SHIELD_HALF_WIDTH, r * SHIELD_SHOULDER);
    format!(
        "M{:.1} {top:.1}L{:.1} {top:.1}L{:.1} {cy:.1}Q{:.1} {:.1} {cx:.1} {:.1}Q{:.1} {:.1} {:.1} {cy:.1}Z",
        cx - w, cx + w, cx + w, cx + w, cy + shoulder, cy + r, cx - w, cy + shoulder, cx - w,
    )
}
