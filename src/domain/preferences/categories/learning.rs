//! Learning: spaced-repetition review limits, desired retention and the review reminder.

use super::super::bounds::{Bounds, HOUR_OF_DAY};
use super::super::choices::ReviewWeekday;
use serde::{Deserialize, Serialize};

/// Reviews per day.
pub const DAILY_REVIEWS: Bounds = Bounds::new(10, 2000, 10);
/// New cards introduced per day (never more than the review limit).
pub const NEW_CARDS: Bounds = Bounds::new(0, 500, 5);
/// Target recall probability, in percent.
pub const DESIRED_RETENTION: Bounds = Bounds::new(70, 99, 1);
const DEFAULT_DAILY_REVIEWS: u32 = 200;
const DEFAULT_NEW_CARDS: u32 = 20;
const DEFAULT_RETENTION: u32 = 90;
const DEFAULT_REMINDER_HOUR: u32 = 9;
const DEFAULT_WEEKLY_HOUR: u32 = 18;

/// Review preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LearningPrefs {
    /// Cards answered per day at most, new ones included (the Review page's queue).
    pub daily_review_limit: u32,
    /// New cards introduced per day (the Review page's queue).
    pub new_cards_per_day: u32,
    /// Percent: the recall probability FSRS schedules each card for.
    pub desired_retention: u32,
    /// A daily notification when cards are due.
    pub review_reminder: bool,
    /// Local hour of the reminder.
    pub reminder_hour: u32,
    /// A weekly notification to do the weekly review (sb E2).
    pub weekly_review_reminder: bool,
    /// The weekday of the weekly review.
    pub weekly_review_day: ReviewWeekday,
    /// Local hour of the weekly review reminder.
    pub weekly_review_hour: u32,
}

impl Default for LearningPrefs {
    fn default() -> Self {
        Self {
            daily_review_limit: DEFAULT_DAILY_REVIEWS,
            new_cards_per_day: DEFAULT_NEW_CARDS,
            desired_retention: DEFAULT_RETENTION,
            review_reminder: false,
            reminder_hour: DEFAULT_REMINDER_HOUR,
            weekly_review_reminder: false,
            weekly_review_day: ReviewWeekday::default(),
            weekly_review_hour: DEFAULT_WEEKLY_HOUR,
        }
    }
}

impl LearningPrefs {
    /// Clamps every value; new cards never exceed the daily review limit.
    pub fn validated(self) -> Self {
        let daily_review_limit = DAILY_REVIEWS.clamp_u32(self.daily_review_limit);
        Self {
            daily_review_limit,
            new_cards_per_day: NEW_CARDS.clamp_u32(self.new_cards_per_day).min(daily_review_limit),
            desired_retention: DESIRED_RETENTION.clamp_u32(self.desired_retention),
            reminder_hour: HOUR_OF_DAY.clamp_u32(self.reminder_hour),
            weekly_review_hour: HOUR_OF_DAY.clamp_u32(self.weekly_review_hour),
            ..self
        }
    }
}
