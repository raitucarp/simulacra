use super::*;
use crate::domain::{HumanChronotype, PostingCadence};

const SEED: u64 = 7;

fn minute() -> Duration {
    Duration::minutes(1)
}

fn agent() -> ProfileId {
    ProfileId::new("agent_preview")
}

fn clockwork(cadence: PostingCadence, start: u8, end: u8) -> AgentConfig {
    AgentConfig {
        is_active: true,
        posting_cadence: cadence,
        active_hours_start: start,
        active_hours_end: end,
        spontaneity_rate: 0,
        ..AgentConfig::default()
    }
}

#[test]
fn an_inactive_or_manual_agent_never_posts() {
    let off = AgentConfig { is_active: false, ..clockwork(PostingCadence::Hourly, 0, 0) };
    assert_eq!(simulate_week(&off, &agent(), minute(), SEED).total(), 0);
    let manual = clockwork(PostingCadence::ManualOnly, 0, 0);
    assert_eq!(simulate_week(&manual, &agent(), minute(), SEED).total(), 0);
}

#[test]
fn a_clockwork_hourly_agent_posts_once_per_active_hour() {
    let cfg = clockwork(PostingCadence::Hourly, 9, 17);
    let week = simulate_week(&cfg, &agent(), minute(), SEED);
    for day in 0..DAYS_PER_WEEK {
        assert_eq!(week.day_total(day), 8, "day {day}");
        assert_eq!(week.posts[day][8], 0);
        assert_eq!(week.posts[day][9], 1);
        assert_eq!(week.posts[day][16], 1);
        assert_eq!(week.posts[day][17], 0);
    }
    assert_eq!(week.total(), 56);
    assert_eq!(week.busiest_hour(), 1);
}

#[test]
fn active_hours_that_wrap_past_midnight_are_respected() {
    let cfg = clockwork(PostingCadence::Hourly, 22, 2);
    let week = simulate_week(&cfg, &agent(), minute(), SEED);
    for hour in [22, 23, 0, 1] {
        assert_eq!(week.posts[3][hour], 1, "hour {hour}");
    }
    assert_eq!(week.posts[3][2], 0);
    assert_eq!(week.posts[3][12], 0);
}

#[test]
fn morning_and_evening_posts_only_in_its_slots() {
    let cfg = clockwork(PostingCadence::MorningAndEvening, 0, 0);
    let week = simulate_week(&cfg, &agent(), minute(), SEED);
    for day in 0..DAYS_PER_WEEK {
        assert_eq!(week.posts[day][7], 1);
        assert_eq!(week.posts[day][18], 1);
        assert_eq!(week.day_total(day), 2);
    }
}

#[test]
fn spontaneity_spreads_posts_out_and_is_deterministic() {
    let steady = clockwork(PostingCadence::Every30Minutes, 0, 0);
    let jittery = AgentConfig { spontaneity_rate: 100, chronotype: HumanChronotype::EarlyBirdLark, ..steady.clone() };
    let steady_week = simulate_week(&steady, &agent(), minute(), SEED);
    let jittery_week = simulate_week(&jittery, &agent(), minute(), SEED);
    assert!(jittery_week.total() < steady_week.total());
    assert!(jittery_week.total() > 0);
    assert_eq!(jittery_week, simulate_week(&jittery, &agent(), minute(), SEED));
}

#[test]
fn a_non_positive_tick_previews_nothing() {
    let cfg = clockwork(PostingCadence::Hourly, 0, 0);
    assert_eq!(simulate_week(&cfg, &agent(), Duration::zero(), SEED), WeekPreview::default());
}

#[test]
fn the_week_starts_on_monday_and_skips_off_days() {
    use crate::domain::traits::{OptionSet, Weekday};
    let days = OptionSet::new([Weekday::Monday, Weekday::Sunday]);
    let cfg = AgentConfig { active_days: days, ..clockwork(PostingCadence::Hourly, 9, 17) };
    let week = simulate_week(&cfg, &agent(), minute(), SEED);
    assert_eq!((week.day_total(0), week.day_total(6)), (8, 8));
    for day in 1..6 {
        assert_eq!(week.day_total(day), 0, "day {day}");
    }
}
