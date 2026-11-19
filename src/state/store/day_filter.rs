//! Timeline narrowing to one local calendar day, and the cadence heatmap it is picked from.

use super::AppState;
use crate::domain::activity::{local_date, ActivityHeatmap};
use crate::domain::Post;
use jiff::civil::Date;
use jiff::tz::TimeZone;

/// Keeps the posts whose local date in `tz` is `day` (all of them when `day` is `None`).
fn scope_to_day<'a>(
    posts: impl Iterator<Item = &'a Post> + 'a,
    day: Option<Date>,
    tz: TimeZone,
) -> impl Iterator<Item = &'a Post> + 'a {
    posts.filter(move |post| day.is_none_or(|day| local_date(post, &tz) == day))
}

impl AppState {
    /// Narrows the timeline to `day`, or clears the filter when it already shows `day`.
    /// Returns the filter now in effect.
    pub fn toggle_feed_day_filter(&mut self, day: Date) -> Option<Date> {
        self.feed_day_filter = if self.feed_day_filter == Some(day) { None } else { Some(day) };
        self.feed_day_filter
    }

    /// Removes the day filter.
    pub fn clear_feed_day_filter(&mut self) {
        self.feed_day_filter = None;
    }

    /// Narrows `posts` to the active day filter (local time); passes everything through
    /// when no day is selected. The feed applies this before grouping and tab filtering.
    pub fn day_scoped<'a>(&self, posts: impl Iterator<Item = &'a Post> + 'a) -> impl Iterator<Item = &'a Post> + 'a {
        scope_to_day(posts, self.feed_day_filter, TimeZone::system())
    }

    /// Current-network posts matching `query` that were created on `day` in time zone `tz`.
    pub fn day_filtered_posts_in<'a>(
        &'a self,
        day: Date,
        query: &'a str,
        tz: TimeZone,
    ) -> impl Iterator<Item = &'a Post> + 'a {
        scope_to_day(self.filtered_posts(query), Some(day), tz)
    }

    /// Cadence heatmap of the active network for the year ending today (local time).
    pub fn current_activity_heatmap(&self) -> ActivityHeatmap {
        ActivityHeatmap::build(&self.posts, &self.active_network_id, crate::time::local_now().date(), &TimeZone::system())
    }
}
