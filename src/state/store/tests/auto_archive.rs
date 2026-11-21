//! Auto-archive (Y6): per post and as a default, carried out by the heartbeat.

use crate::agent::schedule::FixedClock;
use crate::domain::preferences::ArchiveAfter;
use crate::domain::PostId;
use crate::state::store::AppState;
use crate::storage::{Database, Vault};
use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp};

fn created(state: &AppState, id: &PostId) -> Timestamp {
    state.get_post(id).expect("post").created_at
}

fn tick(state: &mut AppState, at: Timestamp) -> Vec<PostId> {
    state.lifecycle_heartbeat(&FixedClock(at.to_zoned_datetime(TimeZone::UTC))).expect("tick").archived
}

#[test]
fn the_default_is_stamped_on_the_users_new_posts() {
    let mut state = AppState::new_in_memory();
    let before = state.submit_post("written before the default").expect("post");
    let mut prefs = state.preferences().clone();
    prefs.writing.auto_archive = ArchiveAfter::Month;
    state.update_preferences(prefs);
    let after = state.submit_post("written after the default").expect("post");
    assert_eq!(state.get_post(&before).and_then(|p| p.lifecycle.archive_at), None, "older posts keep theirs");
    let expected = created(&state, &after) + crate::time::days(30);
    assert_eq!(state.get_post(&after).and_then(|p| p.lifecycle.archive_at), Some(expected));
}

#[test]
fn the_heartbeat_archives_what_is_due_and_it_survives_a_restart() {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let (soon, later) = (state.submit_post("goes soon").expect("post"), state.submit_post("goes later").expect("post"));
    state.set_auto_archive(&soon, ArchiveAfter::Week).expect("per post");
    state.set_auto_archive(&later, ArchiveAfter::Year).expect("per post");
    let week = created(&state, &soon) + crate::time::days(7);
    assert_eq!(ArchiveAfter::of(created(&state, &soon), Some(week)), Some(ArchiveAfter::Week), "the menu ticks the choice");
    assert!(tick(&mut state, week - SignedDuration::from_secs(1)).is_empty(), "not before its moment");
    assert_eq!(tick(&mut state, week), vec![soon.clone()]);
    assert!(state.filtered_posts("").all(|p| p.id != soon), "it left the feed");
    assert_eq!(state.filtered_posts("goes soon").count(), 1, "search still finds it");
    let loaded = AppState::load_from_db(&vault).expect("reload");
    assert!(loaded.get_post(&soon).is_some_and(|p| p.lifecycle.is_archived() && p.lifecycle.archive_at.is_none()));
    assert!(loaded.get_post(&later).is_some_and(|p| !p.lifecycle.is_archived()));
}

#[test]
fn never_stops_it_and_unarchiving_keeps_the_post_out_of_the_archive() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("stays").expect("post");
    let posted = created(&state, &id);
    state.set_auto_archive(&id, ArchiveAfter::Week).expect("set");
    state.set_auto_archive(&id, ArchiveAfter::Never).expect("clear");
    assert!(tick(&mut state, posted + crate::time::days(400)).is_empty());
    state.set_auto_archive(&id, ArchiveAfter::Week).expect("set again");
    assert_eq!(tick(&mut state, posted + crate::time::days(8)), vec![id.clone()]);
    state.set_archived(&id, false).expect("unarchive");
    assert!(tick(&mut state, posted + crate::time::days(9)).is_empty(), "not archived again");
}
