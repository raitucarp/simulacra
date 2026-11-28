//! Node sizing, colors, edge curvature and hit testing shared by the full and mini graphs.

use super::viewport::{Fit, Frame, Viewport};
use crate::domain::graph::NodeKind;
use crate::state::graph::GraphScene;
use crate::theme::ThemeTokens;
use gpui_kit::Rgba;

/// Divisor turning a sum of two coordinates into their midpoint.
const MIDPOINT_DIVISOR: f32 = 2.0;
/// Extra pixels around a node that still count as hitting it.
pub(super) const HIT_SLOP: f32 = 4.0;

/// Node radius grown with the square root of its degree, between `min` and `max`.
pub(crate) fn node_radius(degree: usize, max_degree: usize, (min, max): (f32, f32)) -> f32 {
    let share = (degree as f32 / max_degree.max(1) as f32).sqrt().clamp(0.0, 1.0);
    min + (max - min) * share
}

/// Fill color of a node kind.
pub(crate) fn node_color(kind: NodeKind, theme: &ThemeTokens) -> Rgba {
    match kind {
        NodeKind::Post => theme.accent_agent,
        NodeKind::Document => theme.accent_user,
        NodeKind::Hashtag => theme.accent_alt,
        NodeKind::Profile => theme.accent_success,
    }
}

/// `color` with its alpha multiplied by `opacity`.
pub(crate) fn faded(color: Rgba, opacity: f32) -> Rgba {
    Rgba { a: color.a * opacity, ..color }
}

/// Control point bending the edge `a → b` sideways by `bend` times its length.
pub(crate) fn control_point(a: (f32, f32), b: (f32, f32), bend: f32) -> (f32, f32) {
    let mid = ((a.0 + b.0) / MIDPOINT_DIVISOR, (a.1 + b.1) / MIDPOINT_DIVISOR);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    (mid.0 - dy * bend, mid.1 + dx * bend)
}

/// Highest degree in the scene (at least one).
pub(crate) fn max_degree(scene: &GraphScene) -> usize {
    scene.degrees.iter().copied().max().unwrap_or(0).max(1)
}

/// Index of the node drawn under `cursor`, preferring the closest one.
pub(super) fn hit_test(
    scene: &GraphScene,
    view: &Viewport,
    frame: &Frame,
    fit: Fit,
    radius_range: (f32, f32),
    cursor: (f32, f32),
) -> Option<usize> {
    let top = max_degree(scene);
    let mut best: Option<(usize, f32)> = None;
    for (idx, point) in scene.layout.positions().iter().enumerate() {
        let (x, y) = view.to_screen(frame, fit, *point);
        let distance = ((x - cursor.0).powi(2) + (y - cursor.1).powi(2)).sqrt();
        let reach = node_radius(scene.degrees[idx], top, radius_range) + HIT_SLOP;
        if distance <= reach && best.is_none_or(|(_, d)| distance < d) {
            best = Some((idx, distance));
        }
    }
    best.map(|(idx, _)| idx)
}
