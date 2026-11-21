//! Archiving (Y5): archived posts leave the feed and stay searchable.

use crate::domain::post::PostStatus;
use crate::domain::PostId;
use crate::state::store::AppState;
use crate::state::threads::{stream_posts, FeedTab};
use crate::storage::{Database, Vault};

fn feed(state: &AppState) -> Vec<PostId> {
    stream_posts(state, FeedTab::default(), "").iter().map(|p| p.id.clone()).collect()
}

#[test]
fn archived_posts_leave_the_feed_and_stay_searchable() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("a lighthouse keeper's notebook").expect("post");
    assert!(state.set_archived(&id, true).expect("archive"));
    assert!(!feed(&state).contains(&id), "out of the feed");
    assert!(state.filtered_posts("").all(|p| p.id != id), "out of the unfiltered timeline");
    let found: Vec<&PostId> = stream_posts(&state, FeedTab::default(), "lighthouse").iter().map(|p| &p.id).collect();
    assert_eq!(found, vec![&id], "the feed's search still finds it");
    assert!(state.get_post(&id).is_some_and(|p| p.lifecycle.status == PostStatus::Archived));
    assert!(!state.set_archived(&id, true).expect("again"), "archiving twice changes nothing");
}

#[test]
fn unarchiving_brings_the_post_back_and_both_survive_a_restart() {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let (kept, back) = (state.submit_post("kept away").expect("post"), state.submit_post("brought back").expect("post"));
    state.set_archived(&kept, true).expect("archive");
    state.set_archived(&back, true).expect("archive");
    assert!(state.set_archived(&back, false).expect("unarchive"));
    assert!(feed(&state).contains(&back) && !feed(&state).contains(&kept));
    let loaded = AppState::load_from_db(&vault).expect("reload");
    assert!(feed(&loaded).contains(&back) && !feed(&loaded).contains(&kept));
    assert_eq!(vault.search_posts("kept away", 5).expect("search"), vec![kept], "the full-text search keeps it");
}

#[test]
fn agents_posts_are_never_archived() {
    let mut state = AppState::new_in_memory();
    let agent_post = state.posts.iter().find(|p| !state.author_of(p).is_human_persona()).map(|p| p.id.clone()).expect("seeded agent post");
    assert!(state.set_archived(&agent_post, true).is_err());
}
