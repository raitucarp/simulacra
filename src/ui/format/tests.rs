use super::{stamp_at, Clock};
use crate::domain::preferences::{DateFormat, TimeFormat};
use crate::time::utc;
use jiff::tz::{self, TimeZone};
use jiff::ZonedDateTime;

const TWELVE: Clock = Clock { date: DateFormat::Iso, time: TimeFormat::TwelveHour };
const MONTH_FIRST: Clock = Clock { date: DateFormat::MonthDayYear, time: TimeFormat::TwentyFourHour };
const DAY_FIRST: Clock = Clock { date: DateFormat::DayMonthYear, time: TimeFormat::TwentyFourHour };

/// 2026-11-26 10:00 in UTC.
fn now_utc() -> ZonedDateTime {
    utc(2026, 11, 26, 10, 0, 0).to_zoned_datetime(TimeZone::UTC)
}

#[test]
fn today_shows_only_the_time() {
    rust_i18n::set_locale("en");
    assert_eq!(stamp_at(utc(2026, 11, 26, 8, 5, 0), &now_utc(), Clock::DEFAULT), "08:05");
    assert_eq!(stamp_at(utc(2026, 11, 26, 8, 5, 0), &now_utc(), TWELVE), "8:05 AM");
}

#[test]
fn yesterday_is_named() {
    rust_i18n::set_locale("en");
    assert_eq!(stamp_at(utc(2026, 11, 25, 23, 30, 0), &now_utc(), Clock::DEFAULT), "Yesterday 23:30");
}

#[test]
fn earlier_this_year_shows_day_and_month() {
    rust_i18n::set_locale("en");
    let at = utc(2026, 8, 12, 21, 30, 0);
    assert_eq!(stamp_at(at, &now_utc(), Clock::DEFAULT), "12 Aug");
    assert_eq!(stamp_at(at, &now_utc(), MONTH_FIRST), "Aug 12");
}

#[test]
fn another_year_shows_the_full_date_in_the_chosen_order() {
    rust_i18n::set_locale("en");
    let at = utc(2025, 12, 31, 9, 0, 0);
    assert_eq!(stamp_at(at, &now_utc(), Clock::DEFAULT), "2025-12-31");
    assert_eq!(stamp_at(at, &now_utc(), DAY_FIRST), "31/12/2025");
    assert_eq!(stamp_at(at, &now_utc(), MONTH_FIRST), "12/31/2025");
}

#[test]
fn days_are_counted_in_the_readers_time_zone() {
    rust_i18n::set_locale("en");
    let jakarta = utc(2026, 11, 26, 1, 0, 0).to_zoned_datetime(TimeZone::fixed(tz::offset(7)));
    // 23:30 UTC yesterday is 06:30 today in UTC+7.
    assert_eq!(stamp_at(utc(2026, 11, 25, 23, 30, 0), &jakarta, Clock::DEFAULT), "06:30");
}

#[test]
fn a_later_day_shows_its_date() {
    rust_i18n::set_locale("en");
    assert_eq!(stamp_at(utc(2026, 11, 28, 9, 0, 0), &now_utc(), Clock::DEFAULT), "28 Nov");
}
