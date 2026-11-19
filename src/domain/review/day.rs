//! Review days: a learner's day ends at a rollover hour after midnight, so a late-night
//! session still counts towards the day it started in.

use jiff::civil::Date;
use jiff::ZonedDateTime;

/// Local hour a new review day starts.
pub const DAY_ROLLOVER_HOUR: i64 = 4;

/// The review day `at` falls in (local time, shifted by the rollover hour).
pub fn review_day(at: &ZonedDateTime) -> Date {
    (at.datetime() - jiff::SignedDuration::from_hours(DAY_ROLLOVER_HOUR)).date()
}

/// Whole days from `from` to `to` (zero when `to` is not later).
pub fn days_between(from: Date, to: Date) -> u32 {
    (to - from).get_days().max(0) as u32
}

/// `day` moved `days` forward.
pub fn add_days(day: Date, days: u32) -> Date {
    day + crate::time::days(i64::from(days))
}
