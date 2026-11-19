use super::{basic, day};
use crate::domain::review::item::{Anchor, AnchorSide, ReviewItem, Subject};
use crate::domain::review::log::{apply_answer, Answer, ReviewEntry, ReviewMode};
use crate::domain::review::queue::{build_queue, plan_today, DailyLimits};
use crate::domain::review::schedule::Policy;
use crate::domain::review::{Grade, Phase};
use chrono::Utc;

/// `card` reviewed Good on day 0, so it is learnt and due later.
fn learnt(card: ReviewItem, due_offset: i64) -> ReviewItem {
    let (mut card, _) = apply_answer(&card, &answer(&card, Grade::Good), Utc::now(), day(0), &Policy::with_retention_percent(90));
    card.schedule.due = day(due_offset);
    card
}

fn answer(card: &ReviewItem, grade: Grade) -> Answer {
    Answer { item_id: card.id.clone(), grade, mode: ReviewMode::Review, confidence: None, correct: None }
}

fn deck(due: usize, new: usize) -> Vec<ReviewItem> {
    let due_cards = (0..due).map(|i| learnt(basic(&format!("d{i}"), &format!("due {i}"), "a"), -(i as i64)));
    let new_cards = (0..new).map(|i| basic(&format!("n{i}"), &format!("new {i}"), "a"));
    due_cards.chain(new_cards).collect()
}

const LIMITS: DailyLimits = DailyLimits { reviews: 200, new_cards: 20 };

#[test]
fn the_queue_holds_due_cards_and_rations_new_ones() {
    let cards = deck(5, 30);
    let plan = plan_today(&cards, &[], day(1), LIMITS);
    assert_eq!((plan.due, plan.new_waiting, plan.queued_due, plan.queued_new), (5, 30, 5, 20));
    assert_eq!(build_queue(&cards, &[], day(1), LIMITS).len(), 25);
}

#[test]
fn the_review_limit_caps_new_cards_too() {
    let cards = deck(8, 30);
    let plan = plan_today(&cards, &[], day(1), DailyLimits { reviews: 10, new_cards: 20 });
    assert_eq!((plan.queued_due, plan.queued_new), (8, 2));
}

#[test]
fn todays_answers_use_up_the_limits() {
    let cards = deck(0, 30);
    let log: Vec<ReviewEntry> = cards[..15]
        .iter()
        .map(|card| apply_answer(card, &answer(card, Grade::Good), Utc::now(), day(0), &Policy::with_retention_percent(90)).1)
        .collect();
    assert!(log.iter().all(|e| e.phase_before == Phase::New));
    let plan = plan_today(&cards[15..], &log, day(0), LIMITS);
    assert_eq!((plan.answered, plan.introduced, plan.queued_new), (15, 15, 5));
    assert_eq!(plan_today(&cards[15..], &log, day(1), LIMITS).queued_new, 15, "a new day brings a new ration");
}

#[test]
fn new_cards_are_interleaved_and_sources_spread_apart() {
    let mut cards = deck(6, 2);
    cards.iter_mut().take(3).for_each(|card| card.subject = Subject::Anchored { anchor: Anchor::post(&crate::domain::PostId::new("same")), side: AnchorSide::Back, written: "q".into() });
    let queue = build_queue(&cards, &[], day(1), LIMITS);
    let positions: Vec<usize> = queue.iter().enumerate().filter(|(_, id)| id.as_str().starts_with('n')).map(|(i, _)| i).collect();
    assert!(positions[0] > 0 && positions[1] > positions[0] + 1, "new cards spread out: {positions:?}");
    let group = |i: usize| cards.iter().find(|c| c.id == queue[i]).unwrap().group_key();
    assert!((1..queue.len()).all(|i| group(i) != group(i - 1)), "no two neighbours share a source");
}

#[test]
fn suspended_and_future_cards_stay_out() {
    let mut cards = deck(3, 1);
    cards[0].suspended = true;
    cards[1].schedule.due = day(9);
    assert_eq!(plan_today(&cards, &[], day(1), LIMITS).due, 1);
}

#[test]
fn retired_items_stay_out() {
    let mut cards = deck(2, 2);
    cards[0].retired = true;
    cards[3].retired = true;
    let plan = plan_today(&cards, &[], day(1), LIMITS);
    assert_eq!((plan.due, plan.new_waiting), (1, 1));
}
