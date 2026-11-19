//! Posts over time, stacked by kind. Every day of the span gets a bar while it fits
//! [`MAX_BUCKETS`]; longer spans group consecutive days into equal buckets.

use super::KindCounts;
use crate::domain::insights::facts::{PostFacts, PostKind};
use chrono::{Duration, NaiveDate};

/// Bars at most.
pub const MAX_BUCKETS: usize = 90;

/// One bar: the first day, how many days it covers and the posts by kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeBucket {
    pub start: NaiveDate,
    pub days: usize,
    pub counts: KindCounts,
}

impl TimeBucket {
    pub fn total(&self) -> usize {
        self.counts.iter().sum()
    }

    /// The last day covered.
    pub fn end(&self) -> NaiveDate {
        self.start + Duration::days(self.days as i64 - 1)
    }
}

/// Buckets from `start` (or the first post's day) to `today`, empty when there is no post.
pub fn timeline(posts: &[PostFacts], start: Option<NaiveDate>, today: NaiveDate) -> Vec<TimeBucket> {
    let first = posts.iter().map(|p| p.day).min();
    let Some(first) = start.or(first).map(|day| day.min(today)) else { return Vec::new() };
    let span = (today - first).num_days() as usize + 1;
    let width = span.div_ceil(MAX_BUCKETS).max(1);
    let count = span.div_ceil(width);
    let mut buckets: Vec<TimeBucket> = (0..count)
        .map(|i| TimeBucket { start: first + Duration::days((i * width) as i64), days: width, counts: [0; PostKind::COUNT] })
        .collect();
    for post in posts.iter().filter(|p| p.day >= first && p.day <= today) {
        let at = (post.day - first).num_days() as usize / width;
        if let Some(bucket) = buckets.get_mut(at) {
            bucket.counts[post.kind.index()] += 1;
        }
    }
    buckets
}
