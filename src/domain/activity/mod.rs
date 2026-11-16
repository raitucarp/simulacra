//! Activity heatmap: posts of one network bucketed by local calendar day into a
//! weekday-aligned year grid (week columns ending at today, Sunday-first rows).

mod build;
mod levels;
mod months;
#[cfg(test)]
mod tests;

pub use build::local_date;
pub use months::MonthLabel;

use chrono::{Datelike, Duration, NaiveDate};

/// Number of week columns: one year plus the current, partial week.
pub const HEATMAP_WEEKS: usize = 53;
/// Rows per week column, Sunday first.
pub const DAYS_PER_WEEK: usize = 7;
/// Total number of grid cells.
pub const HEATMAP_CELLS: usize = HEATMAP_WEEKS * DAYS_PER_WEEK;
/// Highest intensity level; levels run `0..=MAX_ACTIVITY_LEVEL`.
pub const MAX_ACTIVITY_LEVEL: u8 = 4;

/// One visible (non-future) day of the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeatmapDay {
    pub date: NaiveDate,
    pub count: u32,
    pub level: u8,
}

/// Per-day post counts and intensity levels for the year ending today.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityHeatmap {
    start: NaiveDate,
    today: NaiveDate,
    counts: Vec<u32>,
    levels: Vec<u8>,
    total: usize,
}

/// Row of `date` within a Sunday-first week (Sunday = 0 … Saturday = 6).
#[inline]
pub fn weekday_row(date: NaiveDate) -> usize {
    date.weekday().num_days_from_sunday() as usize
}

impl ActivityHeatmap {
    /// First (Sunday) date of the grid.
    pub fn start(&self) -> NaiveDate {
        self.start
    }

    /// Last visible date of the grid.
    pub fn today(&self) -> NaiveDate {
        self.today
    }

    /// Number of posts inside the grid window.
    pub fn total(&self) -> usize {
        self.total
    }

    /// Date shown at `(week, row)`, whether or not it lies in the future.
    pub fn date_at(&self, week: usize, row: usize) -> NaiveDate {
        self.start + Duration::days((week * DAYS_PER_WEEK + row) as i64)
    }

    /// Visible day at `(week, row)`; `None` outside the grid or after today.
    pub fn day(&self, week: usize, row: usize) -> Option<HeatmapDay> {
        if week >= HEATMAP_WEEKS || row >= DAYS_PER_WEEK {
            return None;
        }
        let date = self.date_at(week, row);
        let idx = self.index_of(date)?;
        Some(HeatmapDay { date, count: self.counts[idx], level: self.levels[idx] })
    }

    /// Number of posts on `date` (zero outside the grid).
    pub fn count_on(&self, date: NaiveDate) -> u32 {
        self.index_of(date).map_or(0, |idx| self.counts[idx])
    }

    /// Cell index of `date`, when it lies between the grid start and today.
    pub fn index_of(&self, date: NaiveDate) -> Option<usize> {
        if date < self.start || date > self.today {
            return None;
        }
        Some((date - self.start).num_days() as usize)
    }
}
