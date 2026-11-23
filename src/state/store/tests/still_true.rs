//! N13: the user's claims come back for a check; confirming pushes the next one out; other
//! people's posts and locked ones never come back.

use super::locked::locked_post;
use crate::domain::post::still_true::FIRST_WAIT_DAYS;
use crate::state::store::AppState;
use jiff::{SignedDuration, Timestamp};

fn days_later(days: i64) -> Timestamp {
    Timestamp::now().checked_add(SignedDuration::from_hours(24 * days)).expect("time")
}

#[test]
fn old_claims_are_due_and_a_confirmation_sends_them_away_for_longer() {
    let mut state = AppState::new_in_memory();
    let claim = state.submit_post("Cold brew keeps for two weeks.").expect("posted");
    let locked = locked_post(&mut state);
    assert!(state.due_for_check(days_later(FIRST_WAIT_DAYS - 1)).iter().all(|post| post.id != claim), "not yet");
    let due: Vec<_> = state.due_for_check(days_later(FIRST_WAIT_DAYS + 1)).into_iter().map(|post| post.id.clone()).collect();
    assert!(due.contains(&claim));
    assert!(!due.contains(&locked), "a locked post is never asked about");
    let check = state.confirm_still_true(&claim, days_later(FIRST_WAIT_DAYS + 1)).expect("confirmed");
    assert_eq!(check.streak, 1);
    assert!(state.due_for_check(days_later(FIRST_WAIT_DAYS * 2)).iter().all(|post| post.id != claim));
    assert!(state.due_for_check(days_later(FIRST_WAIT_DAYS * 4)).iter().any(|post| post.id == claim));
    assert!(state.confirm_still_true(&locked, Timestamp::now()).is_err());
}
