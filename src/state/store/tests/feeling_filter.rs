//! The feelings filter (M4): the feed's Feelings tab and an author's feelings.

use super::feelings::tender;
use crate::domain::ProfileId;
use crate::state::store::{AppState, Page, ThreadAnchor, ThreadDraft};
use crate::state::{feed_entries, FeedTab};

#[test]
fn the_feelings_tab_shows_only_conversations_with_a_feeling() {
    let mut state = AppState::new_in_memory();
    let thought = state.submit_thread(&[ThreadDraft::text("a plain thought")], &ThreadAnchor::default()).expect("posted")[0].clone();
    let feeling = state.submit_feeling(tender(), &ThreadAnchor::default()).expect("felt");
    let shown: Vec<_> = feed_entries(&state, FeedTab::Feelings).into_iter().map(|post| post.id.clone()).collect();
    assert!(shown.contains(&feeling));
    assert!(!shown.contains(&thought));
    assert!(shown.iter().all(|id| state.get_post(id).is_some_and(|post| post.kind.is_feeling() || post.reply_to_id.is_some())));
    assert!(FeedTab::ALL.contains(&FeedTab::Feelings));
}

#[test]
fn an_author_s_feelings_are_listed_newest_first() {
    let mut state = AppState::new_in_memory();
    let author = ProfileId::new(state.active_profile_id.as_str());
    state.submit_thread(&[ThreadDraft::text("a plain thought")], &ThreadAnchor::default()).expect("posted");
    let first = state.submit_feeling(tender(), &ThreadAnchor::default()).expect("felt");
    let mine: Vec<_> = state.feelings_by(&author, Page::first(10)).into_iter().map(|post| post.id.clone()).collect();
    assert_eq!(mine, vec![first]);
    assert!(state.feelings_by(&ProfileId::new("nobody"), Page::first(10)).is_empty());
}
