//! Feeling posts (M1): stored with their emojis, intensity and note, and read back so.

use crate::domain::post::{Intensity, PostKind};
use crate::error::AppError;
use crate::state::store::{AppState, ThreadAnchor};
use crate::storage::{Database, Vault};

pub(super) fn tender() -> PostKind {
    PostKind::feeling(&["🥹".to_string(), "💛".to_string()], Intensity::new(4), "a letter from home").expect("valid")
}

#[test]
fn a_feeling_is_posted_with_its_text_and_kind() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_feeling(tender(), &ThreadAnchor::default()).expect("posted");
    let post = state.get_post(&id).expect("on the timeline");
    assert_eq!(post.kind, tender());
    assert_eq!(post.content, "🥹💛 a letter from home");
    assert_eq!(state.submit_feeling(PostKind::Question, &ThreadAnchor::default()), Err(AppError::EmptyContent), "only feelings");
}

#[test]
fn a_feeling_survives_a_restart() {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let id = state.submit_feeling(tender(), &ThreadAnchor::default()).expect("posted");
    let loaded = AppState::load_from_db(&vault).expect("reload");
    assert_eq!(loaded.get_post(&id).expect("stored").kind, tender());
}
