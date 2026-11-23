//! N14: retracting and correcting the user's posts: a reason is needed, a locked post and
//! anyone else's are refused, and the mark can be taken back.

use super::locked::locked_post;
use crate::domain::post::correction::CorrectionKind;
use crate::state::store::AppState;
use jiff::Timestamp;

#[test]
fn a_post_is_marked_with_a_reason_and_the_mark_can_be_taken_back() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("Honey never spoils.").expect("posted");
    assert!(state.mark_correction(&id, CorrectionKind::Retracted, "   ", Timestamp::now()).is_err(), "a reason is needed");
    let mark = state.mark_correction(&id, CorrectionKind::Corrected, "It can ferment if wet.", Timestamp::now()).expect("marked");
    assert_eq!((mark.kind, mark.reason.as_str()), (CorrectionKind::Corrected, "It can ferment if wet."));
    assert_eq!(state.get_post(&id).and_then(|p| p.extras.correction.as_deref().map(|c| c.kind)), Some(CorrectionKind::Corrected));
    state.mark_correction(&id, CorrectionKind::Retracted, "The study was a hoax.", Timestamp::now()).expect("remarked");
    assert_eq!(state.get_post(&id).and_then(|p| p.extras.correction.as_deref().map(|c| c.kind)), Some(CorrectionKind::Retracted));
    state.withdraw_correction(&id).expect("withdrawn");
    assert!(state.get_post(&id).is_some_and(|p| p.extras.correction.is_none()));
}

#[test]
fn a_locked_post_and_an_agents_post_are_never_marked() {
    let mut state = AppState::new_in_memory();
    let locked = locked_post(&mut state);
    assert_eq!(state.mark_correction(&locked, CorrectionKind::Retracted, "Leaky reason.", Timestamp::now()).err(), Some(crate::error::AppError::PostLocked));
    assert!(state.get_post(&locked).is_some_and(|p| p.extras.correction.is_none()));
    let agent_post = state.posts.iter().find(|post| state.author_of(post).is_agent()).map(|post| post.id.clone());
    if let Some(agent_post) = agent_post {
        assert!(state.mark_correction(&agent_post, CorrectionKind::Retracted, "Not mine to retract.", Timestamp::now()).is_err());
    }
}
