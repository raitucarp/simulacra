//! Unit tests of the review core.

mod ai;
mod cards;
mod fsrs;
mod queue;
mod quiz;
mod schedule;
mod session;
mod stats;

use super::card::{Card, CardId, CardKind, CardSource};
use super::schedule::CardSchedule;
use crate::domain::NetworkId;
use chrono::{NaiveDate, TimeZone, Utc};

/// A fixed review day.
pub(super) fn day(offset: i64) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 3, 1).unwrap() + chrono::Duration::days(offset)
}

/// A basic card `id` asking `front` → `back`, new on day 0.
pub(super) fn basic(id: &str, front: &str, back: &str) -> Card {
    Card {
        id: CardId::new(id),
        network_id: NetworkId::new("net"),
        kind: CardKind::Basic { front: front.into(), back: back.into() },
        source: CardSource::Sentence { text: front.into() },
        tags: Vec::new(),
        created_at: Utc.with_ymd_and_hms(2026, 3, 1, 9, 0, 0).unwrap(),
        schedule: CardSchedule::new_card(day(0)),
        suspended: false,
    }
}
