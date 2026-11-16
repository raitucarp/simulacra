//! The incremental post index must always equal a from-scratch rebuild.

use crate::domain::{Post, PostId};
use crate::state::store::{AppState, PostIndex};

/// Asserts every lookup of `state.index` matches a freshly rebuilt index.
fn assert_matches_rebuild(state: &AppState) {
    let mut fresh = PostIndex::default();
    fresh.rebuild(&state.posts);
    assert_eq!(state.index.len(), fresh.len());
    for (position, post) in state.posts.iter().enumerate() {
        assert_eq!(state.index.position(post.id.as_str()), Some(position), "position of {}", post.id);
        let replies: Vec<usize> = state.index.reply_positions(&post.id).collect();
        assert_eq!(replies, fresh.reply_positions(&post.id).collect::<Vec<_>>(), "replies of {}", post.id);
        assert_eq!(state.index.meta(post.id.as_str()), fresh.meta(post.id.as_str()));
        assert_eq!(state.index.network_count(&post.network_id), fresh.network_count(&post.network_id));
    }
}

fn reply_to(state: &AppState, parent: &PostId, id: &str) -> Post {
    let author = state.active_profile_id.clone();
    Post::create_reply(id, state.active_network_id.clone(), author, "agent reply #Tag", parent.clone(), None, None)
        .unwrap()
}

#[test]
fn head_middle_and_removal_updates_match_a_rebuild() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("root [[post_1]]").unwrap();
    for i in 0..3 {
        state.submit_post(&format!("filler {i}")).unwrap();
    }
    assert_matches_rebuild(&state);

    // Agent replies land right after their parent, deep inside the vector.
    state.insert_reply_after_parent(reply_to(&state, &root, "sim_1")).unwrap();
    state.insert_reply_after_parent(reply_to(&state, &root, "sim_2")).unwrap();
    assert_matches_rebuild(&state);
    assert_eq!(state.replies_for_post(&root).len(), 2);

    // Repost toggles insert at the head and remove from the head.
    state.repost(&root).unwrap();
    assert_matches_rebuild(&state);
    state.repost(&root).unwrap();
    assert_matches_rebuild(&state);

    // Removing from the middle shifts only newer entries.
    let middle = state.post_position(&root).unwrap();
    state.remove_post_at(middle);
    assert_matches_rebuild(&state);
    assert_eq!(state.index.full_rebuilds(), 1, "only the initial load rebuilds");
}

#[test]
fn single_mutations_bump_the_revision_without_rebuilding() {
    let mut state = AppState::new_in_memory();
    let (rebuilds, revision) = (state.index.full_rebuilds(), state.index.revision());
    let id = state.submit_post("fresh thought").unwrap();
    assert!(state.index.revision() > revision);
    assert_eq!(state.index.full_rebuilds(), rebuilds);
    assert_eq!(state.post_position(&id), Some(0));
    assert!(state.index.meta(id.as_str()).is_some_and(|m| m.search_text == "fresh thought"));
}

#[test]
fn search_uses_cached_text_and_author_names() {
    let mut state = AppState::new_in_memory();
    state.submit_post("Needle In The Haystack").unwrap();
    assert_eq!(state.filtered_posts("needle in").count(), 1);
    let author = state.current_profile().unwrap().name.to_uppercase();
    let by_author = state.filtered_posts(&author).count();
    assert!(by_author >= 1, "author name matches case-insensitively");
}
