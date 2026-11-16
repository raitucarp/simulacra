//! Store operations behind the post overflow menu and the "create network" dialog.

use crate::domain::Network;
use crate::state::store::{network_slug, validate_network_name, AppState, NetworkNameError};
use crate::state::store::MAX_NETWORK_NAME_CHARS;

#[test]
fn delete_post_removes_it_and_releases_the_parent_reply_counter() {
    let mut state = AppState::new_in_memory();
    let root_id = state.submit_post("Root thought").expect("root");
    let reply_id = state.submit_reply(&root_id, "A reply", None, None).expect("reply");
    let before = state.get_post(&root_id).expect("root").replies_count;

    state.delete_post(&reply_id).expect("delete reply");

    assert!(state.get_post(&reply_id).is_none());
    assert_eq!(state.get_post(&root_id).expect("root").replies_count, before - 1);
    let stored = state.db.as_ref().expect("db").load_posts().expect("load");
    assert!(stored.iter().all(|p| p.id != reply_id), "deleted from SQLite too");
}

#[test]
fn delete_post_takes_its_instant_reposts_but_keeps_replies() {
    let mut state = AppState::new_in_memory();
    let root_id = state.submit_post("Original").expect("root");
    let repost_id = state.repost(&root_id).expect("repost");
    let reply_id = state.submit_reply(&root_id, "Still here", None, None).expect("reply");

    state.delete_post(&root_id).expect("delete root");

    assert!(state.get_post(&root_id).is_none());
    assert!(state.get_post(&repost_id).is_none(), "empty repost goes with its target");
    assert!(state.get_post(&reply_id).is_some(), "replies are kept");
    assert_eq!(state.post_index_by_id.len(), state.posts.len(), "indices resynced");
}

#[test]
fn deleting_an_instant_repost_clears_the_target_flag() {
    let mut state = AppState::new_in_memory();
    let root_id = state.submit_post("Worth sharing").expect("root");
    let repost_id = state.repost(&root_id).expect("repost");

    state.delete_post(&repost_id).expect("delete repost");

    let target = state.get_post(&root_id).expect("target");
    assert_eq!(target.reposts_count, 0);
    assert!(!target.is_reposted);
}

#[test]
fn delete_post_reports_unknown_ids() {
    let mut state = AppState::new_in_memory();
    assert!(state.delete_post(&"post_missing".into()).is_err());
}

#[test]
fn network_names_are_trimmed_and_validated() {
    let existing = vec![Network::new("net_a", "Agora", "agora", "")];
    assert_eq!(validate_network_name("  Stoa  ", &existing), Ok("Stoa".to_string()));
    assert_eq!(validate_network_name("   ", &existing), Err(NetworkNameError::Empty));
    assert_eq!(
        validate_network_name(" agora ", &existing),
        Err(NetworkNameError::Duplicate("agora".to_string()))
    );
    let too_long = "x".repeat(MAX_NETWORK_NAME_CHARS + 1);
    assert_eq!(validate_network_name(&too_long, &existing), Err(NetworkNameError::TooLong));
    let at_limit = "y".repeat(MAX_NETWORK_NAME_CHARS);
    assert!(validate_network_name(&at_limit, &existing).is_ok());
}

#[test]
fn network_slugs_collapse_punctuation_and_case() {
    assert_eq!(network_slug("Stoa & Garden"), "stoa-garden");
    assert_eq!(network_slug("  Vienna   Circle! "), "vienna-circle");
    assert_eq!(network_slug("Ética"), "ética");
}

#[test]
fn create_network_persists_and_activates_it() {
    let mut state = AppState::new_in_memory();
    let count = state.networks.len();

    let id = state.create_network("  Lyceum ").expect("create");

    assert_eq!(state.networks.len(), count + 1);
    assert_eq!(state.active_network_id, id);
    assert_eq!(state.current_network().map(|n| n.name.as_str()), Some("Lyceum"));
    let stored = state.db.as_ref().expect("db").load_networks().expect("load");
    assert!(stored.iter().any(|n| n.id == id && n.slug == "lyceum"));
    assert!(matches!(state.create_network("lyceum"), Err(NetworkNameError::Duplicate(_))));
}
