//! Card actions on an instant repost act on the original thought.

use crate::domain::PostId;
use crate::state::store::AppState;

/// A fresh store with one original thought and its instant repost.
fn store_with_repost() -> (AppState, PostId, PostId) {
    let mut state = AppState::new_in_memory();
    let original = state.submit_post("Original thought").expect("Submit original");
    let wrapper = state.repost(&original).expect("Repost");
    (state, original, wrapper)
}

#[test]
fn action_target_resolves_instant_reposts_only() {
    let (mut state, original, wrapper) = store_with_repost();
    assert_eq!(state.action_target(&wrapper), original);
    assert_eq!(state.action_target(&original), original);
    let quote = state.submit_quote(&original, "Commentary", None, None).expect("Quote");
    assert_eq!(state.action_target(&quote), quote);
    let missing = PostId::new("post_missing");
    assert_eq!(state.action_target(&missing), missing);
}

#[test]
fn liking_an_instant_repost_likes_the_original() {
    let (mut state, original, wrapper) = store_with_repost();
    state.toggle_like(&wrapper);
    let target = state.get_post(&original).expect("Original");
    assert_eq!((target.likes, target.is_liked), (1, true));
    let shell = state.get_post(&wrapper).expect("Wrapper");
    assert_eq!((shell.likes, shell.is_liked), (0, false));
}

#[test]
fn replying_to_an_instant_repost_replies_to_the_original() {
    let (mut state, original, wrapper) = store_with_repost();
    let reply = state.submit_reply(&wrapper, "A reply", None, None).expect("Reply");
    assert_eq!(state.get_post(&reply).and_then(|p| p.reply_to_id.clone()), Some(original.clone()));
    assert_eq!(state.get_post(&original).map(|p| p.replies_count), Some(1));
    assert_eq!(state.get_post(&wrapper).map(|p| p.replies_count), Some(0));
}

#[test]
fn quoting_an_instant_repost_quotes_the_original() {
    let (mut state, original, wrapper) = store_with_repost();
    let quote = state.submit_quote(&wrapper, "Commentary", None, None).expect("Quote");
    assert_eq!(state.get_post(&quote).and_then(|p| p.quote_id.clone()), Some(original));
}

#[test]
fn reposting_an_instant_repost_toggles_the_original() {
    let (mut state, original, wrapper) = store_with_repost();
    state.repost(&wrapper).expect("Un-repost through the wrapper");
    let target = state.get_post(&original).expect("Original");
    assert_eq!((target.reposts_count, target.is_reposted), (0, false));
    assert!(state.get_post(&wrapper).is_none());
}
