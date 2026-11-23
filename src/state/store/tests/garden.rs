//! N3: a garden note leaves the stream with the thread under it, matures as it is tended,
//! and goes back to the stream when returned.

use super::locked::{locked_post, SECRET};
use crate::domain::post::garden::Maturity;
use crate::state::store::AppState;
use crate::state::FeedTab;

fn at(day: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 11, day, 9, 0, 0)
}

fn in_stream(state: &AppState, id: &crate::domain::PostId) -> bool {
    crate::state::threads::stream_posts(state, FeedTab::ForYou, "").iter().any(|post| &post.id == id)
}

#[test]
fn a_planted_thread_leaves_the_stream_but_search_still_finds_it() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("Evergreen idea about attention.").expect("root");
    let reply = state.submit_post_with_attachments("And a follow-up.", None, None, Some(root.clone()), None).expect("reply");
    assert!(state.set_garden(&root, Some(Maturity::Seed), at(1)).expect("planted"));
    assert!(!in_stream(&state, &root) && !in_stream(&state, &reply), "the note and its thread live in the garden");
    assert!(state.filtered_posts("attention").any(|post| post.id == root), "search still finds it");
    assert!(state.in_garden(state.get_post(&reply).expect("reply")));
    assert!(state.set_garden(&root, None, at(2)).expect("returned"));
    assert!(in_stream(&state, &root) && in_stream(&state, &reply));
}

#[test]
fn tending_matures_a_note_and_records_when() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("A seed of a thought.").expect("posted");
    state.set_garden(&id, Some(Maturity::Seed), at(1)).expect("planted");
    assert!(!state.set_garden(&id, Some(Maturity::Seed), at(2)).expect("same"), "no change");
    state.set_garden(&id, Some(Maturity::Evergreen), at(3)).expect("grown");
    let note = state.get_post(&id).and_then(|post| post.extras.garden).expect("note");
    assert_eq!((note.maturity, note.planted_at, note.tended_at), (Maturity::Evergreen, at(1), Some(at(3))));
}

#[test]
fn only_the_user_s_own_posts_are_planted_and_a_locked_one_keeps_its_seal() {
    let mut state = AppState::new_in_memory();
    let agent_post = state.posts.iter().find(|post| !state.author_of(post).is_human_persona()).map(|post| post.id.clone());
    if let Some(id) = agent_post {
        assert!(state.set_garden(&id, Some(Maturity::Seed), at(1)).is_err(), "an agent's post is not the user's to plant");
    }
    let locked = locked_post(&mut state);
    state.set_garden(&locked, Some(Maturity::Budding), at(1)).expect("planted");
    let post = state.get_post(&locked).expect("post");
    assert!(post.is_locked() && !post.content.contains(SECRET));
}
