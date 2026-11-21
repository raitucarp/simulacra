//! Warning before auto-deleting a post others answer (Y8).

use crate::domain::{Post, PostId};
use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};

/// A post by `author` answering `target` (a reply, or a quote when `quote`).
fn answer(state: &AppState, author: &crate::domain::ProfileId, target: &PostId, quote: bool) -> Post {
    let mut post = state.posts[0].clone();
    post.id = PostId::new(format!("answer_{target}_{quote}"));
    post.author_id = author.clone();
    post.reply_to_id = (!quote).then(|| target.clone());
    post.quote_id = quote.then(|| target.clone());
    post
}

#[test]
fn replies_and_quotes_by_others_count_and_the_users_own_do_not() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("worth answering").expect("post");
    let own = state.submit_thread(&[ThreadDraft::text("my own follow-up")], &ThreadAnchor { reply_to: Some(id.clone()), quote_of: None }).expect("reply");
    assert_eq!(own.len(), 1);
    assert!(!state.answered_by_others(&id), "the author's own reply does not count");
    let agent = state.posts.iter().map(|p| p.author_id.clone()).find(|author| !state.user_profiles.iter().any(|p| &p.id == author && p.is_human_persona())).expect("seeded agent");
    let quote = answer(&state, &agent, &id, true);
    state.insert_reply_after_parent(quote).expect("quote");
    assert!(state.answered_by_others(&id), "a quote by an agent counts");
    let other = state.submit_post("answered by a reply").expect("post");
    let reply = answer(&state, &agent, &other, false);
    state.insert_reply_after_parent(reply).expect("reply");
    assert!(state.answered_by_others(&other), "a reply by an agent counts");
    assert!(!state.answered_by_others(&PostId::new("missing")));
}
