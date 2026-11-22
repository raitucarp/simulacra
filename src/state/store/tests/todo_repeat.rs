//! D8: completing a repeating to-do brings its next occurrence round instead of closing it.

use crate::domain::symbols::EntryState::{Done, Open};
use crate::domain::PostId;
use crate::state::store::AppState;
use jiff::civil::{date, Date};

/// A post of the user's written at noon UTC on Sunday 22 November 2026.
fn written(state: &mut AppState, text: &str) -> PostId {
    let id = state.submit_post(text).unwrap();
    let post = state.posts.iter_mut().find(|post| post.id == id).unwrap();
    post.created_at = crate::time::utc(2026, 11, 22, 12, 0, 0);
    id
}

fn due(state: &AppState, id: &PostId, index: usize) -> Option<Date> {
    state.post_todos(id)[index].due(date(2026, 11, 22))
}

#[test]
fn completing_a_repeating_item_moves_it_to_its_next_day() {
    let mut state = AppState::new_in_memory();
    let id = written(&mut state, "[ ] water the plants ^mon ^^mon,thu\n[ ] once ^mon");
    assert_eq!(due(&state, &id, 0), Some(date(2026, 11, 23)));
    assert_eq!(state.toggle_todo_on(&id, 0, date(2026, 11, 23)), Ok(Open), "it comes round again");
    assert_eq!(due(&state, &id, 0), Some(date(2026, 11, 26)));
    assert_eq!(state.post_todos(&id)[0].completions, 1);
    assert_eq!(state.toggle_todo_on(&id, 1, date(2026, 11, 23)), Ok(Done), "a plain item just closes");
    assert!(state.get_post(&id).unwrap().extras.head.is_none(), "still no version");
}

#[test]
fn a_late_completion_skips_the_days_already_gone() {
    let mut state = AppState::new_in_memory();
    let id = written(&mut state, "[ ] stretch ^^daily");
    assert_eq!(due(&state, &id, 0), None, "no due day until it is first done");
    state.toggle_todo_on(&id, 0, date(2026, 11, 22)).unwrap();
    assert_eq!(due(&state, &id, 0), Some(date(2026, 11, 23)));
    state.toggle_todo_on(&id, 0, date(2026, 11, 27)).unwrap();
    assert_eq!(due(&state, &id, 0), Some(date(2026, 11, 28)), "counted from the day it was done, not the missed ones");
    assert_eq!(state.post_todos(&id)[0].completions, 2);
}

#[test]
fn monthly_items_stay_on_their_day_and_their_children_start_over() {
    let mut state = AppState::new_in_memory();
    let id = written(&mut state, "[ ] pay rent ^2026-11-30 ^^monthly\n  [ ] check the bank");
    state.toggle_todo_on(&id, 1, date(2026, 11, 30)).unwrap();
    state.toggle_todo_on(&id, 0, date(2026, 11, 30)).unwrap();
    assert_eq!(due(&state, &id, 0), Some(date(2026, 12, 30)));
    assert_eq!(state.post_todos(&id)[1].state, Open, "the sub-items of the next round are open again");
    state.toggle_todo_on(&id, 1, date(2026, 12, 30)).unwrap();
    state.toggle_todo_on(&id, 0, date(2026, 12, 30)).unwrap();
    assert_eq!(due(&state, &id, 0), Some(date(2027, 1, 30)));
}

#[test]
fn an_unreadable_rule_closes_the_item() {
    let mut state = AppState::new_in_memory();
    let id = written(&mut state, "[ ] something ^^sometimes");
    assert_eq!(state.toggle_todo_on(&id, 0, date(2026, 11, 22)), Ok(Done));
}
