//! Injectable wall clock so scheduling decisions are testable.

use jiff::ZonedDateTime;

/// Source of the current local date-time (with its UTC offset).
pub trait Clock {
    /// The current local time.
    fn now(&self) -> ZonedDateTime;
}

/// The operating system's local clock.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> ZonedDateTime {
        crate::time::local_now()
    }
}

/// A clock frozen at a fixed instant (tests and replays).
#[derive(Debug, Clone)]
pub struct FixedClock(pub ZonedDateTime);

impl Clock for FixedClock {
    fn now(&self) -> ZonedDateTime {
        self.0.clone()
    }
}
