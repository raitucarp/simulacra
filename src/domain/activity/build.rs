//! Bucketing posts into the heatmap grid.

use super::{levels, weekday_row, ActivityHeatmap, DAYS_PER_WEEK, HEATMAP_CELLS, HEATMAP_WEEKS};
use crate::domain::{NetworkId, Post};
use jiff::civil::Date;
use jiff::tz::TimeZone;

/// Calendar date of `post.created_at` in the time zone `tz`.
pub fn local_date(post: &Post, tz: &TimeZone) -> Date {
    post.created_at.to_zoned_datetime(tz.clone()).date()
}

/// Sunday that starts the first week column of a grid ending in the week of `today`.
fn grid_start(today: Date) -> Date {
    let back_days = weekday_row(today) + (HEATMAP_WEEKS - 1) * DAYS_PER_WEEK;
    today - crate::time::days(back_days as i64)
}

impl ActivityHeatmap {
    /// Buckets the posts of `network` by their local date in `tz` into the year ending `today`.
    pub fn build<'a>(
        posts: impl IntoIterator<Item = &'a Post>,
        network: &NetworkId,
        today: Date,
        tz: &TimeZone,
    ) -> Self {
        let start = grid_start(today);
        let mut heatmap = Self {
            start,
            today,
            counts: vec![0; HEATMAP_CELLS],
            levels: Vec::new(),
            total: 0,
        };
        for post in posts.into_iter().filter(|p| &p.network_id == network) {
            if let Some(idx) = heatmap.index_of(local_date(post, tz)) {
                heatmap.counts[idx] += 1;
                heatmap.total += 1;
            }
        }
        heatmap.levels = levels::quantile_levels(&heatmap.counts);
        heatmap
    }
}
