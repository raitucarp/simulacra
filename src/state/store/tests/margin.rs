//! N2: margin replies keep the span they answer, follow it through edits and never quote a
//! locked post.

use super::locked::{locked_post, SECRET};
use crate::domain::post::anchor::AnchorPlace;
use crate::error::AppError;
use crate::state::store::AppState;

const TEXT: &str = "Habits compound. Small steps add up over years.";

#[test]
fn a_margin_reply_answers_one_span_of_the_version_it_quoted() {
    let mut state = AppState::new_in_memory();
    let parent = state.submit_post(TEXT).expect("parent");
    let reply = state.submit_margin_reply(&parent, 17..47, "Only if you keep them.").expect("reply");
    let anchor = state.get_post(&reply).and_then(|post| post.extras.anchor.clone()).expect("anchor");
    assert_eq!((anchor.quote.as_str(), anchor.start, anchor.version), ("Small steps add up over years.", 17, Some(Default::default())));
    assert_eq!(state.get_post(&reply).expect("reply").reply_to_id.as_ref(), Some(&parent));
    let notes = state.margin_notes(&parent);
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].place, AnchorPlace::Exact(17..47));
}

#[test]
fn notes_follow_their_span_through_edits_and_sort_by_place() {
    let mut state = AppState::new_in_memory();
    let parent = state.submit_post(TEXT).expect("parent");
    let late = state.submit_margin_reply(&parent, 17..47, "Second span.").expect("late");
    let early = state.submit_margin_reply(&parent, 0..16, "First span.").expect("early");
    let plain = state.submit_post_with_attachments("A plain reply.", None, None, Some(parent.clone()), None).expect("plain");
    let order: Vec<_> = state.margin_notes(&parent).into_iter().map(|note| note.reply).collect();
    assert_eq!(order, vec![early.clone(), late.clone()], "plain replies are no margin notes");
    assert!(!order.contains(&plain));
    state.edit_post(&parent, "Truly said: Habits compound. Small steps add up over years.").expect("edited");
    let notes = state.margin_notes(&parent);
    assert!(notes.iter().all(|note| matches!(note.place, AnchorPlace::Moved(_) | AnchorPlace::Exact(_))), "{notes:?}");
    state.edit_post(&parent, "Rewritten from scratch into a different claim.").expect("edited");
    assert!(state.margin_notes(&parent).iter().all(|note| note.place == AnchorPlace::Gone));
}

#[test]
fn a_locked_post_is_never_quoted_in_the_margin() {
    let mut state = AppState::new_in_memory();
    let locked = locked_post(&mut state);
    let before = state.posts.len();
    assert_eq!(state.submit_margin_reply(&locked, 0..1, "Quoting a secret."), Err(AppError::PostLocked));
    assert_eq!(state.posts.len(), before, "nothing was published");
    let mut imported = crate::domain::Post::create_reply("post_imported", state.active_network_id.clone(), state.active_profile_id.clone(), "An old note.", locked.clone(), None, None).expect("reply");
    imported.extras.anchor = Some(crate::domain::post::anchor::SpanAnchor { quote: SECRET.to_string(), start: 0, version: None });
    state.insert_reply_after_parent(imported).expect("stored");
    let notes = state.margin_notes(&locked);
    assert_eq!(notes.len(), 1);
    assert!(notes.iter().all(|note| !note.quote.contains(SECRET) && note.place == AnchorPlace::Gone), "only the placeholder is shown");
}

#[test]
fn a_span_outside_the_text_is_refused() {
    let mut state = AppState::new_in_memory();
    let parent = state.submit_post(TEXT).expect("parent");
    assert!(state.submit_margin_reply(&parent, 40..400, "Too far.").is_err());
    assert!(state.margin_notes(&parent).is_empty());
}
