//! Pure geometry of the 24-hour clock ring: midnight at the top, hours running clockwise.

use crate::agent::schedule::HOURS_PER_DAY;
use std::f32::consts::TAU;

/// Share of each hour's arc left empty between neighboring segments.
const SEGMENT_GAP: f32 = 0.14;
/// Straight pieces approximating each arc of a segment.
const ARC_STEPS: u16 = 4;
const HALF: f32 = 2.0;

/// Angle (radians, clockwise from the top) of one hour of the ring.
fn hour_span() -> f32 {
    TAU / f32::from(HOURS_PER_DAY)
}

/// The point at `angle` (clockwise from the top) and `radius` around `center`.
fn polar((cx, cy): (f32, f32), angle: f32, radius: f32) -> (f32, f32) {
    (cx + radius * angle.sin(), cy - radius * angle.cos())
}

/// The hour under the point offset `(dx, dy)` from the ring's center (screen y grows down).
pub(super) fn hour_at(dx: f32, dy: f32) -> u8 {
    let angle = dx.atan2(-dy).rem_euclid(TAU);
    ((angle / hour_span()).floor() as u8).min(HOURS_PER_DAY - 1)
}

/// The outline of hour `hour`'s ring segment between radii `inner` and `outer`: the outer
/// arc clockwise, then the inner arc back.
pub(super) fn segment(hour: u8, center: (f32, f32), (inner, outer): (f32, f32)) -> Vec<(f32, f32)> {
    let span = hour_span();
    let start = span * f32::from(hour % HOURS_PER_DAY) + span * SEGMENT_GAP / HALF;
    let sweep = span * (1.0 - SEGMENT_GAP);
    let angle = |step: u16| start + sweep * f32::from(step) / f32::from(ARC_STEPS);
    let outer_arc = (0..=ARC_STEPS).map(|step| polar(center, angle(step), outer));
    let inner_arc = (0..=ARC_STEPS).rev().map(|step| polar(center, angle(step), inner));
    outer_arc.chain(inner_arc).collect()
}
