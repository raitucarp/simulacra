//! D3: due phrases as calendar days, around month ends, leap years and time zones.

use super::super::due::{local_day, resolve_due};
use super::super::*;
use jiff::civil::{date, Date};
use jiff::tz::{Offset, TimeZone};

/// Sunday 22 November 2026.
fn sunday() -> Date {
    date(2026, 11, 22)
}

fn due(phrase: &str, today: Date) -> Option<Date> {
    resolve_due(phrase, today)
}

#[test]
fn fixed_days_and_words() {
    assert_eq!(due("2026-12-01", sunday()), Some(date(2026, 12, 1)));
    assert_eq!(due("2026-02-30", sunday()), None);
    assert_eq!(due("today", sunday()), Some(sunday()));
    assert_eq!(due("TOMORROW", sunday()), Some(date(2026, 11, 23)));
    assert_eq!(due("tmrw", date(2026, 12, 31)), Some(date(2027, 1, 1)));
    for nonsense in ["someday", "", "next", "next banana", "+", "+3x", "13-40", "32nd", "0th"] {
        assert_eq!(due(nonsense, sunday()), None, "{nonsense}");
    }
}

#[test]
fn weekdays_count_from_the_day_written() {
    assert_eq!(due("fri", sunday()), Some(date(2026, 11, 27)));
    assert_eq!(due("Friday", sunday()), Some(date(2026, 11, 27)));
    assert_eq!(due("sun", sunday()), Some(sunday()), "the day itself when it is that weekday");
    assert_eq!(due("next sun", sunday()), Some(date(2026, 11, 29)));
    assert_eq!(due("next-fri", sunday()), Some(date(2026, 11, 27)));
    assert_eq!(due("thurs", date(2026, 12, 31)), Some(date(2026, 12, 31)));
    assert_eq!(due("fri", date(2026, 12, 31)), Some(date(2027, 1, 1)), "across the year end");
}

#[test]
fn next_week_month_and_year() {
    assert_eq!(due("next week", sunday()), Some(date(2026, 11, 23)));
    assert_eq!(due("next week", date(2026, 11, 23)), Some(date(2026, 11, 30)), "a Monday moves a whole week");
    assert_eq!(due("next month", date(2026, 1, 31)), Some(date(2026, 2, 1)));
    assert_eq!(due("next-month", date(2026, 12, 15)), Some(date(2027, 1, 1)));
    assert_eq!(due("next year", sunday()), Some(date(2027, 1, 1)));
    assert_eq!(due("eom", date(2028, 2, 3)), Some(date(2028, 2, 29)));
    assert_eq!(due("end-of-week", sunday()), Some(sunday()));
}

#[test]
fn relative_steps_clamp_at_month_ends() {
    assert_eq!(due("+3d", sunday()), Some(date(2026, 11, 25)));
    assert_eq!(due("+3", date(2026, 11, 29)), Some(date(2026, 12, 2)));
    assert_eq!(due("+2w", sunday()), Some(date(2026, 12, 6)));
    assert_eq!(due("+1m", date(2026, 1, 31)), Some(date(2026, 2, 28)));
    assert_eq!(due("+1m", date(2028, 1, 31)), Some(date(2028, 2, 29)), "leap year");
    assert_eq!(due("+1m", date(2026, 3, 31)), Some(date(2026, 4, 30)));
    assert_eq!(due("+1y", date(2028, 2, 29)), Some(date(2029, 2, 28)));
    assert_eq!(due("+99999d", sunday()), None, "too far is not a date");
}

#[test]
fn days_of_the_month_and_month_days() {
    assert_eq!(due("15th", sunday()), Some(date(2026, 12, 15)));
    assert_eq!(due("22nd", sunday()), Some(sunday()));
    assert_eq!(due("31st", date(2026, 11, 2)), Some(date(2026, 11, 30)), "a short month uses its last day");
    assert_eq!(due("31st", date(2026, 12, 31)), Some(date(2026, 12, 31)));
    assert_eq!(due("30th", date(2027, 1, 31)), Some(date(2027, 2, 28)));
    assert_eq!(due("12-01", sunday()), Some(date(2026, 12, 1)));
    assert_eq!(due("11-01", sunday()), Some(date(2027, 11, 1)), "a passed month-day is next year's");
    assert_eq!(due("02-29", sunday()), Some(date(2028, 2, 29)));
}

#[test]
fn the_day_written_depends_on_the_time_zone() {
    let late_utc = jiff::Timestamp::from_second(1_764_545_400).expect("instant"); // 2025-11-30 23:30 UTC
    let utc = local_day(late_utc, &TimeZone::UTC);
    let jakarta = local_day(late_utc, &TimeZone::fixed(Offset::constant(7)));
    let hawaii = local_day(late_utc, &TimeZone::fixed(Offset::constant(-10)));
    assert_eq!((utc, jakarta, hawaii), (date(2025, 11, 30), date(2025, 12, 1), date(2025, 11, 30)));
    assert_eq!(due("tomorrow", utc), Some(date(2025, 12, 1)));
    assert_eq!(due("tomorrow", jakarta), Some(date(2025, 12, 2)));
    assert_eq!(due("eom", utc), Some(date(2025, 11, 30)));
    assert_eq!(due("eom", jakarta), Some(date(2025, 12, 31)));
}

#[test]
fn next_and_the_word_after_it_are_one_due_token() {
    let item = parse_todos("[ ] plan the trip ^next month !").remove(0);
    assert_eq!(item.meta.due.as_deref(), Some("next month"));
    assert_eq!(item.text, "plan the trip");
    assert_eq!(item.due_date(sunday()), Some(date(2026, 12, 1)));
    let lone = parse_todos("[ ] ^next steps").remove(0);
    assert_eq!(lone.meta.due.as_deref(), Some("next"), "a word that does not complete the phrase stays text");
    assert_eq!(lone.text, "steps");
    assert_eq!(lone.due_date(sunday()), None);
}
