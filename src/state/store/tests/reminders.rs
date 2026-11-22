//! D10: due reminders on the lifecycle heartbeat, once per due day.

use crate::agent::schedule::FixedClock;
use crate::domain::{Post, PostId};
use crate::state::store::AppState;
use jiff::civil::date;
use jiff::tz::TimeZone;

/// A post of the user's written at noon UTC on Sunday 22 November 2026.
fn written(state: &mut AppState, text: &str) -> PostId {
    let id = state.submit_post(text).unwrap();
    state.posts.iter_mut().find(|post| post.id == id).unwrap().created_at = crate::time::utc(2026, 11, 22, 12, 0, 0);
    id
}

fn texts(state: &mut AppState, day: jiff::civil::Date) -> Vec<String> {
    state.remind_due(day).unwrap().into_iter().map(|reminder| reminder.text).collect()
}

#[test]
fn items_due_today_or_overdue_are_reminded_of_once() {
    let mut state = AppState::new_in_memory();
    written(&mut state, "[ ] call mum ^tue\n[ ] pay rent ^mon\n[x] done ^mon\n[ ] later ^fri\n[ ] no date");
    assert_eq!(texts(&mut state, date(2026, 11, 22)), Vec::<String>::new());
    assert_eq!(texts(&mut state, date(2026, 11, 24)), vec!["call mum", "pay rent"]);
    assert!(texts(&mut state, date(2026, 11, 24)).is_empty(), "once per due day");
    assert!(texts(&mut state, date(2026, 11, 25)).is_empty(), "an overdue item is not reminded of every day");
    assert_eq!(texts(&mut state, date(2026, 11, 27)), vec!["later"]);
    assert!(state.get_post(&state.posts[0].id.clone()).unwrap().extras.head.is_none(), "no version");
}

#[test]
fn a_repeating_item_is_reminded_of_on_each_of_its_days() {
    let mut state = AppState::new_in_memory();
    let id = written(&mut state, "[ ] stretch ^mon ^^weekly");
    assert_eq!(texts(&mut state, date(2026, 11, 23)), vec!["stretch"]);
    state.toggle_todo_on(&id, 0, date(2026, 11, 23)).unwrap();
    assert!(texts(&mut state, date(2026, 11, 29)).is_empty());
    assert_eq!(texts(&mut state, date(2026, 11, 30)), vec!["stretch"]);
}

#[test]
fn agents_to_dos_are_not_the_users_and_the_heartbeat_reports_reminders() {
    let mut state = AppState::new_in_memory();
    let agent = state.user_profiles.iter().find(|p| p.is_agent()).cloned().unwrap();
    let mut theirs = Post::create("agent_list", "net1", agent.id.as_str(), "[ ] theirs ^today").unwrap();
    theirs.created_at = crate::time::utc(2026, 11, 22, 12, 0, 0);
    state.commit_post(theirs).unwrap();
    written(&mut state, "[ ] mine ^today");
    let clock = FixedClock(crate::time::utc(2026, 11, 22, 18, 0, 0).to_zoned_datetime(TimeZone::UTC));
    let report = state.lifecycle_heartbeat(&clock).unwrap();
    assert_eq!(report.reminders.iter().map(|r| r.text.as_str()).collect::<Vec<_>>(), vec!["mine"]);
    assert!(!report.is_empty());
    assert!(state.lifecycle_heartbeat(&clock).unwrap().is_empty());
}
