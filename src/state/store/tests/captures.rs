use crate::state::store::AppState;
use crate::state::{feed_entries, FeedTab};

#[test]
fn captures_land_in_the_inbox_and_nowhere_else_is_filtered() {
    let mut state = AppState::new_in_memory();
    let before = state.posts.len();
    let for_you_before = feed_entries(&state, FeedTab::ForYou).len();
    assert_eq!(state.inbox_count(), 0);
    assert_eq!(state.capture("A thought worth keeping", None).expect("note"), 1);
    assert_eq!(state.capture("https://example.org/essay", None).expect("link"), 1);
    assert!(state.capture("   ", None).is_err(), "blank captures are refused");
    assert_eq!(state.posts.len(), before + 2);
    assert_eq!(state.inbox_count(), 2);
    let inbox = feed_entries(&state, FeedTab::Inbox);
    assert_eq!(inbox.len(), 2);
    assert!(inbox.iter().any(|post| post.link_url.as_deref() == Some("https://example.org/essay")));
    assert_eq!(feed_entries(&state, FeedTab::ForYou).len(), for_you_before + 2);
}

#[test]
fn long_captures_become_a_thread_at_the_head_of_the_timeline() {
    let mut state = AppState::new_in_memory();
    let posts = state.capture(&"Keep the friction low. ".repeat(40), None).expect("thread");
    assert!(posts > 1);
    let head = &state.posts[0];
    assert!(head.reply_to_id.is_some(), "the newest post is the end of the thread");
    assert_eq!(state.post_position(&head.id), Some(0), "the index is rebuilt");
    let root = state.posts[posts - 1].clone();
    assert_eq!(root.replies_count, 1);
    assert_eq!(state.replies_for_post(&root.id).len(), 1);
}
