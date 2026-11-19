//! True retention over time: of the reviews of learnt cards in a week, the share recalled.
//! (Answers of new and relearning cards say little about long-term memory.)

use crate::domain::review::log::ReviewEntry;
use chrono::{Duration, NaiveDate};

/// Weeks the retention chart covers.
pub const RETENTION_WEEKS: usize = 12;
const DAYS_PER_WEEK: i64 = 7;

/// One week of the chart, oldest first.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RetentionPoint {
    /// First day of the week.
    pub start: NaiveDate,
    pub reviews: usize,
    /// Share recalled (0–1), when there were reviews.
    pub retention: Option<f64>,
}

/// The last `weeks` weeks ending with the week of `today`.
pub fn retention_by_week(log: &[ReviewEntry], today: NaiveDate, weeks: usize) -> Vec<RetentionPoint> {
    (0..weeks)
        .rev()
        .map(|back| {
            let end = today - Duration::days(DAYS_PER_WEEK * back as i64);
            let start = end - Duration::days(DAYS_PER_WEEK - 1);
            let reviews: Vec<&ReviewEntry> = log.iter().filter(|e| e.is_mature_review() && e.day >= start && e.day <= end).collect();
            let recalled = reviews.iter().filter(|e| e.grade.is_recall()).count();
            let retention = (!reviews.is_empty()).then(|| recalled as f64 / reviews.len() as f64);
            RetentionPoint { start, reviews: reviews.len(), retention }
        })
        .collect()
}
