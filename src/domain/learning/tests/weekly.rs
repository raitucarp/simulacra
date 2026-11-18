use super::{at, day, item_on_post, network, own_post, session_on};
use crate::domain::learning::gaps::GapEntry;
use crate::domain::learning::weekly::{suggest_actions, weekly_digest, weekly_review_due, NextAction, WeekInputs, WeekWindow, WeeklyReminder};
use crate::domain::review::fsrs::Memory;
use crate::domain::review::schedule::Phase;
use crate::domain::review::{Grade, ItemId, ReviewEntry, ReviewMode};
use chrono::Weekday;

fn entry(item: &str, offset: i64, grade: Grade, phase: Phase) -> ReviewEntry {
    ReviewEntry {
        item_id: ItemId::new(item),
        network_id: network(),
        reviewed_at: at(offset),
        day: day(offset),
        grade,
        mode: ReviewMode::Review,
        confidence: None,
        correct: None,
        phase_before: phase,
        interval: 1,
        stability: 1.0,
        difficulty: 5.0,
        retrievability: None,
    }
}

fn gap(concept: &str, scores: [f64; 6]) -> GapEntry {
    GapEntry { concept: concept.into(), mentions: 1, scores, strength: 0.2 }
}

#[test]
fn the_week_is_the_seven_days_ending_today() {
    let week = WeekWindow::ending(day(10));
    assert_eq!((week.start, week.end), (day(4), day(10)));
    assert!(week.contains(day(4)) && !week.contains(day(3)) && !week.contains(day(11)));
}

#[test]
fn the_digest_looks_back_over_the_week_and_ahead_to_what_is_due() {
    let mut due_soon = item_on_post("i1", "p1", "Q1?");
    due_soon.schedule.phase = Phase::Review;
    due_soon.schedule.memory = Some(Memory { stability: 3.0, difficulty: 5.0 });
    due_soon.schedule.due = day(13);
    let mut due_now = due_soon.clone();
    due_now.id = ItemId::new("i2");
    due_now.schedule.due = day(9);
    let mut later = due_soon.clone();
    later.id = ItemId::new("i3");
    later.schedule.due = day(30);
    let items = [due_soon, due_now, later, item_on_post("i4", "p4", "new one")];
    let log = [entry("i1", 5, Grade::Good, Phase::New), entry("i2", 9, Grade::Again, Phase::Review), entry("i1", 10, Grade::Easy, Phase::Learning), entry("i3", 2, Grade::Good, Phase::New)];
    let posts = [own_post("a", "#necessity today", 9), own_post("b", "#necessity again", 6), own_post("old", "#necessity", 1)];
    let mut session = session_on("p1");
    session.updated_at = at(8);
    let digest = weekly_digest(&WeekInputs { items: &items, log: &log, own_posts: &posts, sessions: &[session], today: day(10) });
    assert_eq!(digest.reviews, 3, "the entry of day 2 is last week's");
    assert_eq!(digest.recall_share, Some(2.0 / 3.0));
    assert_eq!(digest.items_started, 1);
    assert_eq!(digest.recalled, vec![ItemId::new("i1")], "distinct items, most recent first");
    assert_eq!(digest.posts, vec!["a".to_string(), "b".to_string()]);
    assert_eq!(digest.concepts, vec![("necessity".to_string(), 2)]);
    assert_eq!(digest.tutor_sessions, 1);
    assert_eq!((digest.due_today, digest.due_week), (1, 2), "new items are not due");
}

#[test]
fn actions_start_with_due_reviews_then_strengthen_each_weak_concept() {
    let writing = gap("modes", [0.0, 0.5, 0.5, 0.5, 0.5, 0.5]);
    let sources = gap("substance", [0.5, 0.0, 0.5, 0.5, 0.5, 0.5]);
    let mastery = gap("attributes", [0.5, 0.5, 0.5, 0.1, 0.5, 0.5]);
    let extra = gap("extra", [0.0; 6]);
    let actions = suggest_actions(4, &[writing, sources, mastery, extra]);
    assert_eq!(
        actions,
        vec![
            NextAction::ReviewDue { count: 4 },
            NextAction::WritePost { concept: "modes".into() },
            NextAction::ImportSource { concept: "substance".into() },
            NextAction::AskTutor { concept: "attributes".into() },
        ]
    );
    assert_eq!(suggest_actions(0, &[]), vec![]);
    let same = [gap("a", [0.5, 0.0, 0.5, 0.3, 0.5, 0.5]), gap("b", [0.5, 0.0, 0.5, 0.3, 0.6, 0.5])];
    assert_eq!(
        suggest_actions(0, &same),
        vec![NextAction::ImportSource { concept: "a".into() }, NextAction::AskTutor { concept: "b".into() }],
        "a second concept weakest on sources gets its next weakest dimension's action"
    );
    assert_eq!(actions[1].key(), "write_post");
    assert_eq!(actions[0].concept(), None);
}

#[test]
fn the_reminder_shows_once_on_its_day_after_its_hour() {
    let reminder = WeeklyReminder { enabled: true, weekday: Weekday::Sun, hour: 18 };
    let sunday = chrono::NaiveDate::from_ymd_opt(2026, 11, 15).unwrap();
    assert!(weekly_review_due(&reminder, sunday, Weekday::Sun, 18, None));
    assert!(!weekly_review_due(&reminder, sunday, Weekday::Sun, 17, None), "too early");
    assert!(!weekly_review_due(&reminder, sunday, Weekday::Sun, 20, Some(sunday)), "already shown today");
    assert!(!weekly_review_due(&reminder, sunday, Weekday::Mon, 20, None), "another day");
    assert!(!weekly_review_due(&WeeklyReminder { enabled: false, ..reminder }, sunday, Weekday::Sun, 20, None));
}
