use super::day;
use crate::domain::review::day::{review_day, DAY_ROLLOVER_HOUR};
use crate::domain::review::schedule::{preview, review, CardSchedule, Phase, Policy, SAME_DAY};
use crate::domain::review::Grade;
use chrono::{FixedOffset, NaiveDate, TimeZone};

fn policy() -> Policy {
    Policy::with_retention_percent(90)
}

/// Reviews a new card with `grades`, each on its due day.
fn run(grades: &[Grade], policy: &Policy) -> Vec<CardSchedule> {
    grades
        .iter()
        .scan(CardSchedule::new_card(day(0)), |schedule, grade| {
            let today = schedule.due.max(day(0));
            *schedule = review(schedule, *grade, today, policy);
            Some(schedule.clone())
        })
        .collect()
}

#[test]
fn intervals_grow_with_each_good_review() {
    let history = run(&[Grade::Good; 6], &policy());
    let intervals: Vec<u32> = history.iter().map(|s| s.interval).collect();
    assert!(intervals.windows(2).all(|w| w[1] > w[0]), "{intervals:?}");
    assert!(history.iter().all(|s| s.phase == Phase::Review));
    assert_eq!(history.last().unwrap().reps, 6);
}

#[test]
fn a_lapse_shrinks_stability_and_counts() {
    let history = run(&[Grade::Good, Grade::Good, Grade::Good, Grade::Again], &policy());
    let (before, after) = (&history[2], &history[3]);
    assert!(after.memory.unwrap().stability < before.memory.unwrap().stability);
    assert_eq!((after.phase, after.lapses, after.interval), (Phase::Relearning, 1, SAME_DAY));
    assert_eq!(after.due, after.last_review.unwrap(), "due again the same day");
    let relearnt = review(after, Grade::Good, after.due, &policy());
    assert_eq!(relearnt.phase, Phase::Review);
    assert!(relearnt.interval < before.interval, "relearnt intervals restart short");
}

#[test]
fn the_retention_target_changes_intervals() {
    let card = &run(&[Grade::Good, Grade::Good], &policy())[1];
    let strict = preview(card, card.due, &Policy::with_retention_percent(97))[Grade::Good.index()];
    let normal = preview(card, card.due, &policy())[Grade::Good.index()];
    let relaxed = preview(card, card.due, &Policy::with_retention_percent(75))[Grade::Good.index()];
    assert!(strict < normal && normal < relaxed, "{strict} < {normal} < {relaxed}");
}

#[test]
fn the_buttons_promise_ordered_intervals() {
    for card in run(&[Grade::Hard, Grade::Good, Grade::Easy, Grade::Again], &policy()) {
        let [again, hard, good, easy] = preview(&card, card.due, &policy());
        assert_eq!(again, SAME_DAY);
        assert!(1 <= hard && hard <= good && good < easy, "{hard} {good} {easy}");
    }
}

#[test]
fn a_new_card_answered_again_stays_in_learning() {
    let after = review(&CardSchedule::new_card(day(0)), Grade::Again, day(0), &policy());
    assert_eq!((after.phase, after.lapses, after.due), (Phase::Learning, 0, day(0)));
}

#[test]
fn intervals_never_exceed_the_cap() {
    let capped = Policy { maximum_interval: 30, ..policy() };
    let history = run(&[Grade::Easy; 8], &capped);
    assert!(history.iter().all(|s| s.interval <= 30));
    assert_eq!(history.last().unwrap().interval, 30);
}

#[test]
fn the_review_day_rolls_over_after_the_small_hours() {
    let zone = FixedOffset::east_opt(7 * 3600).unwrap();
    let late = zone.with_ymd_and_hms(2026, 3, 2, (DAY_ROLLOVER_HOUR - 1) as u32, 30, 0).unwrap();
    let early = zone.with_ymd_and_hms(2026, 3, 2, DAY_ROLLOVER_HOUR as u32, 0, 0).unwrap();
    assert_eq!(review_day(&late), NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(), "3:30 still counts as yesterday");
    assert_eq!(review_day(&early), NaiveDate::from_ymd_opt(2026, 3, 2).unwrap());
}

#[test]
fn overdue_reviews_use_the_real_elapsed_time() {
    let card = &run(&[Grade::Good, Grade::Good], &policy())[1];
    let on_time = review(card, Grade::Good, card.due, &policy());
    let late = review(card, Grade::Good, card.due + chrono::Duration::days(20), &policy());
    assert!(late.memory.unwrap().stability > on_time.memory.unwrap().stability, "a harder recall strengthens more");
    assert!(card.retrievability(card.due + chrono::Duration::days(20), &policy().params).unwrap() < 0.9);
}
