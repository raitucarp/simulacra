//! Pure geometry of the gap radar: six axes from twelve o'clock clockwise (the chart kit's
//! radial scale), a value (0–1) as a distance along its axis, and where the axis labels sit.

use crate::domain::learning::gaps::{DimensionScores, DIMENSIONS};
use crate::ui::features::insights::chart::scale::RadialScale;
use crate::ui::features::mind_graph::viewport::Frame;

/// Share of the half side kept for the axis labels.
const LABEL_BAND: f32 = 0.26;
/// Where labels sit between the radar's edge and the frame's, as a share of that gap.
const LABEL_REACH: f32 = 0.5;
/// Grid rings, as values.
pub const RINGS: [f64; 4] = [0.25, 0.5, 0.75, 1.0];

/// The radar's centre and radius in `frame`.
pub(crate) fn radar(frame: &Frame) -> ((f32, f32), f32) {
    let half = frame.width.min(frame.height) / 2.0;
    ((frame.x + frame.width / 2.0, frame.y + frame.height / 2.0), half * (1.0 - LABEL_BAND))
}

fn axes(centre: (f32, f32)) -> RadialScale {
    RadialScale { centre, slots: DIMENSIONS }
}

/// Point of `value` (0–1) on axis `axis`.
pub(crate) fn point(frame: &Frame, axis: usize, value: f64) -> (f32, f32) {
    let (centre, radius) = radar(frame);
    axes(centre).point(axis as f32, radius * value.clamp(0.0, 1.0) as f32)
}

/// The closed outline of `scores` (one corner per axis).
pub(crate) fn outline(frame: &Frame, scores: &DimensionScores) -> Vec<(f32, f32)> {
    scores.iter().enumerate().map(|(axis, value)| point(frame, axis, *value)).collect()
}

/// A grid ring at `value`.
pub(crate) fn ring(frame: &Frame, value: f64) -> Vec<(f32, f32)> {
    (0..DIMENSIONS).map(|axis| point(frame, axis, value)).collect()
}

/// Where the label of `axis` is centred.
pub(crate) fn label_anchor(frame: &Frame, axis: usize) -> (f32, f32) {
    let (centre, radius) = radar(frame);
    let half = frame.width.min(frame.height) / 2.0;
    axes(centre).point(axis as f32, radius + (half - radius) * LABEL_REACH)
}
