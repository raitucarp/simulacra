//! `submit_thread` accepts a tree, not only a chain (T4).

use crate::domain::compose::compose_lines;
use crate::domain::PostId;
use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};

fn tree(texts: &[(&str, Option<usize>)]) -> Vec<ThreadDraft> {
    texts.iter().map(|(text, reply_to)| ThreadDraft { reply_to: *reply_to, ..ThreadDraft::text(*text) }).collect()
}

fn parent_of(state: &AppState, id: &PostId) -> Option<PostId> {
    state.get_post(id).unwrap().reply_to_id.clone()
}

#[test]
fn drafts_reply_to_the_draft_they_name_and_chain_otherwise() {
    let mut state = AppState::new_in_memory();
    let drafts = tree(&[("root", None), ("first reply", Some(0)), ("its reply", Some(1)), ("second reply", Some(0)), ("next in the chain", None)]);
    let ids = state.submit_thread(&drafts, &ThreadAnchor::default()).expect("a tree");
    assert_eq!(parent_of(&state, &ids[0]), None);
    assert_eq!(parent_of(&state, &ids[1]).as_ref(), Some(&ids[0]));
    assert_eq!(parent_of(&state, &ids[2]).as_ref(), Some(&ids[1]));
    assert_eq!(parent_of(&state, &ids[3]).as_ref(), Some(&ids[0]), "a sibling of the first reply");
    assert_eq!(parent_of(&state, &ids[4]).as_ref(), Some(&ids[3]), "no target keeps the chain");
    assert_eq!(state.get_post(&ids[0]).unwrap().replies_count, 2);
}

#[test]
fn a_draft_naming_itself_or_a_later_one_stores_nothing() {
    let mut state = AppState::new_in_memory();
    let before = state.posts().len();
    for bad in [tree(&[("root", None), ("loop", Some(1))]), tree(&[("root", Some(3)), ("x", None)])] {
        assert!(state.submit_thread(&bad, &ThreadAnchor::default()).is_err());
    }
    assert_eq!(state.posts().len(), before);
}

#[test]
fn free_compose_text_publishes_as_the_tree_it_previews() {
    let mut state = AppState::new_in_memory();
    let anchor_post = state.submit_post("What do you read slowly?").unwrap();
    let lines = compose_lines("Poetry\n  - because rhythm\nLetters\n  - old ones");
    let drafts: Vec<ThreadDraft> = lines.iter().map(|line| ThreadDraft { reply_to: line.parent, ..ThreadDraft::text(line.text.clone()) }).collect();
    let ids = state.submit_thread(&drafts, &ThreadAnchor { reply_to: Some(anchor_post.clone()), quote_of: None }).unwrap();
    let parents: Vec<Option<PostId>> = ids.iter().map(|id| parent_of(&state, id)).collect();
    assert_eq!(parents, vec![Some(anchor_post), Some(ids[0].clone()), Some(ids[0].clone()), Some(ids[2].clone())]);
}
