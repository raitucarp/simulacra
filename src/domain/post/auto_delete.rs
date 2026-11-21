//! Auto-delete (Y7): when a post deletes itself, chosen from a few delays, and the countdown
//! its card shows until then.

use jiff::{SignedDuration, Timestamp};

/// Seconds in a minute, minutes in an hour and hours in a day, for the countdown.
const SECONDS_PER_MINUTE: i64 = 60;
const MINUTES_PER_HOUR: i64 = 60;
const HOURS_PER_DAY: i64 = 24;
/// "In 30 days".
const MONTH_DAYS: i64 = 30;
/// "In a week".
const WEEK_DAYS: i64 = 7;

/// How long from now a post deletes itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteAfter {
    Hour,
    Day,
    Week,
    Month,
}

impl DeleteAfter {
    pub const ALL: [Self; 4] = [Self::Hour, Self::Day, Self::Week, Self::Month];

    /// Stable key (locale keys `data.auto_delete.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Hour => "hour",
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
        }
    }

    fn delay(self) -> SignedDuration {
        match self {
            Self::Hour => SignedDuration::from_hours(1),
            Self::Day => crate::time::days(1),
            Self::Week => crate::time::days(WEEK_DAYS),
            Self::Month => crate::time::days(MONTH_DAYS),
        }
    }

    /// The moment a post chosen to delete itself at `now` is deleted.
    pub fn delete_at(self, now: Timestamp) -> Timestamp {
        now.checked_add(self.delay()).unwrap_or(Timestamp::MAX)
    }
}

/// Time left before a post deletes itself, rounded up to the unit it is shown in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Countdown {
    Minutes(i64),
    Hours(i64),
    Days(i64),
}

impl Countdown {
    /// What is left from `now` until `at` (at least a minute while it has not passed).
    pub fn until(at: Timestamp, now: Timestamp) -> Self {
        let minutes = (at.duration_since(now).as_secs().max(0) + SECONDS_PER_MINUTE - 1) / SECONDS_PER_MINUTE;
        let hours = (minutes + MINUTES_PER_HOUR - 1) / MINUTES_PER_HOUR;
        match minutes {
            m if m < MINUTES_PER_HOUR => Self::Minutes(m.max(1)),
            _ if hours < HOURS_PER_DAY => Self::Hours(hours),
            _ => Self::Days((hours + HOURS_PER_DAY - 1) / HOURS_PER_DAY),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delays_and_countdowns_read_as_their_names_say() {
        let now = crate::time::utc(2026, 11, 22, 9, 0, 0);
        assert_eq!(DeleteAfter::Hour.delete_at(now), crate::time::utc(2026, 11, 22, 10, 0, 0));
        assert_eq!(DeleteAfter::Month.delete_at(now), crate::time::utc(2026, 12, 22, 9, 0, 0));
        let left = |secs: i64| Countdown::until(now + SignedDuration::from_secs(secs), now);
        assert_eq!(left(0), Countdown::Minutes(1), "never zero while it waits");
        assert_eq!(left(90), Countdown::Minutes(2));
        assert_eq!(left(3_600), Countdown::Hours(1));
        assert_eq!(left(5 * 3_600 + 1), Countdown::Hours(6));
        assert_eq!(left(24 * 3_600), Countdown::Days(1));
        assert_eq!(left(3 * 24 * 3_600 - 60), Countdown::Days(3));
    }
}
