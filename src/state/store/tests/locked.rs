//! Locked posts (X2): the sealed text lives in `extras.lock`; the full-text index, the
//! meaning index, the agents' context and their prompts only ever see the placeholder.

use crate::agent::tools::ToolContext;
use crate::agent::DialecticEngine;
use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::{LockPassword, LOCKED_PLACEHOLDER};
use crate::domain::PostId;
use crate::error::AppError;
use crate::state::semantic::index_sources;
use crate::state::store::{seal_thread, AppState, ThreadAnchor, ThreadDraft};
use crate::storage::{Database, Vault};
use jiff::{SignedDuration, Timestamp};

pub(super) const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };
pub(super) const SECRET: &str = "Quetzalcoatl orchard confession";

pub(super) fn password() -> LockPassword {
    LockPassword::new("seven tidy lanterns")
}

/// A store with one locked post holding [`SECRET`].
pub(super) fn locked_post(state: &mut AppState) -> PostId {
    let ids = state.submit_locked_thread(&[ThreadDraft::text(SECRET)], &ThreadAnchor::default(), &password(), FAST).expect("locked");
    ids[0].clone()
}

fn leaks(text: &str) -> bool {
    text.contains("Quetzalcoatl") || text.contains("confession")
}

#[test]
fn a_locked_post_holds_the_placeholder_and_opens_with_its_password() {
    let mut state = AppState::new_in_memory();
    let id = locked_post(&mut state);
    let post = state.get_post(&id).expect("on the timeline");
    assert_eq!(post.content, LOCKED_PLACEHOLDER);
    assert_eq!(post.unlock(&password()).expect("opens").as_str(), SECRET);
    assert!(state.index.meta(id.as_str()).is_some_and(|meta| meta.tokens.iter().all(|token| !leaks(&format!("{token:?}")))));
}

#[test]
fn the_vault_and_its_full_text_search_never_hold_the_plaintext() {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let id = locked_post(&mut state);
    for term in ["quetzalcoatl", "orchard", "confession"] {
        assert!(vault.search_posts(term, 10).expect("search").is_empty(), "{term}");
    }
    let loaded = AppState::load_from_db(&vault).expect("reload");
    let stored = loaded.get_post(&id).expect("stored");
    assert!(!leaks(&serde_json::to_string(stored).expect("json")));
    assert_eq!(stored.unlock(&password()).expect("still opens").as_str(), SECRET);
}

#[test]
fn embeddings_agents_and_prompts_never_see_the_plaintext() {
    let mut state = AppState::new_in_memory();
    let id = locked_post(&mut state);
    let sources = index_sources(&state);
    assert!(sources.iter().all(|source| !leaks(&source.text) && !source.owner.id.contains(id.as_str())), "the meaning index");
    assert!(state.filtered_posts("orchard").next().is_none(), "the feed search");
    let network = state.active_network_id.clone();
    let context = ToolContext::capture(&network, &state.posts, &state.user_profiles, &state.documents);
    assert!(context.posts.iter().all(|post| post.id != id.as_str() && !leaks(&post.content)), "the agents' context");
    let agent = state.user_profiles.iter().find(|p| p.is_agent()).cloned().expect("a seeded agent");
    let prompt = DialecticEngine::persona_user_prompt(&state.authored(state.get_post(&id).expect("post")), &agent);
    assert!(!leaks(&prompt), "a reply prompt about the post");
}

#[test]
fn only_sealed_text_only_persona_threads_are_stored() {
    let mut state = AppState::new_in_memory();
    let built = state.build_locked_thread(&[ThreadDraft::text(SECRET)], &ThreadAnchor::default()).expect("built");
    assert_eq!(state.publish_locked(built.clone(), None, Timestamp::now()), Err(AppError::LockFailed), "an unsealed post is refused");
    let with_link = ThreadDraft { link: Some("https://example.org".into()), ..ThreadDraft::text(SECRET) };
    assert_eq!(state.build_locked_thread(&[with_link], &ThreadAnchor::default()).err(), Some(AppError::LockedWithAttachments));
    let sealed = seal_thread(built, &password(), FAST).expect("sealed");
    let now = Timestamp::now();
    assert_eq!(state.publish_locked(sealed.clone(), Some(now - SignedDuration::from_hours(1)), now), Err(AppError::ScheduleNotInFuture));
    let ids = state.publish_locked(sealed, Some(now + SignedDuration::from_hours(1)), now).expect("scheduled");
    assert!(state.get_post(&ids[0]).is_none() && state.scheduled_posts()[0].is_locked());
    let agent = state.user_profiles.iter().find(|p| p.is_agent()).map(|p| p.id.clone()).expect("agent");
    state.switch_profile(&agent).expect("switch");
    assert!(matches!(state.build_locked_thread(&[ThreadDraft::text("x")], &ThreadAnchor::default()), Err(AppError::NotAPersona(_))));
}

#[test]
fn a_locked_post_is_never_edited_as_plain_text() {
    let mut state = AppState::new_in_memory();
    let id = locked_post(&mut state);
    assert_eq!(state.edit_post(&id, "now in the open"), Err(AppError::PostLocked));
    assert_eq!(state.get_post(&id).expect("post").content, LOCKED_PLACEHOLDER);
}
