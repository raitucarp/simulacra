//! N18: the user's questions stay open until marked answered, can be reopened, and are listed
//! oldest first; a locked post is never made a question, nor a feeling.

use super::locked::locked_post;
use crate::domain::post::open_question::{question_state, QuestionState};
use crate::domain::post::PostKind;
use crate::error::AppError;
use crate::state::store::AppState;
use jiff::Timestamp;

#[test]
fn a_question_stays_open_until_answered_and_can_be_reopened() {
    let mut state = AppState::new_in_memory();
    let older = state.submit_post("Why do cats purr?").expect("posted");
    let newer = state.submit_post("What makes bread rise?").expect("posted");
    assert!(state.resolve_question(&older, None, Timestamp::now()).is_err(), "not a question yet");
    state.set_question(&newer, true).expect("asked");
    state.set_question(&older, true).expect("asked");
    assert_eq!(state.open_questions().iter().map(|post| post.id.clone()).collect::<Vec<_>>(), vec![older.clone(), newer.clone()], "oldest first");
    let answer = state.submit_post("Yeast breathes out gas.").expect("posted");
    state.resolve_question(&newer, Some(&answer), Timestamp::now()).expect("answered");
    let answered = state.get_post(&newer).expect("post");
    assert_eq!((question_state(answered), answered.extras.answered.as_ref().and_then(|a| a.answer.clone())), (Some(QuestionState::Answered), Some(answer)));
    assert_eq!(state.open_questions().len(), 1);
    state.reopen_question(&newer).expect("reopened");
    assert_eq!(state.open_questions().len(), 2);
    state.set_question(&older, false).expect("a thought again");
    assert_eq!(state.get_post(&older).map(|post| post.kind.clone()), Some(PostKind::Standard));
}

#[test]
fn a_locked_post_is_never_a_question_and_never_listed_open() {
    let mut state = AppState::new_in_memory();
    let locked = locked_post(&mut state);
    assert_eq!(state.set_question(&locked, true).err(), Some(AppError::PostLocked));
    assert_eq!(state.resolve_question(&locked, None, Timestamp::now()).err(), Some(AppError::PostLocked));
    assert!(state.open_questions().iter().all(|post| post.id != locked));
}
