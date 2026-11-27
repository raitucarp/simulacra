//! How dates and times read on screen: always in local time, written the way the General
//! settings ask (date order, 12- or 24-hour clock).
//!
//! The settings live in a render-side global, set at startup and whenever they change, like
//! the interface language.

mod stamp;
#[cfg(test)]
mod tests;

pub use stamp::{post_stamp, stamp_at};

use crate::domain::preferences::{DateFormat, GeneralPrefs, TimeFormat};
use jiff::civil::Date;
use jiff::{Timestamp, ZonedDateTime};
use std::sync::RwLock;

/// The date and clock settings every on-screen date follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    pub date: DateFormat,
    pub time: TimeFormat,
}

impl Clock {
    /// ISO dates and a 24-hour clock (the settings' defaults).
    pub const DEFAULT: Self = Self { date: DateFormat::Iso, time: TimeFormat::TwentyFourHour };

    /// The date and clock settings among the General preferences.
    pub fn of(general: &GeneralPrefs) -> Self {
        Self { date: general.date_format, time: general.time_format }
    }

    /// The `strftime` pattern of a calendar date.
    pub const fn date_pattern(self) -> &'static str {
        match self.date {
            DateFormat::Iso => "%Y-%m-%d",
            DateFormat::DayMonthYear => "%d/%m/%Y",
            DateFormat::MonthDayYear => "%m/%d/%Y",
        }
    }

    /// The `strftime` pattern of a time of day.
    pub const fn time_pattern(self) -> &'static str {
        match self.time {
            TimeFormat::TwentyFourHour => "%H:%M",
            TimeFormat::TwelveHour => "%-I:%M %p",
        }
    }
}

static CLOCK: RwLock<Clock> = RwLock::new(Clock::DEFAULT);

/// Adopts the user's date and clock settings.
pub fn set_clock(clock: Clock) {
    *CLOCK.write().unwrap_or_else(|poisoned| poisoned.into_inner()) = clock;
}

/// The date and clock settings in effect.
pub fn clock() -> Clock {
    *CLOCK.read().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// `day` as the settings write a date.
pub fn date(day: Date) -> String {
    day.strftime(clock().date_pattern()).to_string()
}

/// The time of day of `at`, as the settings write it.
pub fn time_of_day(at: &ZonedDateTime) -> String {
    at.strftime(clock().time_pattern()).to_string()
}

/// `at` in local time as a full date and time ("2026-11-17 14:41").
pub fn date_time(at: Timestamp) -> String {
    let local = crate::time::local(at);
    format!("{} {}", date(local.date()), time_of_day(&local))
}
