use super::{basic, day};
use crate::domain::review::log::{apply_answer, Answer, ReviewEntry, ReviewMode};
use crate::domain::review::schedule::{Phase, Policy};
use crate::domain::review::stats::*;
use crate::domain::review::{Confidence, Grade};
use jiff::Timestamp;

fn entry(on: i64, grade: Grade, phase_before: Phase) -> ReviewEntry {
    let card = basic("c", "q", "a");
    let answer = Answer { item_id: card.id.clone(), grade, mode: ReviewMode::Review, confidence: None, correct: None };
    let (_, entry) = apply_answer(&card, &answer, Timestamp::now(), day(on), &Policy::with_retention_percent(90));
    ReviewEntry { phase_before, ..entry }
}

#[test]
fn the_forecast_counts_overdue_today_and_ignores_new_cards() {
    let mut cards: Vec<_> = (0..4).map(|i| basic(&format!("c{i}"), "q", "a")).collect();
    for (card, due) in cards.iter_mut().zip([-3, 0, 2, 45]).take(4) {
        card.schedule.phase = Phase::Review;
        card.schedule.due = day(due);
    }
    cards.push(basic("new", "q", "a"));
    let forecast = due_forecast(&cards, day(0), FORECAST_DAYS);
    assert_eq!(forecast.len(), FORECAST_DAYS);
    assert_eq!((forecast[0], forecast[2], forecast.iter().sum::<usize>()), (2, 1, 3));
}

#[test]
fn retention_counts_only_reviews_of_learnt_cards() {
    let log = vec![
        entry(0, Grade::Good, Phase::Review),
        entry(-1, Grade::Again, Phase::Review),
        entry(-2, Grade::Hard, Phase::Review),
        entry(0, Grade::Again, Phase::New),
        entry(-10, Grade::Good, Phase::Review),
    ];
    let weeks = retention_by_week(&log, day(0), RETENTION_WEEKS);
    assert_eq!(weeks.len(), RETENTION_WEEKS);
    let this_week = weeks.last().unwrap();
    assert_eq!(this_week.reviews, 3);
    assert!((this_week.retention.unwrap() - 2.0 / 3.0).abs() < 1e-9);
    assert_eq!(weeks[RETENTION_WEEKS - 2].retention, Some(1.0));
    assert_eq!(weeks[0].retention, None);
}

#[test]
fn cards_are_counted_by_state() {
    let mut cards: Vec<_> = (0..5).map(|i| basic(&format!("c{i}"), "q", "a")).collect();
    cards[1].schedule.phase = Phase::Relearning;
    cards[2].schedule.phase = Phase::Review;
    cards[2].schedule.interval = 3;
    cards[3].schedule.phase = Phase::Review;
    cards[3].schedule.interval = MATURE_INTERVAL;
    cards[4].suspended = true;
    let counts = state_counts(&cards);
    assert_eq!(counts.0, [1, 1, 1, 1, 1]);
    assert_eq!((counts.get(ItemState::Mature), counts.total()), (1, 5));
}

#[test]
fn the_heatmap_has_week_columns_and_quartile_levels() {
    let log: Vec<ReviewEntry> = (0..8).map(|_| entry(0, Grade::Good, Phase::Review)).chain([entry(-1, Grade::Good, Phase::Review)]).collect();
    let weeks = review_heatmap(&log, day(0), 4);
    assert_eq!(weeks.len(), 4);
    assert!(weeks.iter().all(|week| week.len() == 7));
    let cells: Vec<&HeatmapCell> = weeks.iter().flatten().collect();
    let today = cells.iter().find(|c| c.day == day(0)).unwrap();
    assert_eq!((today.count, today.level), (8, HEATMAP_LEVELS));
    assert_eq!(cells.iter().find(|c| c.day == day(-1)).unwrap().level, 1);
    assert!(cells.iter().filter(|c| c.day > day(0)).all(|c| c.future && c.count == 0));
    assert_eq!(heatmap_level(0, 10), 0);
}

#[test]
fn calibration_compares_confidence_with_correctness() {
    let checked = |confidence, correct| ReviewEntry { confidence: Some(confidence), correct: Some(correct), ..entry(0, Grade::Good, Phase::Review) };
    let log = vec![checked(Confidence::Sure, true), checked(Confidence::Sure, true), checked(Confidence::Guess, false), entry(0, Grade::Good, Phase::Review)];
    let rows = calibration(&log);
    assert_eq!(rows.iter().map(|r| r.answers).collect::<Vec<_>>(), vec![1, 0, 2]);
    assert_eq!((rows[0].accuracy(), rows[1].accuracy(), rows[2].accuracy()), (Some(0.0), None, Some(1.0)));
}

#[test]
fn the_streak_counts_back_from_today_or_yesterday() {
    let log = vec![entry(-1, Grade::Good, Phase::Review), entry(-2, Grade::Good, Phase::Review), entry(-4, Grade::Good, Phase::Review)];
    assert_eq!(current_streak(&log, day(0)), 2, "today not reviewed yet keeps yesterday's streak");
    let with_today: Vec<ReviewEntry> = log.iter().cloned().chain([entry(0, Grade::Again, Phase::New)]).collect();
    assert_eq!(current_streak(&with_today, day(0)), 3);
    assert_eq!(current_streak(&log, day(5)), 0);
}
