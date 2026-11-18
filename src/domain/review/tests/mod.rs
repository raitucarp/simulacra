//! Unit tests of the review core.

mod ai;
mod connections;
mod items;
mod legacy;
mod seed;
mod fsrs;
mod queue;
mod quiz;
mod schedule;
mod session;
mod stats;

use super::item::{Anchor, AnchorSide, ItemId, ReviewItem, Subject};
use super::schedule::ItemSchedule;
use crate::domain::NetworkId;
use chrono::{NaiveDate, TimeZone, Utc};

/// A fixed review day.
pub(super) fn day(offset: i64) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 3, 1).unwrap() + chrono::Duration::days(offset)
}

/// An item `id` asking `front` (written) about a passage reading `back`, new on day 0.
pub(super) fn basic(id: &str, front: &str, back: &str) -> ReviewItem {
    ReviewItem {
        id: ItemId::new(id),
        network_id: NetworkId::new("net"),
        subject: Subject::Anchored { anchor: Anchor::passage(format!("doc_{id}"), None, back), side: AnchorSide::Back, written: front.into() },
        tags: Vec::new(),
        created_at: Utc.with_ymd_and_hms(2026, 3, 1, 9, 0, 0).unwrap(),
        schedule: ItemSchedule::new_item(day(0)),
        suspended: false,
        retired: false,
    }
}

/// No posts exist (answers come from passages and written faces).
pub(super) fn no_posts(_: &str) -> Option<String> {
    None
}
