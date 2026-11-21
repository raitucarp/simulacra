//! The post lifecycle heartbeat (Y2), driven by a fake clock.

use crate::agent::schedule::FixedClock;
use crate::domain::post::PostStatus;
use crate::domain::PostId;
use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};
use crate::storage::{Database, Vault};
use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp};

const HOUR: SignedDuration = SignedDuration::from_hours(1);

fn clock(at: Timestamp) -> FixedClock {
    FixedClock(at.to_zoned_datetime(TimeZone::UTC))
}

fn start() -> Timestamp {
    crate::time::utc(2026, 11, 21, 8, 0, 0)
}

fn schedule(state: &mut AppState, texts: &[&str], anchor: &ThreadAnchor, at: Timestamp) -> Vec<PostId> {
    let drafts: Vec<ThreadDraft> = texts.iter().map(|text| ThreadDraft::text(*text)).collect();
    state.schedule_thread(&drafts, anchor, at, start()).expect("scheduled")
}

#[test]
fn nothing_goes_out_before_its_moment() {
    let mut state = AppState::new_in_memory();
    schedule(&mut state, &["later"], &ThreadAnchor::default(), start() + HOUR);
    let report = state.lifecycle_heartbeat(&clock(start() + HOUR - SignedDuration::from_secs(1))).expect("tick");
    assert!(report.is_empty());
    assert_eq!(state.scheduled_posts().len(), 1);
}

#[test]
fn due_posts_are_published_at_the_tick_in_thread_order() {
    let mut state = AppState::new_in_memory();
    let soon = schedule(&mut state, &["first", "second"], &ThreadAnchor::default(), start() + HOUR);
    let later = schedule(&mut state, &["much later"], &ThreadAnchor::default(), start() + SignedDuration::from_hours(5));
    let tick = start() + HOUR + SignedDuration::from_secs(30);
    let report = state.lifecycle_heartbeat(&clock(tick)).expect("tick");
    assert_eq!(report.published, soon);
    assert_eq!(state.posts[0].id, soon[1], "the newest post leads the timeline");
    assert_eq!(state.posts[1].id, soon[0]);
    let first = state.get_post(&soon[0]).expect("published");
    assert_eq!((first.lifecycle.status, first.lifecycle.publish_at, first.created_at), (PostStatus::Published, None, tick));
    assert_eq!(first.replies_count, 1, "the second post counts as its reply");
    assert_eq!(state.index.len(), state.posts.len());
    let waiting: Vec<PostId> = state.scheduled_posts().iter().map(|p| p.id.clone()).collect();
    assert_eq!(waiting, later);
}

#[test]
fn a_scheduled_reply_counts_on_its_parent_only_once_published() {
    let mut state = AppState::new_in_memory();
    let parent = state.submit_post("a question").expect("parent");
    let anchor = ThreadAnchor { reply_to: Some(parent.clone()), quote_of: None };
    schedule(&mut state, &["an answer"], &anchor, start() + HOUR);
    assert_eq!(state.get_post(&parent).map(|p| p.replies_count), Some(0));
    state.lifecycle_heartbeat(&clock(start() + HOUR)).expect("tick");
    assert_eq!(state.get_post(&parent).map(|p| p.replies_count), Some(1));
}

#[test]
fn published_posts_are_saved_as_published() {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let ids = schedule(&mut state, &["Zanzibar at noon"], &ThreadAnchor::default(), start() + HOUR);
    state.lifecycle_heartbeat(&clock(start() + SignedDuration::from_hours(2))).expect("tick");
    let loaded = AppState::load_from_db(&vault).expect("reload");
    assert!(loaded.scheduled_posts().is_empty());
    assert!(loaded.get_post(&ids[0]).is_some_and(|p| p.lifecycle.status == PostStatus::Published));
    assert_eq!(vault.search_posts("zanzibar", 5).expect("search"), ids);
}
