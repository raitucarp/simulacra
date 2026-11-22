//! D5: a body as the checklist renders it.

use super::super::entries::{body_parts, todo_entries, BodyPart};
use super::super::state::TodoState;
use super::super::*;
use jiff::civil::date;

const BODY: &str = "Weekend\n[ ] clean ^sat !!\n  [x] kitchen\n  [ ] garage &home\nThen rest.\n\n[ ] call mum ^fri";

#[test]
fn entries_are_flat_with_depth_parent_and_children() {
    let entries = todo_entries(BODY, &TodoState::default());
    let shape: Vec<_> = entries.iter().map(|e| (e.item.text.as_str(), e.depth, e.parent)).collect();
    assert_eq!(shape, vec![("clean", 0, None), ("kitchen", 1, Some(0)), ("garage", 1, Some(0)), ("call mum", 0, None)]);
    assert_eq!(entries[0].children, vec![1, 2]);
    assert_eq!(entries[0].settled_children, 1, "the kitchen is done, the garage is not");
    assert!(entries.iter().all(|e| e.item.children.is_empty()));
    assert_eq!(entries[1].state, EntryState::Done);
}

#[test]
fn the_body_is_cut_into_text_runs_and_items() {
    let parts = body_parts(BODY);
    let expected = vec![
        BodyPart::Text("Weekend".into()),
        BodyPart::Item(0),
        BodyPart::Item(1),
        BodyPart::Item(2),
        BodyPart::Text("Then rest.".into()),
        BodyPart::Item(3),
    ];
    assert_eq!(parts, expected);
    assert_eq!(body_parts("no to-dos here"), vec![BodyPart::Text("no to-dos here".into())]);
    assert!(body_parts("[ ] only").iter().all(|part| matches!(part, BodyPart::Item(_))));
}

#[test]
fn a_fenced_box_stays_in_the_text() {
    let parts = body_parts("```\n[ ] not a task\n```\n[ ] a task");
    assert_eq!(parts, vec![BodyPart::Text("```\n[ ] not a task\n```".into()), BodyPart::Item(0)]);
}

#[test]
fn due_days_count_from_the_day_written_and_overdue_needs_an_open_item() {
    let entries = todo_entries(BODY, &TodoState::default());
    let written = date(2026, 11, 22); // a Sunday
    assert_eq!(entries[0].due(written), Some(date(2026, 11, 28)));
    assert_eq!(entries[3].due(written), Some(date(2026, 11, 27)));
    assert_eq!(entries[1].due(written), None);
    assert!(!entries[3].is_overdue(written, date(2026, 11, 27)), "due today is not overdue");
    assert!(entries[3].is_overdue(written, date(2026, 11, 28)));
    let done = todo_entries("[x] paid ^fri", &TodoState::default()).remove(0);
    assert!(!done.is_overdue(written, date(2026, 12, 25)), "a done item is never overdue");
}
