//! Timeline narrowing to one local calendar day, and the cadence heatmap it is picked from.

use super::AppState;
use crate::domain::activity::{local_date, ActivityHeatmap};
use crate::domain::Post;
use chrono::{Local, NaiveDate, TimeZone};

impl AppState {
    /// Narrows the timeline to `day`, or clears the filter when it already shows `day`.
    /// Returns the filter now in effect.
    pub fn toggle_feed_day_filter(&mut self, day: NaiveDate) -> Option<NaiveDate> {
        self.feed_day_filter = if self.feed_day_filter == Some(day) { None } else { Some(day) };
        self.feed_day_filter
    }

    /// Removes the day filter.
    pub fn clear_feed_day_filter(&mut self) {
        self.feed_day_filter = None;
    }

    /// Current-network posts matching `query` that were created on the local date `day`.
    pub fn day_filtered_posts<'a>(&'a self, day: NaiveDate, query: &'a str) -> impl Iterator<Item = &'a Post> {
        self.day_filtered_posts_in(day, query, Local)
    }

    /// [`Self::day_filtered_posts`] with an explicit time zone.
    pub fn day_filtered_posts_in<'a, Tz: TimeZone + 'a>(
        &'a self,
        day: NaiveDate,
        query: &'a str,
        tz: Tz,
    ) -> impl Iterator<Item = &'a Post> {
        self.filtered_posts(query).filter(move |post| local_date(post, &tz) == day)
    }

    /// Cadence heatmap of the active network for the year ending today (local time).
    pub fn current_activity_heatmap(&self) -> ActivityHeatmap {
        ActivityHeatmap::build(&self.posts, &self.active_network_id, Local::now().date_naive(), &Local)
    }
}
