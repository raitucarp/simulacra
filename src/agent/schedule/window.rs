//! Hour-of-day windows used for active hours, cadence slots and chronotype peaks.

use std::cmp::Ordering;

/// Number of hours in a day; hours are taken modulo this value.
pub const HOURS_PER_DAY: u8 = 24;

/// A half-open window of local hours `[start, end)`.
///
/// - `start < end`: an ordinary daytime window (`7..23` is 07:00–22:59).
/// - `start > end`: the window wraps past midnight (`22..4` is 22:00–03:59).
/// - `start == end`: the whole day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HourWindow {
    pub start: u8,
    pub end: u8,
}

impl HourWindow {
    /// A window starting at `start` (inclusive) and ending at `end` (exclusive).
    pub const fn new(start: u8, end: u8) -> Self {
        Self { start, end }
    }

    /// Whether the local `hour` falls inside the window.
    pub fn contains(&self, hour: u8) -> bool {
        let start = self.start % HOURS_PER_DAY;
        let end = self.end % HOURS_PER_DAY;
        let hour = hour % HOURS_PER_DAY;
        match start.cmp(&end) {
            Ordering::Equal => true,
            Ordering::Less => (start..end).contains(&hour),
            Ordering::Greater => hour >= start || hour < end,
        }
    }
}
