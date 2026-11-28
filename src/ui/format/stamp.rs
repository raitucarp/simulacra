//! The short "when" on a post: only the time today, "Yesterday 14:41", the day and month
//! within the year, the full date before that (or for a post scheduled on a later day).

use super::{clock, Clock};
use crate::domain::preferences::DateFormat;
use crate::time::days_from_ce;
use jiff::{Timestamp, ZonedDateTime};
use rust_i18n::t;

/// The short "when" of a post written at `at`, read now in local time.
pub fn post_stamp(at: Timestamp) -> String {
    stamp_at(at, &crate::time::local_now(), clock())
}

/// The short "when" of `at`, read at `now` (in `now`'s time zone) with `clock`.
pub fn stamp_at(at: Timestamp, now: &ZonedDateTime, clock: Clock) -> String {
    let local = at.to_zoned_datetime(now.time_zone().clone());
    let time = local.strftime(clock.time_pattern()).to_string();
    match days_from_ce(now.date()) - days_from_ce(local.date()) {
        0 => time,
        1 => t!("format.yesterday_at", time = time).to_string(),
        _ if local.year() == now.year() => short_date(local.month(), local.day(), clock.date),
        _ => local.date().strftime(clock.date_pattern()).to_string(),
    }
}

/// "12 Aug" (or "Aug 12" with month-first dates) in the interface language.
fn short_date(month: i8, day: i8, order: DateFormat) -> String {
    let month_key = format!("cadence.month_{month}");
    let month = t!(&month_key).to_string();
    let key = if order == DateFormat::MonthDayYear { "format.month_day" } else { "format.day_month" };
    t!(key, month = month, day = day).to_string()
}
