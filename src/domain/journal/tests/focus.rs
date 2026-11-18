use super::{day, noon};
use crate::domain::journal::focus::stats::{minutes_by_day, newest_sessions, totals};
use crate::domain::journal::focus::timer::{clock_text, whole_minutes, MS_PER_MINUTE, SUSPEND_GAP_MS};
use crate::domain::journal::focus::{custom_minutes, FocusSession, FocusTimer, Outcome, Phase};
use crate::domain::journal::RecordId;

const SECOND: i64 = 1_000;

#[test]
fn runs_and_finishes_at_the_planned_length() {
    let timer = FocusTimer::start(25, 0);
    let half = (1..=12 * 60).map(|s| s * SECOND).fold(timer, FocusTimer::tick);
    assert!(half.is_running());
    assert_eq!(half.remaining(12 * MS_PER_MINUTE), 13 * MS_PER_MINUTE);
    let ticks = (1..=25 * 60).map(|s| s * SECOND).fold(timer, FocusTimer::tick);
    assert_eq!(ticks.phase, Phase::Finished { early: false });
    assert_eq!((ticks.elapsed(i64::MAX / 2), ticks.progress(0)), (25 * MS_PER_MINUTE, 1.0));
}

#[test]
fn pause_and_resume_bank_only_running_time() {
    let t = FocusTimer::start(50, 0).pause(10 * MS_PER_MINUTE);
    assert_eq!(t.phase, Phase::Paused { suspended: false });
    assert_eq!(t.elapsed(40 * MS_PER_MINUTE), 10 * MS_PER_MINUTE, "paused time does not count");
    let resumed = t.resume(40 * MS_PER_MINUTE);
    assert_eq!(resumed.elapsed(45 * MS_PER_MINUTE), 15 * MS_PER_MINUTE);
    assert_eq!(resumed.resume(0), resumed, "resuming a running timer changes nothing");
    assert_eq!(t.pause(99), t, "pausing twice changes nothing");
}

#[test]
fn stopping_early_and_late() {
    let early = FocusTimer::start(25, 0).stop(5 * MS_PER_MINUTE);
    assert_eq!((early.phase, early.elapsed(0)), (Phase::Finished { early: true }, 5 * MS_PER_MINUTE));
    assert_eq!(early.tick(99 * MS_PER_MINUTE), early, "a finished timer ignores ticks");
    let late = FocusTimer::start(1, 0).stop(5 * MS_PER_MINUTE);
    assert_eq!(late.phase, Phase::Finished { early: false });
}

#[test]
fn skipped_ticks_lose_nothing_and_a_sleep_pauses() {
    let t = FocusTimer::start(90, 0).tick(SECOND).tick(SUSPEND_GAP_MS);
    assert_eq!(t.elapsed(SUSPEND_GAP_MS), SUSPEND_GAP_MS, "a slow repaint still counts wall-clock time");
    let slept = t.tick(SUSPEND_GAP_MS + 30 * MS_PER_MINUTE);
    assert_eq!(slept.phase, Phase::Paused { suspended: true });
    assert_eq!(slept.elapsed(0), SUSPEND_GAP_MS, "the sleep itself is not focus");
}

#[test]
fn a_clock_moving_back_counts_no_time() {
    let t = FocusTimer::start(25, 10 * MS_PER_MINUTE).tick(12 * MS_PER_MINUTE);
    let back = t.tick(MS_PER_MINUTE);
    assert_eq!(back.elapsed(MS_PER_MINUTE), 2 * MS_PER_MINUTE, "kept what was counted before the jump");
    assert_eq!(back.tick(2 * MS_PER_MINUTE).elapsed(2 * MS_PER_MINUTE), 3 * MS_PER_MINUTE, "and counts on from the new reading");
}

#[test]
fn text_and_minutes() {
    assert_eq!((clock_text(25 * MS_PER_MINUTE), clock_text(61 * SECOND), clock_text(90 * MS_PER_MINUTE)), ("25:00".into(), "1:01".into(), "1:30:00".into()));
    assert_eq!((whole_minutes(89 * SECOND), whole_minutes(-5)), (1, 0));
    assert_eq!((custom_minutes(" 45 "), custom_minutes("0"), custom_minutes("999"), custom_minutes("x")), (Some(45), None, None, None));
}

fn session(id: &str, offset: i64, minutes: u32, outcome: Option<Outcome>) -> FocusSession {
    FocusSession { id: RecordId::new(id), day: day(offset), started_at: noon(offset), ended_at: noon(offset), intention: "x".into(), planned_minutes: 25, focused_minutes: minutes, link: None, distractions: 0, outcome, reflection: String::new() }
}

#[test]
fn log_statistics() {
    let log = vec![session("a", 0, 25, Some(Outcome::Done)), session("b", 0, 20, Some(Outcome::Partly)), session("c", 1, 50, None)];
    let t = totals(&log);
    assert_eq!((t.sessions, t.minutes, t.done, t.partly, t.not_done, t.done_share), (3, 95, 1, 1, 0, Some(0.5)));
    assert_eq!(minutes_by_day(&log).get(&day(0)), Some(&45));
    assert_eq!(newest_sessions(&log)[0].id.as_str(), "c");
    assert_eq!(totals(&[]).done_share, None);
}
