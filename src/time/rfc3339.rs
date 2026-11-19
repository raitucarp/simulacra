//! RFC 3339 text in the exact shape the app has always stored, and lenient parsing of it.

use jiff::fmt::temporal::Pieces;
use jiff::tz::{Offset, TimeZone};
use jiff::{Timestamp, ZonedDateTime};

/// Seconds in an hour and in a minute, for printing an offset.
const SECONDS_PER_HOUR: i32 = 3_600;
const SECONDS_PER_MINUTE: i32 = 60;
/// Nanoseconds in a millisecond and in a microsecond, for choosing the fraction width.
const NANOS_PER_MILLI: i32 = 1_000_000;
const NANOS_PER_MICRO: i32 = 1_000;

/// `at` in UTC as stored in SQLite: `2026-11-18T10:00:00.250+00:00`.
pub fn rfc3339(at: Timestamp) -> String {
    write(at, Offset::UTC, false)
}

/// `at` with its own UTC offset: `2026-11-18T17:00:00+07:00` (`+00:00` for UTC).
pub fn offset_rfc3339(at: &ZonedDateTime) -> String {
    write(at.timestamp(), at.offset(), false)
}

/// Prints `at` at `offset`; `zulu` writes a zero offset as `Z`.
pub(super) fn write(at: Timestamp, offset: Offset, zulu: bool) -> String {
    let civil = offset.to_datetime(at);
    let stamp = civil.strftime("%Y-%m-%dT%H:%M:%S");
    format!("{stamp}{}{}", fraction(civil.subsec_nanosecond()), offset_text(offset, zulu))
}

/// Fraction of a second in whole milli-, micro- or nanoseconds; nothing when it is zero.
fn fraction(nanos: i32) -> String {
    match nanos {
        0 => String::new(),
        n if n % NANOS_PER_MILLI == 0 => format!(".{:03}", n / NANOS_PER_MILLI),
        n if n % NANOS_PER_MICRO == 0 => format!(".{:06}", n / NANOS_PER_MICRO),
        n => format!(".{n:09}"),
    }
}

/// `+HH:MM` / `-HH:MM`, or `Z` for a zero offset when `zulu` is set.
fn offset_text(offset: Offset, zulu: bool) -> String {
    let seconds = offset.seconds();
    if zulu && seconds == 0 {
        return "Z".to_owned();
    }
    let sign = if seconds < 0 { '-' } else { '+' };
    let seconds = seconds.abs();
    let (hours, minutes) = (seconds / SECONDS_PER_HOUR, seconds % SECONDS_PER_HOUR / SECONDS_PER_MINUTE);
    format!("{sign}{hours:02}:{minutes:02}")
}

/// Parses an RFC 3339 / ISO 8601 instant with any offset (`Z`, `+00:00`, `+07:00`, ...).
pub fn parse_timestamp(raw: &str) -> Option<Timestamp> {
    raw.trim().parse().ok()
}

/// Parses an instant and keeps the offset it was written with (`Z` reads as UTC).
pub fn parse_offset(raw: &str) -> Option<ZonedDateTime> {
    let raw = raw.trim();
    let at: Timestamp = raw.parse().ok()?;
    let offset = Pieces::parse(raw).ok()?.to_numeric_offset().unwrap_or(Offset::UTC);
    Some(at.to_zoned_datetime(TimeZone::fixed(offset)))
}
