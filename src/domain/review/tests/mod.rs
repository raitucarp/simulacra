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
use jiff::civil::Date;

/// A fixed review day.
pub(super) fn day(offset: i64) -> Date {
    jiff::civil::date(2026, 3, 1) + crate::time::days(offset)
}

/// An item `id` asking `front` (written) about a passage reading `back`, new on day 0.
pub(super) fn basic(id: &str, front: &str, back: &str) -> ReviewItem {
    ReviewItem {
        id: ItemId::new(id),
        network_id: NetworkId::new("net"),
        subject: Subject::Anchored { anchor: Anchor::passage(format!("doc_{id}"), None, back), side: AnchorSide::Back, written: front.into() },
        tags: Vec::new(),
        created_at: crate::time::utc(2026, 3, 1, 9, 0, 0),
        schedule: ItemSchedule::new_item(day(0)),
        suspended: false,
        retired: false,
    }
}

/// No posts exist (answers come from passages and written faces).
pub(super) fn no_posts(_: &str) -> Option<String> {
    None
}
