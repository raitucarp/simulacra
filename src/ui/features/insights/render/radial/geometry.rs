//! Pure geometry of the rhythm dial and the trait radar: where rings, sectors and axes lie
//! in a square frame, and what the pointer is over.

use super::RINGS;
use crate::domain::insights::activity::HOURS;
use crate::domain::insights::facts::TRAIT_AXES;
use crate::ui::features::insights::chart::scale::RadialScale;
use crate::ui::features::mind_graph::viewport::Frame;

/// Agents drawn on the radar at most (one series colour each).
pub const RADAR_AGENTS: usize = 8;
/// The dial's hole, as a share of its outer radius.
const INNER_SHARE: f32 = 0.26;
/// Share of the half side kept for labels outside the dial and the radar.
const DIAL_LABEL_BAND: f32 = 0.12;
const RADAR_LABEL_BAND: f32 = 0.24;
/// Arc steps per hour sector.
const ARC_STEPS: usize = 4;
/// Top of a trait value.
pub const TRAIT_MAX: f32 = 100.0;
/// Pointer reach beyond the radar, as a share of its radius.
const RADAR_REACH: f32 = 1.15;

/// The rhythm dial in a frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dial {
    pub centre: (f32, f32),
    pub inner: f32,
    pub outer: f32,
    pub ring: f32,
}

fn centre_and_half(frame: &Frame) -> ((f32, f32), f32) {
    ((frame.x + frame.width / 2.0, frame.y + frame.height / 2.0), frame.width.min(frame.height) / 2.0)
}

pub fn dial(frame: &Frame) -> Dial {
    let (centre, half) = centre_and_half(frame);
    let outer = half * (1.0 - DIAL_LABEL_BAND);
    let inner = outer * INNER_SHARE;
    Dial { centre, inner, outer, ring: (outer - inner) / RINGS as f32 }
}

pub fn hours(dial: &Dial) -> RadialScale {
    RadialScale { centre: dial.centre, slots: HOURS }
}

/// The weekday ring and hour sector under `at`.
pub fn rhythm_cell_at(frame: &Frame, at: (f32, f32)) -> Option<(usize, usize)> {
    let dial = dial(frame);
    let (hour, distance) = hours(&dial).locate(at);
    (distance >= dial.inner && distance < dial.outer).then(|| (((distance - dial.inner) / dial.ring) as usize).min(RINGS - 1)).map(|day| (day, hour))
}

/// Outline of the cell of weekday ring `day` and hour `hour` (outer arc, then inner arc back).
pub fn sector(dial: &Dial, day: usize, hour: usize) -> Vec<(f32, f32)> {
    let scale = hours(dial);
    let (low, high) = (dial.inner + dial.ring * day as f32, dial.inner + dial.ring * (day + 1) as f32);
    let steps = (0..=ARC_STEPS).map(|s| hour as f32 + s as f32 / ARC_STEPS as f32);
    let outer = steps.clone().map(|a| scale.point(a, high));
    let inner = steps.rev().map(|a| scale.point(a, low));
    outer.chain(inner).collect()
}

/// The radar's centre and radius in a frame.
pub fn radar(frame: &Frame) -> ((f32, f32), f32) {
    let (centre, half) = centre_and_half(frame);
    (centre, half * (1.0 - RADAR_LABEL_BAND))
}

pub fn axes(centre: (f32, f32)) -> RadialScale {
    RadialScale { centre, slots: TRAIT_AXES }
}

/// Point of `value` (0–100) on axis `axis`.
pub fn radar_point(frame: &Frame, axis: usize, value: f32) -> (f32, f32) {
    let (centre, radius) = radar(frame);
    axes(centre).point(axis as f32, radius * (value / TRAIT_MAX).clamp(0.0, 1.0))
}

/// The axis closest in angle to `at`, when it is over the radar.
pub fn radar_axis_at(frame: &Frame, at: (f32, f32)) -> Option<usize> {
    let (centre, radius) = radar(frame);
    let halves = RadialScale { centre, slots: TRAIT_AXES * 2 };
    let (slot, distance) = halves.locate(at);
    (distance <= radius * RADAR_REACH).then_some(((slot + 1) / 2) % TRAIT_AXES)
}
