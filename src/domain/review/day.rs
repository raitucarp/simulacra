//! Review days: a learner's day ends at a rollover hour after midnight, so a late-night
//! session still counts towards the day it started in.

use chrono::{DateTime, Duration, NaiveDate, TimeZone};

/// Local hour a new review day starts.
pub const DAY_ROLLOVER_HOUR: i64 = 4;

/// The review day `at` falls in (local time, shifted by the rollover hour).
pub fn review_day<Tz: TimeZone>(at: &DateTime<Tz>) -> NaiveDate {
    (at.naive_local() - Duration::hours(DAY_ROLLOVER_HOUR)).date()
}

/// Whole days from `from` to `to` (zero when `to` is not later).
pub fn days_between(from: NaiveDate, to: NaiveDate) -> u32 {
    (to - from).num_days().max(0) as u32
}

/// `day` moved `days` forward.
pub fn add_days(day: NaiveDate, days: u32) -> NaiveDate {
    day + Duration::days(i64::from(days))
}
