//! Injectable wall clock so scheduling decisions are testable.

use chrono::{DateTime, FixedOffset, Local};

/// Source of the current local date-time (with its UTC offset).
pub trait Clock {
    /// The current local time.
    fn now(&self) -> DateTime<FixedOffset>;
}

/// The operating system's local clock.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<FixedOffset> {
        Local::now().fixed_offset()
    }
}

/// A clock frozen at a fixed instant (tests and replays).
#[derive(Debug, Clone, Copy)]
pub struct FixedClock(pub DateTime<FixedOffset>);

impl Clock for FixedClock {
    fn now(&self) -> DateTime<FixedOffset> {
        self.0
    }
}
