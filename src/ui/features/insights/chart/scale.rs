//! Scales mapping data to pixels: linear (values to a span), band (categories to slots with
//! gaps) and radial (angle and radius around a centre). Pure and unit-tested.

use std::f32::consts::{FRAC_PI_2, TAU};

/// Maps `domain` linearly onto `range` (clamped when asked).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearScale {
    pub domain: (f64, f64),
    pub range: (f32, f32),
}

impl LinearScale {
    pub fn new(domain: (f64, f64), range: (f32, f32)) -> Self {
        Self { domain, range }
    }

    /// Position of `value` in the range, clamped to the domain (the range start for a
    /// degenerate domain). With the unit range it is the share of the domain covered.
    pub fn share(&self, value: f64) -> f32 {
        let span = self.domain.1 - self.domain.0;
        let t = if span.abs() < f64::EPSILON { 0.0 } else { (((value - self.domain.0) / span) as f32).clamp(0.0, 1.0) };
        self.range.0 + (self.range.1 - self.range.0) * t
    }
}

/// `count` equal slots over `length` pixels with `gap` between them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BandScale {
    pub count: usize,
    pub length: f32,
    pub gap: f32,
}

impl BandScale {
    /// Width of one slot (never negative).
    pub fn bandwidth(&self) -> f32 {
        let count = self.count.max(1) as f32;
        ((self.length - self.gap * (count - 1.0)) / count).max(0.0)
    }
}

/// Polar placement: `slots` equal angles starting at twelve o'clock, clockwise.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadialScale {
    pub centre: (f32, f32),
    pub slots: usize,
}

impl RadialScale {
    /// Angle (radians, screen coordinates) of the start of slot `index`.
    pub fn angle(&self, index: f32) -> f32 {
        index / self.slots.max(1) as f32 * TAU - FRAC_PI_2
    }

    /// Point at `radius` on the angle of slot position `index`.
    pub fn point(&self, index: f32, radius: f32) -> (f32, f32) {
        let a = self.angle(index);
        (self.centre.0 + radius * a.cos(), self.centre.1 + radius * a.sin())
    }

    /// Slot and distance from the centre of a position.
    pub fn locate(&self, (x, y): (f32, f32)) -> (usize, f32) {
        let (dx, dy) = (x - self.centre.0, y - self.centre.1);
        let turn = (dy.atan2(dx) + FRAC_PI_2).rem_euclid(TAU) / TAU;
        let slot = ((turn * self.slots as f32) as usize).min(self.slots.saturating_sub(1));
        (slot, (dx * dx + dy * dy).sqrt())
    }
}
