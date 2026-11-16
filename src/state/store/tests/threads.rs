use crate::domain::{MAX_POST_CHARS, PostId};
use crate::error::AppError;
use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};

fn drafts(texts: &[&str]) -> Vec<ThreadDraft> {
    texts.iter().map(|t| ThreadDraft::text(*t)).collect()
}

fn post<'a>(state: &'a AppState, id: &PostId) -> &'a crate::domain::Post {
    state.get_post(id).expect("post exists")
}

#[test]
fn thread_chains_each_post_to_the_previous_one() {
    let mut state = AppState::new_in_memory();
    let ids = state
        .submit_thread(&drafts(&["first", "second", "third"]), &ThreadAnchor::default())
        .expect("thread publishes");
    assert_eq!(ids.len(), 3);
    assert_eq!(post(&state, &ids[0]).reply_to_id, None);
    assert_eq!(post(&state, &ids[1]).reply_to_id.as_ref(), Some(&ids[0]));
    assert_eq!(post(&state, &ids[2]).reply_to_id.as_ref(), Some(&ids[1]));
    let counts: Vec<usize> = ids.iter().map(|id| post(&state, id).replies_count).collect();
    assert_eq!(counts, vec![1, 1, 0]);
    assert_eq!(state.post_index_by_id.len(), state.posts.len());
}

#[test]
fn reply_thread_starts_under_the_reply_target() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("root").expect("root");
    let anchor = ThreadAnchor { reply_to: Some(root.clone()), quote_of: None };
    let ids = state.submit_thread(&drafts(&["a", "b"]), &anchor).expect("thread");
    assert_eq!(post(&state, &ids[0]).reply_to_id.as_ref(), Some(&root));
    assert_eq!(post(&state, &ids[1]).reply_to_id.as_ref(), Some(&ids[0]));
    assert_eq!(post(&state, &root).replies_count, 1);
}

#[test]
fn quote_and_attachments_apply_to_the_first_post_only() {
    let mut state = AppState::new_in_memory();
    let target = state.submit_post("target").expect("target");
    let mut thread = drafts(&["commentary", "follow-up"]);
    thread[0].link = Some("https://example.com".into());
    let anchor = ThreadAnchor { reply_to: None, quote_of: Some(target.clone()) };
    let ids = state.submit_thread(&thread, &anchor).expect("thread");
    assert_eq!(post(&state, &ids[0]).quote_id.as_ref(), Some(&target));
    assert_eq!(post(&state, &ids[0]).link_url.as_deref(), Some("https://example.com"));
    assert_eq!(post(&state, &ids[1]).quote_id, None);
}

#[test]
fn invalid_draft_stores_nothing() {
    let mut state = AppState::new_in_memory();
    let before = state.posts.len();
    let long = "x".repeat(MAX_POST_CHARS + 1);
    let err = state
        .submit_thread(&drafts(&["fine", &long, "also fine"]), &ThreadAnchor::default())
        .expect_err("too long");
    assert_eq!(err, AppError::ContentTooLong { max: MAX_POST_CHARS, actual: MAX_POST_CHARS + 1 });
    assert_eq!(state.posts.len(), before);

    let err = state.submit_thread(&drafts(&["ok", "  "]), &ThreadAnchor::default());
    assert_eq!(err, Err(AppError::EmptyContent));
    assert_eq!(state.submit_thread(&[], &ThreadAnchor::default()), Err(AppError::EmptyContent));
    assert_eq!(state.posts.len(), before);
}

#[test]
fn missing_anchor_targets_are_rejected() {
    let mut state = AppState::new_in_memory();
    let before = state.posts.len();
    let ghost = PostId::new("post_ghost");
    let reply = ThreadAnchor { reply_to: Some(ghost.clone()), quote_of: None };
    assert!(state.submit_thread(&drafts(&["a"]), &reply).is_err());
    let quote = ThreadAnchor { reply_to: None, quote_of: Some(ghost) };
    assert!(state.submit_thread(&drafts(&["a"]), &quote).is_err());
    assert_eq!(state.posts.len(), before);
}

#[test]
fn replies_index_counts_only_reply_to_relations() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("root").expect("root");
    let reply = state.submit_reply(&root, "a real reply", None, None).expect("reply");
    state.submit_quote(&root, "a quote is not a reply", None, None).expect("quote");
    state.submit_post(&format!("a wikilink [[{root}]] is not a reply")).expect("link");
    let replies: Vec<&PostId> = state.replies_for_post(&root).iter().map(|p| &p.id).collect();
    assert_eq!(replies, vec![&reply]);
    assert_eq!(state.replies_index.get(&root).map(Vec::len), Some(1));
}
