//! Editing a locked post (X5): the new version is sealed again and every kept version keeps
//! its own seal, so no version's text is ever stored in the open.

use super::locked::{password, FAST, SECRET};
use crate::domain::data::history::PostRecord;
use crate::domain::post::lock::{open_text, seal_text, LockPassword, LOCKED_PLACEHOLDER};
use crate::error::AppError;
use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};
use crate::state::PostEdit;
use crate::storage::{Database, Vault};
use jiff::Timestamp;

const REWORDED: &str = "Quetzalcoatl orchard confession, reworded";

fn vault_state() -> (AppState, crate::domain::PostId) {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault));
    let id = super::locked::locked_post(&mut state);
    (state, id)
}

fn reword(state: &mut AppState, id: &crate::domain::PostId, text: &str, with: &LockPassword) {
    let sealed = seal_text(text, with, FAST).expect("sealed");
    state.edit_locked_post(id, &PostEdit::text(text), sealed, SECRET, Timestamp::now()).expect("edited");
}

#[test]
fn an_edit_seals_the_new_text_and_keeps_the_old_version_sealed() {
    let (mut state, id) = vault_state();
    reword(&mut state, &id, REWORDED, &password());
    let post = state.get_post(&id).expect("post");
    assert_eq!(post.content, LOCKED_PLACEHOLDER);
    assert_eq!(post.unlock(&password()).expect("opens").as_str(), REWORDED);
    let kept = state.post_revisions(&id);
    assert_eq!(kept.len(), 1);
    assert_eq!(kept[0].content, LOCKED_PLACEHOLDER, "the kept version holds no text");
    assert!(!kept[0].record_json.contains("Quetzalcoatl"));
    let record = PostRecord::parse(&kept[0].record_json).expect("record");
    assert_eq!(open_text(&record.lock.expect("its own seal"), &password()).expect("opens").as_str(), SECRET);
}

#[test]
fn switching_back_to_an_older_version_brings_its_seal_back() {
    let (mut state, id) = vault_state();
    reword(&mut state, &id, REWORDED, &password());
    let older = state.post_revisions(&id)[0].id.clone();
    state.switch_post_head(&id, &older, Timestamp::now()).expect("switched");
    let post = state.get_post(&id).expect("post");
    assert_eq!(post.content, LOCKED_PLACEHOLDER);
    assert_eq!(post.unlock(&password()).expect("opens").as_str(), SECRET);
    assert!(state.post_revisions(&id).iter().all(|r| r.content == LOCKED_PLACEHOLDER && !r.record_json.contains("reworded")));
}

#[test]
fn a_new_password_seals_only_the_new_version() {
    let (mut state, id) = vault_state();
    let other = LockPassword::new("another long password");
    reword(&mut state, &id, REWORDED, &other);
    assert_eq!(state.get_post(&id).expect("post").unlock(&other).expect("opens").as_str(), REWORDED);
    let record = PostRecord::parse(&state.post_revisions(&id)[0].record_json).expect("record");
    assert_eq!(open_text(&record.lock.expect("seal"), &password()).expect("the old version keeps its password").as_str(), SECRET);
}

#[test]
fn only_locked_posts_are_edited_this_way() {
    let mut state = AppState::new_in_memory();
    let plain = state.submit_thread(&[ThreadDraft::text("an open thought")], &ThreadAnchor::default()).expect("posted")[0].clone();
    let sealed = seal_text("x", &password(), FAST).expect("sealed");
    assert_eq!(state.edit_locked_post(&plain, &PostEdit::text("x"), sealed, "an open thought", Timestamp::now()).err(), Some(AppError::LockFailed));
}
