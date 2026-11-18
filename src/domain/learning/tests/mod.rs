//! Unit tests of the learning tools.

mod bloom;
mod gaps;
mod goals;
mod palace;
mod reply;
mod tutor;
mod weekly;

use super::goals::OwnPost;
use super::tutor::machine::SessionStart;
use super::tutor::{SessionId, TutorSession, TutorSource};
use crate::domain::review::item::{Anchor, AnchorSide, ItemId, ReviewItem, Subject};
use crate::domain::review::schedule::ItemSchedule;
use crate::domain::NetworkId;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

/// A fixed day.
pub(super) fn day(offset: i64) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 11, 2).unwrap() + chrono::Duration::days(offset)
}

/// A fixed moment on day `offset`.
pub(super) fn at(offset: i64) -> DateTime<Utc> {
    Utc.from_utc_datetime(&day(offset).and_hms_opt(10, 0, 0).unwrap())
}

pub(super) fn network() -> NetworkId {
    NetworkId::new("net")
}

/// A new session on post `post`.
pub(super) fn session_on(post: &str) -> TutorSession {
    let start = SessionStart { network_id: network(), source: TutorSource::Post { id: post.into() }, title: "Freedom".into(), material: "Freedom is understanding necessity.".into() };
    TutorSession::start(SessionId("tutor_1".into()), start, at(0))
}

/// An item on post `post` with question `question`, new on day 0.
pub(super) fn item_on_post(id: &str, post: &str, question: &str) -> ReviewItem {
    ReviewItem {
        id: ItemId::new(id),
        network_id: network(),
        subject: Subject::Anchored { anchor: Anchor::Post { id: post.into() }, side: AnchorSide::Back, written: question.into() },
        tags: Vec::new(),
        created_at: at(0),
        schedule: ItemSchedule::new_item(day(0)),
        suspended: false,
        retired: false,
    }
}

/// An own post written on day `offset`.
pub(super) fn own_post(id: &str, text: &str, offset: i64) -> OwnPost {
    OwnPost { id: id.into(), root_id: id.into(), text: text.into(), day: day(offset) }
}
