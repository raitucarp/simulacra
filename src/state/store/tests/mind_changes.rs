//! N19: marking a post as one that changed the user's mind: linked to the post it revises
//! and/or with the old view; never a locked post on either side, nor anyone else's; the
//! revised post knows, and an agent reads the old view with the post.

use super::locked::locked_post;
use crate::error::AppError;
use crate::state::store::AppState;
use jiff::Timestamp;

#[test]
fn a_post_revises_an_earlier_one_and_the_mark_can_be_taken_back() {
    let mut state = AppState::new_in_memory();
    let before = state.submit_post("More roads mean less traffic.").expect("posted");
    let after = state.submit_post("Induced demand: more roads, more cars.").expect("posted");
    assert_eq!(state.mark_changed_mind(&after, None, "  ", Timestamp::now()).err(), Some(AppError::EmptyContent), "a link or the old view is needed");
    assert!(state.mark_changed_mind(&after, Some(&after), "", Timestamp::now()).is_err(), "never itself");
    let change = state.mark_changed_mind(&after, Some(&before), "Roads ease traffic.", Timestamp::now()).expect("marked");
    assert_eq!((change.revises.as_ref(), change.was.as_str()), (Some(&before), "Roads ease traffic."));
    assert_eq!(state.revised_by(&before).iter().map(|post| post.id.clone()).collect::<Vec<_>>(), vec![after.clone()]);
    assert_eq!(state.mind_changes().len(), 1);
    assert!(state.get_post(&after).expect("post").agent_text().contains("before, they thought: Roads ease traffic."));
    state.unmark_changed_mind(&after).expect("unmarked");
    assert!(state.revised_by(&before).is_empty() && state.mind_changes().is_empty());
}

#[test]
fn a_locked_post_is_never_marked_nor_linked() {
    let mut state = AppState::new_in_memory();
    let locked = locked_post(&mut state);
    let open = state.submit_post("I was wrong about this.").expect("posted");
    assert_eq!(state.mark_changed_mind(&open, Some(&locked), "", Timestamp::now()).err(), Some(AppError::PostLocked));
    assert_eq!(state.mark_changed_mind(&locked, Some(&open), "Leaky old view.", Timestamp::now()).err(), Some(AppError::PostLocked));
    let at = state.post_position(&locked).expect("locked");
    state.posts[at].extras.mind_change = Some(Box::new(crate::domain::post::mind::MindChange { revises: Some(open.clone()), was: "Leaky old view.".into(), at: Timestamp::now() }));
    assert!(state.revised_by(&open).is_empty() && state.mind_changes().is_empty(), "a forged mark on a locked post is never listed");
    assert!(!state.get_post(&locked).expect("post").agent_text().contains("Leaky"), "an agent reads the placeholder alone");
}
