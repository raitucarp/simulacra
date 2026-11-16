//! Regression tests for the phase 6 data-layer fixes: dismissed seeds, network counts,
//! unreadable vaults, search after renames and handle normalisation.

use crate::domain::{NetworkId, Profile};
use crate::state::seed::seed_profiles;
use crate::state::store::AppState;
use crate::state::{FeedTab, StreamCache};
use crate::storage::{Database, Vault};
use std::path::PathBuf;

/// A seeded store backed by a fresh in-memory vault.
fn seeded_with_vault() -> (AppState, Vault) {
    let vault = Vault::spawn(Database::in_memory().expect("in-memory db")).expect("vault");
    (AppState::fallback_seed(Some(vault.clone())), vault)
}

/// A seeded autonomous agent with at least one post in the active network.
fn seed_agent_with_posts(state: &AppState) -> Profile {
    let authors: Vec<_> = state.current_network_posts().map(|p| p.author_id.clone()).collect();
    state
        .user_profiles
        .iter()
        .find(|p| p.is_agent() && authors.contains(&p.id))
        .cloned()
        .expect("a seeded agent with posts")
}

/// A unique scratch database path in the OS temp folder.
fn temp_db_path() -> PathBuf {
    std::env::temp_dir().join(format!("simulacra_p6_{}.db", uuid::Uuid::new_v4()))
}

#[test]
fn deleted_seed_agent_stays_deleted_after_reload() {
    let (mut state, vault) = seeded_with_vault();
    let agent = seed_agent_with_posts(&state);
    state.delete_profile(&agent.id).unwrap();

    let reloaded = AppState::load_from_db(&vault).expect("reload");
    assert!(!reloaded.user_profiles.iter().any(|p| p.id == agent.id), "deleted seed came back");
    assert!(reloaded.archived_authors.contains_key(&agent.id), "its posts keep an author");
}

#[test]
fn seeds_missing_from_an_older_vault_are_still_added() {
    let (state, vault) = seeded_with_vault();
    let shipped_later = seed_profiles().into_iter().find(|p| p.is_agent()).expect("seed agent");
    let id = shipped_later.id.clone();
    vault.write(move |db| db.delete_profile(&id)).unwrap();
    drop(state);

    let reloaded = AppState::load_from_db(&vault).expect("reload");
    assert!(reloaded.user_profiles.iter().any(|p| p.id == shipped_later.id), "new seed not added");
}

#[test]
fn network_counts_come_from_the_collections_and_survive_profile_saves() {
    let (mut state, vault) = seeded_with_vault();
    let counts = |s: &AppState| -> Vec<(NetworkId, usize, usize)> {
        s.networks.iter().map(|n| (n.id.clone(), n.doc_count, n.agent_count)).collect()
    };
    for net in &state.networks {
        let docs = state.documents.iter().filter(|d| d.network_id == net.id).count();
        assert_eq!(net.doc_count, docs, "{} shows a count its documents do not back", net.id);
        assert_eq!(net.agent_count, state.agents_for_network(&net.id).len());
    }
    let before = counts(&state);
    let agent = seed_agent_with_posts(&state);
    state.update_profile(agent).unwrap();
    assert_eq!(counts(&state), before, "saving a profile must not change the counts");
    assert_eq!(counts(&AppState::load_from_db(&vault).unwrap()), before);

    state.add_document("Notes", "Body").unwrap();
    let active = state.current_network().expect("active").doc_count;
    assert_eq!(active, state.current_documents().count(), "adding a document updates the count");
}

#[test]
fn unreadable_vault_file_is_reported_and_left_untouched() {
    let path = temp_db_path();
    let garbage = b"definitely not an sqlite database, just bytes".repeat(64);
    std::fs::write(&path, &garbage).unwrap();

    let state = AppState::with_db_path(&path);
    assert!(state.load_error.is_some(), "the failure must be surfaced");
    assert!(state.db.is_none(), "nothing may be written to a vault that failed to open");
    assert_eq!(std::fs::read(&path).unwrap(), garbage, "the file must not be overwritten");
    assert!(!state.networks.is_empty(), "the session still runs on demo data");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn vault_that_opens_but_fails_to_load_is_never_reseeded() {
    let path = temp_db_path();
    {
        let db = Database::open(&path).unwrap();
        db.save_network(&crate::domain::Network::new("net_mine", "Mine", "mine", "Kept")).unwrap();
    }
    {
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch("DROP TABLE trending; CREATE TABLE trending (tag TEXT PRIMARY KEY);").unwrap();
    }

    let state = AppState::with_db_path(&path);
    assert!(state.load_error.is_some(), "the failure must be surfaced");
    assert!(state.db.is_none(), "a vault that failed to load must not be written to");
    drop(state);
    let db = Database::open(&path).unwrap();
    assert!(db.load_profiles().unwrap().is_empty(), "seed profiles were written into the vault");
    assert_eq!(db.load_networks().unwrap().len(), 1, "the user's networks must be untouched");
    drop(db);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn fresh_install_seeds_the_new_vault_without_error() {
    let path = temp_db_path();
    let state = AppState::with_db_path(&path);
    assert!(state.load_error.is_none());
    assert!(state.db.is_some());
    let vault = state.db.clone().unwrap();
    assert!(!vault.load_profiles().unwrap().is_empty(), "first run seeds the vault");
    drop((state, vault));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn cached_search_follows_profile_renames() {
    let mut state = AppState::new_in_memory();
    let mut agent = seed_agent_with_posts(&state);
    let mut cache = StreamCache::default();
    let query = "zarathustra";
    let (before, _) = cache.positions(&state, FeedTab::ForYou, query);
    assert!(before.is_empty());

    agent.name = "Zarathustra".to_string();
    state.update_profile(agent.clone()).unwrap();
    let (after, rebuilt) = cache.positions(&state, FeedTab::ForYou, query);
    assert!(rebuilt, "a rename must invalidate cached search results");
    assert!(after.iter().all(|&i| state.posts[i].author_id == agent.id));
    assert!(!after.is_empty(), "the renamed author's posts must match");

    agent.handle = "@nietzsche_z".to_string();
    state.update_profile(agent).unwrap();
    let (by_handle, _) = cache.positions(&state, FeedTab::ForYou, "NIETZSCHE_Z");
    assert!(!by_handle.is_empty(), "handle renames are searchable too");
}

#[test]
fn saved_handles_collapse_repeated_sigils_and_match_case_insensitively() {
    let mut state = AppState::new_in_memory();
    let mut agent = seed_agent_with_posts(&state);
    agent.handle = "  @@Theo ".to_string();
    state.update_profile(agent.clone()).unwrap();
    let saved = state.user_profiles.iter().find(|p| p.id == agent.id).unwrap();
    assert_eq!(saved.handle, "@Theo");

    for lookup in ["@theo", "theo", "@@THEO", " Theo "] {
        let found = state.profile_by_handle(lookup).map(|p| p.id.clone());
        assert_eq!(found, Some(agent.id.clone()), "lookup `{lookup}`");
    }
    assert!(state.handle_taken("@THEO", None));
    assert!(!state.handle_taken("@theo", Some(&agent.id)), "own handle is not taken");
}
