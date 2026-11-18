//! The learning indicator of a post in review, from its item's schedule.

use crate::domain::review::day::days_between;
use crate::domain::review::ReviewItem;
use crate::ui::features::post::LearnStatus;
use chrono::NaiveDate;

/// Where `item` stands on `today`: paused, new, due (or overdue), or due in some days.
pub fn learn_status(item: &ReviewItem, today: NaiveDate) -> LearnStatus {
    match (item.suspended, item.schedule.is_new(), item.schedule.due <= today) {
        (true, _, _) => LearnStatus::Suspended,
        (_, true, _) => LearnStatus::New,
        (_, _, true) => LearnStatus::Due,
        _ => LearnStatus::Later(days_between(today, item.schedule.due)),
    }
}
