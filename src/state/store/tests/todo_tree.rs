//! D7: a parent to-do closes only when every child is done or dropped, and reopening a child
//! reopens the parents it had let close.

use crate::domain::symbols::EntryState::{self, Done, Dropped, Open};
use crate::domain::PostId;
use crate::error::AppError;
use crate::state::store::AppState;

const MOVE: &str = "[ ] move house\n  [ ] pack the boxes\n    [ ] books\n  [~] sell the piano\n  [ ] book the van";

fn states(state: &AppState, id: &PostId) -> Vec<EntryState> {
    state.post_todos(id).iter().map(|entry| entry.state).collect()
}

#[test]
fn a_parent_waits_for_every_child() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post(MOVE).unwrap();
    assert_eq!(state.toggle_todo(&id, 0), Err(AppError::OpenSubtasks(2)), "pack and van are open; the piano is dropped");
    assert_eq!(state.toggle_todo(&id, 1), Err(AppError::OpenSubtasks(1)), "the books hold the boxes back");
    state.toggle_todo(&id, 2).unwrap();
    state.toggle_todo(&id, 1).unwrap();
    assert_eq!(state.toggle_todo(&id, 0), Err(AppError::OpenSubtasks(1)));
    state.toggle_todo(&id, 4).unwrap();
    assert_eq!(state.toggle_todo(&id, 0), Ok(Done));
    assert_eq!(states(&state, &id), vec![Done, Done, Done, Dropped, Done]);
}

#[test]
fn reopening_a_child_reopens_its_closed_parents() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("[x] trip\n  [x] tickets\n    [x] train\n  [~] hotel").unwrap();
    assert_eq!(state.toggle_todo(&id, 2), Ok(Open));
    assert_eq!(states(&state, &id), vec![Open, Open, Open, Dropped]);
    assert_eq!(state.set_todo_state(&id, 3, Open), Ok(()));
    assert_eq!(states(&state, &id)[3], Open, "reopening a dropped child is allowed");
}

#[test]
fn dropping_a_parent_does_not_wait() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post(MOVE).unwrap();
    assert_eq!(state.set_todo_state(&id, 0, Dropped), Ok(()));
    assert_eq!(states(&state, &id)[0], Dropped);
}
