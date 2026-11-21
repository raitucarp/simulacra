//! The scheduled queue (Y4): edit, cancel and publish now.

use crate::domain::post::{PostStatus, Tint};
use crate::domain::PostId;
use crate::error::AppError;
use crate::state::store::{AppState, ScheduledEdit, ThreadAnchor, ThreadDraft};
use crate::storage::{Database, Vault};
use jiff::{SignedDuration, Timestamp};

const HOUR: SignedDuration = SignedDuration::from_hours(1);

fn now() -> Timestamp {
    crate::time::utc(2026, 11, 21, 8, 0, 0)
}

fn thread(state: &mut AppState, texts: &[&str]) -> Vec<PostId> {
    let drafts: Vec<ThreadDraft> = texts.iter().map(|text| ThreadDraft::text(*text)).collect();
    state.schedule_thread(&drafts, &ThreadAnchor::default(), now() + HOUR, now()).expect("scheduled")
}

fn waiting(state: &AppState) -> Vec<PostId> {
    state.scheduled_posts().iter().map(|p| p.id.clone()).collect()
}

#[test]
fn cancelling_deletes_the_post_and_the_replies_under_it() {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let ids = thread(&mut state, &["one", "two", "three"]);
    let other = thread(&mut state, &["unrelated"]);
    assert_eq!(state.cancel_scheduled(&ids[1]).expect("cancel"), vec![ids[1].clone(), ids[2].clone()]);
    assert_eq!(waiting(&state), vec![ids[0].clone(), other[0].clone()]);
    let loaded = AppState::load_from_db(&vault).expect("reload");
    assert_eq!(waiting(&loaded), waiting(&state), "cancelled posts are gone from the vault");
    assert!(state.cancel_scheduled(&ids[1]).is_err(), "only waiting posts can be cancelled");
}

#[test]
fn publishing_now_takes_the_whole_thread_out() {
    let mut state = AppState::new_in_memory();
    let ids = thread(&mut state, &["one", "two"]);
    let other = thread(&mut state, &["stays"]);
    let at = now() + SignedDuration::from_mins(5);
    assert_eq!(state.publish_scheduled_now(&ids[1], at).expect("publish"), ids);
    assert_eq!(waiting(&state), other);
    let first = state.get_post(&ids[0]).expect("on the timeline");
    assert_eq!((first.lifecycle.status, first.created_at, first.replies_count), (PostStatus::Published, at, 1));
}

#[test]
fn editing_rewrites_and_moves_the_thread_together() {
    let mut state = AppState::new_in_memory();
    let ids = thread(&mut state, &["one", "two"]);
    let later = now() + SignedDuration::from_hours(3);
    let edit = ScheduledEdit { content: "one, rewritten #idea".into(), tint: Some(Tint::Sky), publish_at: Some(later) };
    state.edit_scheduled(&ids[0], edit, now()).expect("edit");
    let first = state.scheduled_post(&ids[0]).expect("still waiting");
    assert_eq!((first.content.as_str(), first.extras.tint), ("one, rewritten #idea", Some(Tint::Sky)));
    assert!(state.scheduled_posts().iter().all(|p| p.lifecycle.publish_at == Some(later)));
    assert_eq!(waiting(&state), ids, "thread order is kept");
}

#[test]
fn an_edit_without_a_moment_publishes_and_bad_edits_change_nothing() {
    let mut state = AppState::new_in_memory();
    let ids = thread(&mut state, &["draft"]);
    let past = ScheduledEdit { content: "fine".into(), tint: None, publish_at: Some(now() - HOUR) };
    assert_eq!(state.edit_scheduled(&ids[0], past, now()), Err(AppError::ScheduleNotInFuture));
    let empty = ScheduledEdit { content: "  ".into(), tint: None, publish_at: Some(now() + HOUR) };
    assert!(state.edit_scheduled(&ids[0], empty, now()).is_err());
    assert_eq!(state.scheduled_post(&ids[0]).map(|p| p.content.as_str()), Some("draft"));
    let go = ScheduledEdit { content: "final words".into(), tint: None, publish_at: None };
    state.edit_scheduled(&ids[0], go, now()).expect("publish");
    assert!(state.scheduled_posts().is_empty());
    assert_eq!(state.get_post(&ids[0]).map(|p| p.content.as_str()), Some("final words"));
}
