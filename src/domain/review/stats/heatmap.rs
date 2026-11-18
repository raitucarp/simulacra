//! Review heatmap: answers per day over the last weeks, as week columns of seven days.

use crate::domain::review::log::ReviewEntry;
use chrono::{Datelike, Duration, NaiveDate};

/// Intensity levels above zero.
pub const HEATMAP_LEVELS: u8 = 4;
const DAYS_PER_WEEK: usize = 7;

/// One day of the heatmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeatmapCell {
    pub day: NaiveDate,
    pub count: usize,
    /// 0 (none) to [`HEATMAP_LEVELS`].
    pub level: u8,
    /// Whether the day is after today (drawn empty).
    pub future: bool,
}

/// The level of `count` against the busiest day `max` (quartiles of the busiest day).
pub fn heatmap_level(count: usize, max: usize) -> u8 {
    if count == 0 || max == 0 {
        return 0;
    }
    (((count * HEATMAP_LEVELS as usize) + max - 1) / max).clamp(1, HEATMAP_LEVELS as usize) as u8
}

/// `weeks` week columns ending with the week of `today` (each Monday first), oldest first.
pub fn review_heatmap(log: &[ReviewEntry], today: NaiveDate, weeks: usize) -> Vec<Vec<HeatmapCell>> {
    let monday = today - Duration::days(i64::from(today.weekday().num_days_from_monday()));
    let first = monday - Duration::days((DAYS_PER_WEEK * weeks.saturating_sub(1)) as i64);
    let count_on = |day: NaiveDate| log.iter().filter(|entry| entry.day == day).count();
    let counts: Vec<Vec<(NaiveDate, usize)>> = (0..weeks)
        .map(|week| {
            (0..DAYS_PER_WEEK)
                .map(|offset| first + Duration::days((week * DAYS_PER_WEEK + offset) as i64))
                .map(|day| (day, if day > today { 0 } else { count_on(day) }))
                .collect()
        })
        .collect();
    let max = counts.iter().flatten().map(|(_, count)| *count).max().unwrap_or(0);
    counts
        .into_iter()
        .map(|week| week.into_iter().map(|(day, count)| HeatmapCell { day, count, level: heatmap_level(count, max), future: day > today }).collect())
        .collect()
}

/// Consecutive days with at least one answer, ending today (or yesterday, when today has
/// none yet).
pub fn current_streak(log: &[ReviewEntry], today: NaiveDate) -> usize {
    let reviewed = |day: NaiveDate| log.iter().any(|entry| entry.day == day);
    let start = if reviewed(today) { today } else { today - Duration::days(1) };
    (0..).map(|back| start - Duration::days(back)).take_while(|day| reviewed(*day)).count()
}
