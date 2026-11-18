//! When the weekly review reminder shows: on the chosen weekday, at or after the chosen
//! hour, once that day (the daily review reminder's rule, on one day of the week).

use chrono::{NaiveDate, Weekday};

/// The reminder's settings (Settings › Learning).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeeklyReminder {
    pub enabled: bool,
    pub weekday: Weekday,
    /// Local hour, 0–23.
    pub hour: u32,
}

/// Whether to remind now: `today` is the chosen weekday, the local `hour` has come, and the
/// reminder was not already shown today.
pub fn weekly_review_due(reminder: &WeeklyReminder, today: NaiveDate, today_weekday: Weekday, hour: u32, shown_on: Option<NaiveDate>) -> bool {
    reminder.enabled && today_weekday == reminder.weekday && hour >= reminder.hour && shown_on != Some(today)
}
