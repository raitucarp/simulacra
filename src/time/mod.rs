//! Time helpers on top of `jiff`: the strings Simulacra persists, whole-day durations and the
//! local clock.
//!
//! Stored formats are the ones the app wrote before it moved to `jiff`, kept byte for byte so
//! existing rows still read, sort and compare as text:
//! - SQLite timestamp columns: RFC 3339 with a `+00:00` offset and 0, 3, 6 or 9 fraction
//!   digits ([`rfc3339`]).
//! - Moments kept with the local offset they happened at (mood check-ins): RFC 3339 with that
//!   offset ([`offset_rfc3339`], and [`serde_offset`] for JSON, which writes `Z` for UTC).
//!
//! The parsers accept these forms and `jiff`'s own.

mod rfc3339;
pub mod serde_offset;
#[cfg(test)]
mod tests;

pub use rfc3339::{offset_rfc3339, parse_offset, parse_timestamp, rfc3339};

use jiff::civil::Date;
use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp, ZonedDateTime};

/// Seconds in a 24-hour day.
const SECONDS_PER_DAY: i64 = 86_400;

/// A duration of `n` 24-hour days.
pub const fn days(n: i64) -> SignedDuration {
    SignedDuration::from_secs(n * SECONDS_PER_DAY)
}

/// The current moment in the operating system's time zone.
pub fn local_now() -> ZonedDateTime {
    ZonedDateTime::now()
}

/// `at` in the operating system's time zone.
pub fn local(at: Timestamp) -> ZonedDateTime {
    at.to_zoned_datetime(TimeZone::system())
}

/// The calendar day of `at` in the operating system's time zone.
pub fn local_date(at: Timestamp) -> Date {
    local(at).date()
}

/// The calendar day of `at` in UTC.
pub fn utc_date(at: Timestamp) -> Date {
    at.to_zoned_datetime(TimeZone::UTC).date()
}

/// The instant at `hour:minute:second` UTC on `day`. Panics on an impossible time (fixtures).
#[track_caller]
pub fn at_utc(day: Date, hour: i8, minute: i8, second: i8) -> Timestamp {
    let civil = day.at(hour, minute, second, 0);
    civil.to_zoned_datetime(TimeZone::UTC).expect("a UTC date-time always exists").timestamp()
}

/// The instant at the given UTC calendar date and time. Panics on an impossible one (fixtures).
#[track_caller]
pub fn utc(year: i16, month: i8, day: i8, hour: i8, minute: i8, second: i8) -> Timestamp {
    at_utc(jiff::civil::date(year, month, day), hour, minute, second)
}

/// Days from the start of the Common Era, counting 0001-01-01 as day 1 (a stable day number).
pub fn days_from_ce(day: Date) -> i64 {
    i64::from((day - jiff::civil::date(1, 1, 1)).get_days()) + 1
}
