//! The stored strings match what the app wrote before `jiff` (captured from `chrono` 0.4), and
//! everything it wrote then still reads.

use super::*;
use jiff::tz::Offset;
use serde::{Deserialize, Serialize};

fn noon() -> Timestamp {
    utc(2025, 5, 1, 10, 0, 0)
}

fn plus_seven() -> TimeZone {
    TimeZone::fixed(Offset::from_hours(7).expect("valid offset"))
}

#[test]
fn rfc3339_matches_the_legacy_column_format() {
    let cases = [
        (0, "2025-05-01T10:00:00+00:00"),
        (250_000_000, "2025-05-01T10:00:00.250+00:00"),
        (123_456_000, "2025-05-01T10:00:00.123456+00:00"),
        (123_456_700, "2025-05-01T10:00:00.123456700+00:00"),
    ];
    for (nanos, legacy) in cases {
        let at = noon() + SignedDuration::from_nanos(nanos);
        assert_eq!(rfc3339(at), legacy);
        assert_eq!(parse_timestamp(legacy), Some(at), "{legacy} reads back");
    }
}

#[test]
fn parse_timestamp_reads_every_legacy_offset_form() {
    for raw in ["2025-05-01T10:00:00Z", "2025-05-01T10:00:00+00:00", "2025-05-01T17:00:00+07:00", " 2025-05-01T10:00:00.000Z "] {
        assert_eq!(parse_timestamp(raw), Some(noon()), "{raw}");
    }
    assert_eq!(parse_timestamp("not a time"), None);
}

#[test]
fn offset_moments_keep_their_offset() {
    let at = noon().to_zoned_datetime(plus_seven());
    assert_eq!(offset_rfc3339(&at), "2025-05-01T17:00:00+07:00");
    let back = parse_offset("2025-05-01T17:00:00+07:00").expect("parses");
    assert_eq!((back.timestamp(), back.hour(), back.offset()), (noon(), 17, at.offset()));
    assert_eq!(parse_offset("2025-05-01T10:00:00Z").map(|z| z.offset()), Some(Offset::UTC));
}

#[derive(Serialize, Deserialize)]
struct Moment {
    #[serde(with = "super::serde_offset")]
    at: ZonedDateTime,
    stamp: Timestamp,
}

#[test]
fn json_written_by_earlier_versions_still_loads() {
    let legacy = r#"{"at":"2025-05-01T17:00:00.250+07:00","stamp":"2025-05-01T10:00:00.123456700Z"}"#;
    let moment: Moment = serde_json::from_str(legacy).expect("legacy JSON loads");
    assert_eq!((moment.at.hour(), moment.at.offset().seconds()), (17, 7 * 3_600));
    assert_eq!(moment.stamp, noon() + SignedDuration::from_nanos(123_456_700));
    let written = serde_json::to_string(&moment).expect("serializes");
    assert!(written.contains(r#""at":"2025-05-01T17:00:00.250+07:00""#), "{written}");
}

#[test]
fn serde_offset_writes_utc_as_zulu() {
    let moment = Moment { at: noon().to_zoned_datetime(TimeZone::UTC), stamp: noon() };
    let written = serde_json::to_string(&moment).expect("serializes");
    assert_eq!(written, r#"{"at":"2025-05-01T10:00:00Z","stamp":"2025-05-01T10:00:00Z"}"#);
}

#[test]
fn whole_days_are_24_hours() {
    assert_eq!(days(2), SignedDuration::from_hours(48));
    assert_eq!(utc_date(noon()), jiff::civil::date(2025, 5, 1));
    assert_eq!(at_utc(jiff::civil::date(2025, 5, 1), 10, 0, 0), noon());
}

#[test]
fn day_numbers_count_from_the_common_era() {
    assert_eq!(days_from_ce(jiff::civil::date(1, 1, 1)), 1);
    assert_eq!(days_from_ce(jiff::civil::date(1970, 1, 1)), 719_163, "the Unix epoch, as chrono numbers it");
}
