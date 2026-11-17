//! Numeric ranges of the preferences: the clamp applied on load and on every edit, and the
//! step the settings steppers move by.

/// Inclusive range of a numeric preference with its stepper increment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    pub min: i64,
    pub max: i64,
    pub step: i64,
}

impl Bounds {
    /// A range from `min` to `max` moved by `step`.
    pub const fn new(min: i64, max: i64, step: i64) -> Self {
        Self { min, max, step }
    }

    /// `value` pulled into the range.
    pub const fn clamp(self, value: i64) -> i64 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }

    /// `value` moved by `steps` increments (negative steps go down), kept in range.
    pub const fn stepped(self, value: i64, steps: i64) -> i64 {
        self.clamp(value.saturating_add(steps.saturating_mul(self.step)))
    }

    /// A `u32` preference pulled into the range.
    pub fn clamp_u32(self, value: u32) -> u32 {
        u32::try_from(self.clamp(i64::from(value))).unwrap_or_default()
    }
}

/// Hours of the day: `0..=23`.
pub const HOUR_OF_DAY: Bounds = Bounds::new(0, 23, 1);
