//! D6: ticks, tidied and carried across edits.

use super::super::entries::todo_entries;
use super::super::state::{TodoState, TodoTick};
use super::super::*;

fn tick(index: usize, text: &str, state: EntryState) -> TodoTick {
    TodoTick { index, text: text.into(), state: Some(state), ..TodoTick::default() }
}

#[test]
fn a_tick_applies_only_while_its_item_keeps_its_text() {
    let ticks = TodoState { ticks: vec![tick(0, "milk", EntryState::Done)] };
    let entries = todo_entries("[ ] milk\n[ ] eggs", &ticks);
    assert_eq!((entries[0].state, entries[1].state), (EntryState::Done, EntryState::Open));
    assert_eq!(todo_entries("[ ] oat milk", &ticks)[0].state, EntryState::Open, "another item at that place is untouched");
}

#[test]
fn tidy_drops_ticks_the_text_already_says_and_ticks_of_missing_items() {
    let mut ticks = TodoState { ticks: vec![tick(1, "eggs", EntryState::Done), tick(0, "milk", EntryState::Done), tick(5, "gone", EntryState::Done)] };
    ticks.tidy("[ ] milk\n[x] eggs");
    assert_eq!(ticks.ticks, vec![tick(0, "milk", EntryState::Done)]);
}

#[test]
fn ticks_follow_their_items_through_an_edit() {
    let ticks = TodoState { ticks: vec![tick(0, "milk", EntryState::Done), tick(1, "eggs", EntryState::Dropped)] };
    let carried = ticks.carried("[ ] milk\n[ ] eggs", "[ ] bread\n[ ] eggs\n[ ] milk\n[ ] milk");
    assert_eq!(carried.ticks, vec![tick(1, "eggs", EntryState::Dropped), tick(2, "milk", EntryState::Done)]);
    assert!(ticks.carried("[ ] milk\n[ ] eggs", "nothing to do").is_empty());
}

#[test]
fn ticks_round_trip_through_json_and_an_empty_state_writes_nothing() {
    let ticks = TodoState { ticks: vec![tick(2, "milk", EntryState::Done)] };
    let json = serde_json::to_string(&ticks).unwrap();
    assert_eq!(serde_json::from_str::<TodoState>(&json).unwrap(), ticks);
    assert_eq!(serde_json::from_str::<TodoState>("{}").unwrap(), TodoState::default());
    let extras = crate::domain::post::PostExtras::default();
    assert_eq!(serde_json::to_string(&extras).unwrap(), "{}");
}
