//! Choosing when a scheduled post goes out (Y3): quick presets, a typed local moment
//! (`2026-11-22 09:00`) and how such a moment is written back.

use jiff::civil::DateTime;
use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp, ZonedDateTime};

/// How a moment is typed and shown: local date and time to the minute.
pub const MOMENT_FORMAT: &str = "%Y-%m-%d %H:%M";
/// Accepted between the date and the time instead of a space.
const ISO_SEPARATOR: char = 'T';
const SPACE: &str = " ";
/// "Tomorrow morning" is at this local hour.
const MORNING_HOUR: i8 = 9;
/// "In a week" is this many days later, at the same time.
const WEEK_DAYS: i64 = 7;
const AN_HOUR: SignedDuration = SignedDuration::from_hours(1);

/// A one-click choice of moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulePreset {
    InAnHour,
    TomorrowMorning,
    InAWeek,
}

impl SchedulePreset {
    pub const ALL: [Self; 3] = [Self::InAnHour, Self::TomorrowMorning, Self::InAWeek];

    /// Stable key (element ids, locale keys `composer.schedule.preset.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::InAnHour => "in_an_hour",
            Self::TomorrowMorning => "tomorrow_morning",
            Self::InAWeek => "in_a_week",
        }
    }

    /// The moment this preset means at `now` (local time).
    pub fn moment(self, now: &ZonedDateTime) -> Option<ZonedDateTime> {
        match self {
            Self::InAnHour => now.checked_add(AN_HOUR).ok(),
            Self::TomorrowMorning => {
                let morning = now.date().tomorrow().ok()?.at(MORNING_HOUR, 0, 0, 0);
                morning.to_zoned_datetime(now.time_zone().clone()).ok()
            }
            Self::InAWeek => now.checked_add(crate::time::days(WEEK_DAYS)).ok(),
        }
    }
}

/// Why a typed moment cannot be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleProblem {
    /// Not a date and time in [`MOMENT_FORMAT`].
    Unreadable,
    /// The moment has already passed.
    NotInFuture,
}

/// The moment typed as `text`, read in `now`'s time zone; it must be after `now`.
pub fn parse_moment(text: &str, now: &ZonedDateTime) -> Result<Timestamp, ScheduleProblem> {
    let typed = text.trim().replacen(ISO_SEPARATOR, SPACE, 1);
    let civil = DateTime::strptime(MOMENT_FORMAT, typed).map_err(|_| ScheduleProblem::Unreadable)?;
    let at = civil.to_zoned_datetime(now.time_zone().clone()).map_err(|_| ScheduleProblem::Unreadable)?.timestamp();
    if at <= now.timestamp() {
        return Err(ScheduleProblem::NotInFuture);
    }
    Ok(at)
}

/// `at` written as it is typed, in time zone `tz`.
pub fn format_moment(at: Timestamp, tz: &TimeZone) -> String {
    at.to_zoned_datetime(tz.clone()).strftime(MOMENT_FORMAT).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> ZonedDateTime {
        crate::time::utc(2026, 11, 21, 22, 30, 15).to_zoned_datetime(TimeZone::UTC)
    }

    #[test]
    fn typed_moments_are_read_in_local_time_and_must_be_ahead() {
        let at = parse_moment(" 2026-11-22 09:00 ", &now()).unwrap();
        assert_eq!(at, crate::time::utc(2026, 11, 22, 9, 0, 0));
        assert_eq!(parse_moment("2026-11-22T09:00", &now()), Ok(at));
        assert_eq!(parse_moment("2026-11-21 22:30", &now()), Err(ScheduleProblem::NotInFuture));
        assert_eq!(parse_moment("tomorrow", &now()), Err(ScheduleProblem::Unreadable));
        assert_eq!(parse_moment("2026-13-01 09:00", &now()), Err(ScheduleProblem::Unreadable));
    }

    #[test]
    fn presets_land_where_their_names_say() {
        let at = |preset: SchedulePreset| format_moment(preset.moment(&now()).unwrap().timestamp(), &TimeZone::UTC);
        assert_eq!(at(SchedulePreset::InAnHour), "2026-11-21 23:30");
        assert_eq!(at(SchedulePreset::TomorrowMorning), "2026-11-22 09:00");
        assert_eq!(at(SchedulePreset::InAWeek), "2026-11-28 22:30");
        for preset in SchedulePreset::ALL {
            assert!(parse_moment(&at(preset), &now()).is_ok(), "{} reads back", preset.key());
        }
    }
}
