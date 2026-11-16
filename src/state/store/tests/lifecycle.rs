use crate::domain::{NetworkId, PostId};
use crate::state::store::AppState;
use crate::storage::Database;
use std::rc::Rc;

#[test]
fn test_store_initialization() {
    let state = AppState::new_in_memory();
    assert!(!state.networks.is_empty());
    assert_eq!(state.active_network_id.as_str(), "net_philosophy");
    assert!(state.current_network().is_some());
}

#[test]
fn test_store_submit_post() {
    let mut state = AppState::new_in_memory();
    let post_id = state
        .submit_post("New cognitive connection [[post_1]] #Ontology")
        .expect("Submission should succeed");

    let found = state.posts.iter().find(|p| p.id == post_id).expect("Post was inserted");
    assert_eq!(found.id, post_id);
    assert_eq!(found.linked_posts, vec![PostId::new("post_1")]);
}

#[test]
fn test_store_switch_network() {
    let mut state = AppState::new_in_memory();
    let target = NetworkId::new("net_systems");
    state.switch_network(&target).expect("Network exists");
    assert_eq!(state.active_network_id, target);
}

#[test]
fn test_store_persistence() {
    let db = Rc::new(Database::in_memory().expect("in-memory db"));
    let mut state = AppState::fallback_seed(Some(db.clone()));
    let post_id = state
        .submit_post("Persistent post test")
        .expect("Submission should succeed");

    let loaded = AppState::load_from_db(&db).expect("Load from db");
    assert!(loaded.posts.iter().any(|p| p.id == post_id));
}

#[test]
fn test_store_theme_tokens_persistence() {
    let db = Rc::new(Database::in_memory().expect("in-memory db"));
    let mut state = AppState::fallback_seed(Some(db.clone()));
    let mut custom_tokens = crate::theme::obsidian_zinc();
    custom_tokens.accent_brand = gpui_kit::rgb(0xff00aa);
    custom_tokens.titlebar_height = gpui_kit::px(40.0);

    state.update_theme_tokens(custom_tokens.clone());
    assert_eq!(state.theme_tokens.accent_brand, gpui_kit::rgb(0xff00aa));

    let loaded = AppState::load_from_db(&db).expect("Load from db");
    assert_eq!(loaded.theme_tokens.accent_brand, gpui_kit::rgb(0xff00aa));
    assert_eq!(loaded.theme_tokens.titlebar_height, gpui_kit::px(40.0));
}

#[test]
fn test_store_zero_copy_accessors() {
    let state = AppState::new_in_memory();
    let posts_slice = state.posts();
    let docs_slice = state.documents();

    assert_eq!(posts_slice.len(), state.posts.len());
    assert_eq!(docs_slice.len(), state.documents.len());
    assert!(std::ptr::eq(posts_slice.as_ptr(), state.posts.as_ptr()));
    assert!(std::ptr::eq(docs_slice.as_ptr(), state.documents.as_ptr()));
}

#[test]
fn scheduler_switch_defaults_off_and_persists() {
    let state = AppState::new_in_memory();
    assert!(!state.scheduler_enabled(), "never surprise-post on a fresh vault");
    state.set_scheduler_enabled(true);
    assert!(state.scheduler_enabled());
    state.set_scheduler_enabled(false);
    assert!(!state.scheduler_enabled());
}
