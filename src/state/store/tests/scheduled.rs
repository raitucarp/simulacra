//! Scheduled posts (Y1): hidden from the feed, its search and agents until they are due.

use crate::agent::tools::ToolContext;
use crate::domain::post::PostStatus;
use crate::domain::{PostId, ProfileId};
use crate::error::AppError;
use crate::state::store::{AppState, Page, ThreadAnchor, ThreadDraft};
use crate::storage::{Database, Vault};
use jiff::{SignedDuration, Timestamp};

const SECRET: &str = "Zanzibar lighthouse musings";
const HOUR: SignedDuration = SignedDuration::from_hours(1);

fn drafts(texts: &[&str]) -> Vec<ThreadDraft> {
    texts.iter().map(|text| ThreadDraft::text(*text)).collect()
}

/// A store with a two-post thread scheduled an hour from `now`.
fn scheduled_thread(state: &mut AppState, now: Timestamp) -> Vec<PostId> {
    let thread = drafts(&[SECRET, "and a second thought"]);
    state.schedule_thread(&thread, &ThreadAnchor::default(), now + HOUR, now).expect("scheduled")
}

#[test]
fn a_scheduled_thread_waits_outside_the_timeline() {
    let mut state = AppState::new_in_memory();
    let (now, before) = (Timestamp::now(), state.posts.len());
    let ids = scheduled_thread(&mut state, now);
    assert_eq!(state.posts.len(), before, "nothing reaches the timeline");
    assert_eq!(state.index.len(), before);
    assert!(ids.iter().all(|id| state.get_post(id).is_none()));
    let waiting: Vec<&PostId> = state.scheduled_posts().iter().map(|p| &p.id).collect();
    assert_eq!(waiting, ids.iter().collect::<Vec<_>>(), "thread order");
    assert!(state.scheduled_posts().iter().all(|p| p.lifecycle.status == PostStatus::Scheduled && p.lifecycle.publish_at == Some(now + HOUR)));
    assert_eq!(state.scheduled_posts()[1].reply_to_id.as_ref(), Some(&ids[0]));
}

#[test]
fn feed_search_and_agents_never_see_a_scheduled_post() {
    let mut state = AppState::new_in_memory();
    scheduled_thread(&mut state, Timestamp::now());
    assert_eq!(state.filtered_posts("zanzibar").count(), 0, "the feed search");
    assert!(state.filtered_posts("").all(|p| !p.lifecycle.is_scheduled()), "the feed");
    let network = state.active_network_id.clone();
    let context = ToolContext::capture(&network, &state.posts, &state.user_profiles, &state.documents);
    assert!(context.posts.iter().all(|p| !p.content.contains(SECRET)), "the agents' context");
}

#[test]
fn the_author_still_finds_them_among_their_posts() {
    let mut state = AppState::new_in_memory();
    let ids = scheduled_thread(&mut state, Timestamp::now());
    let author = ProfileId::new(state.active_profile_id.as_str());
    let mine: Vec<PostId> = state.posts_by(&author, Page::first(usize::MAX)).into_iter().map(|p| p.id.clone()).collect();
    assert!(ids.iter().all(|id| mine.contains(id)));
}

#[test]
fn scheduled_posts_stay_aside_after_a_restart() {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let ids = scheduled_thread(&mut state, Timestamp::now());
    let loaded = AppState::load_from_db(&vault).expect("reload");
    assert_eq!(loaded.posts.len(), state.posts.len());
    let waiting: Vec<PostId> = loaded.scheduled_posts().iter().map(|p| p.id.clone()).collect();
    assert_eq!(waiting, ids);
    assert!(vault.search_posts("zanzibar", 10).expect("search").is_empty(), "the full-text search");
}

#[test]
fn only_future_moments_and_personas_can_schedule() {
    let mut state = AppState::new_in_memory();
    let now = Timestamp::now();
    let past = state.schedule_thread(&drafts(&["late"]), &ThreadAnchor::default(), now - HOUR, now);
    assert_eq!(past, Err(AppError::ScheduleNotInFuture));
    let agent = state.user_profiles.iter().find(|p| p.is_agent()).map(|p| p.id.clone()).expect("a seeded agent");
    state.switch_profile(&agent).expect("switch");
    let by_agent = state.schedule_thread(&drafts(&["mine"]), &ThreadAnchor::default(), now + HOUR, now);
    assert!(matches!(by_agent, Err(AppError::NotAPersona(_))));
    assert!(state.scheduled_posts().is_empty());
}
