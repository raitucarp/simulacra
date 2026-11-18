//! The heat strip of the journal: words written per day over the last weeks, as week columns
//! of seven days (Monday first), levelled like the review heatmap.

use super::entry::JournalEntry;
use crate::domain::review::stats::{heatmap_level, HeatmapCell};
use chrono::{Datelike, Duration, NaiveDate};
use std::collections::BTreeMap;

const DAYS_PER_WEEK: usize = 7;

/// Words written per day.
pub fn words_by_day(entries: &[JournalEntry]) -> BTreeMap<NaiveDate, usize> {
    entries.iter().fold(BTreeMap::new(), |mut days, entry| {
        *days.entry(entry.day).or_default() += entry.word_count().max(1);
        days
    })
}

/// `weeks` week columns of `counts` ending with the week of `today`, oldest first.
pub fn heat_strip(counts: &BTreeMap<NaiveDate, usize>, today: NaiveDate, weeks: usize) -> Vec<Vec<HeatmapCell>> {
    let monday = today - Duration::days(i64::from(today.weekday().num_days_from_monday()));
    let first = monday - Duration::days((DAYS_PER_WEEK * weeks.saturating_sub(1)) as i64);
    let day_at = |week: usize, offset: usize| first + Duration::days((week * DAYS_PER_WEEK + offset) as i64);
    let count_on = |day: NaiveDate| if day > today { 0 } else { counts.get(&day).copied().unwrap_or(0) };
    let max = (0..weeks).flat_map(|w| (0..DAYS_PER_WEEK).map(move |o| (w, o))).map(|(w, o)| count_on(day_at(w, o))).max().unwrap_or(0);
    (0..weeks)
        .map(|week| {
            (0..DAYS_PER_WEEK)
                .map(|offset| {
                    let day = day_at(week, offset);
                    let count = count_on(day);
                    HeatmapCell { day, count, level: heatmap_level(count, max), future: day > today }
                })
                .collect()
        })
        .collect()
}
