use super::*;
use crate::domain::{AgentConfig, HumanChronotype, PostingCadence, ProfileId};
use chrono::{DateTime, Duration, FixedOffset, TimeZone, Utc};

const SEED: u64 = 42;

/// Local time 2026-11-16 at `hour:minute` in UTC+07:00.
fn local(hour: u32, minute: u32) -> DateTime<FixedOffset> {
    let offset = FixedOffset::east_opt(7 * 3600).expect("valid offset");
    offset.with_ymd_and_hms(2026, 11, 16, hour, minute, 0).single().expect("valid time")
}

/// A config that always posts when eligible (no spontaneity jitter, all-day hours).
fn clockwork(cadence: PostingCadence) -> AgentConfig {
    AgentConfig {
        is_active: true,
        posting_cadence: cadence,
        active_hours_start: 0,
        active_hours_end: 0,
        spontaneity_rate: 0,
        ..AgentConfig::default()
    }
}

fn ctx_at(now: DateTime<FixedOffset>, tick: u64) -> ScheduleContext {
    ScheduleContext { now, seed: SEED, tick }
}

fn decide(cfg: &AgentConfig, last: Option<DateTime<Utc>>, now: DateTime<FixedOffset>) -> Decision {
    let id = ProfileId::new("agent_test");
    evaluate(&AgentSlot { id: &id, config: cfg, last_activity: last }, &ctx_at(now, 0))
}

#[test]
fn hour_window_plain_wrapping_and_full_day() {
    let day = HourWindow::new(7, 23);
    assert!(day.contains(7) && day.contains(22));
    assert!(!day.contains(23) && !day.contains(6) && !day.contains(0));

    let night = HourWindow::new(22, 4);
    assert!(night.contains(22) && night.contains(23) && night.contains(0) && night.contains(3));
    assert!(!night.contains(4) && !night.contains(12) && !night.contains(21));

    let whole = HourWindow::new(9, 9);
    assert!((0..HOURS_PER_DAY).all(|h| whole.contains(h)));
}

#[test]
fn inactive_and_manual_agents_are_never_due() {
    let mut cfg = clockwork(PostingCadence::Hourly);
    cfg.is_active = false;
    assert_eq!(decide(&cfg, None, local(12, 0)), Decision::Inactive);

    let manual = clockwork(PostingCadence::ManualOnly);
    assert_eq!(decide(&manual, None, local(12, 0)), Decision::ManualOnly);
}

#[test]
fn active_hours_window_wrapping_midnight_is_honoured() {
    let mut cfg = clockwork(PostingCadence::Hourly);
    cfg.active_hours_start = 22;
    cfg.active_hours_end = 2;
    assert_eq!(decide(&cfg, None, local(23, 30)), Decision::Due);
    assert_eq!(decide(&cfg, None, local(1, 59)), Decision::Due);
    assert_eq!(decide(&cfg, None, local(2, 0)), Decision::OutsideActiveHours);
    assert_eq!(decide(&cfg, None, local(12, 0)), Decision::OutsideActiveHours);
}

#[test]
fn cadence_intervals_gate_repeat_posts() {
    let now = local(12, 0);
    let cases = [
        (PostingCadence::Every15Minutes, 15),
        (PostingCadence::OnEveryNewPost, 15),
        (PostingCadence::Every30Minutes, 30),
        (PostingCadence::Hourly, 60),
    ];
    for (cadence, minutes) in cases {
        let cfg = clockwork(cadence);
        let now_utc = now.with_timezone(&Utc);
        let just_before = now_utc - Duration::minutes(minutes) + Duration::seconds(1);
        let exactly = now_utc - Duration::minutes(minutes);
        assert_eq!(decide(&cfg, Some(just_before), now), Decision::CoolingDown, "{cadence:?}");
        assert_eq!(decide(&cfg, Some(exactly), now), Decision::Due, "{cadence:?}");
    }
}

#[test]
fn slot_cadences_only_post_inside_their_slots() {
    let twice = clockwork(PostingCadence::MorningAndEvening);
    assert_eq!(decide(&twice, None, local(8, 0)), Decision::Due);
    assert_eq!(decide(&twice, None, local(19, 0)), Decision::Due);
    assert_eq!(decide(&twice, None, local(13, 0)), Decision::OutsideCadenceSlot);

    let owl = clockwork(PostingCadence::NightOwlHours);
    assert_eq!(decide(&owl, None, local(0, 30)), Decision::Due);
    assert_eq!(decide(&owl, None, local(15, 0)), Decision::OutsideCadenceSlot);
}

#[test]
fn chronotype_peak_raises_the_posting_chance() {
    let lark = HumanChronotype::EarlyBirdLark;
    assert!(peak_window(lark).contains(6));
    assert!(posting_chance(80, lark, 6) > posting_chance(80, lark, 20));
    assert_eq!(posting_chance(0, lark, 6), PERCENT_MAX);
    assert!(posting_chance(PERCENT_MAX, HumanChronotype::NightOwlInsomniac, 12) >= 1);
    assert!(peak_window(HumanChronotype::TwilightContemplative).contains(23));
}

#[test]
fn spontaneity_rolls_are_seeded_and_deterministic() {
    let cfg = AgentConfig { spontaneity_rate: PERCENT_MAX, ..clockwork(PostingCadence::Hourly) };
    let id = ProfileId::new("agent_jitter");
    let slot = AgentSlot { id: &id, config: &cfg, last_activity: None };
    let run = |seed| -> Vec<Decision> {
        (0..64).map(|tick| evaluate(&slot, &ScheduleContext { now: local(12, 0), seed, tick })).collect()
    };
    assert_eq!(run(SEED), run(SEED));
    let due = run(SEED).iter().filter(|d| **d == Decision::Due).count();
    assert!(due > 0 && due < 64, "high spontaneity should both post and decline: {due}");
    assert!(run(SEED).contains(&Decision::Declined));
}

#[test]
fn due_agents_filters_slots_and_clock_is_injectable() {
    let ready = clockwork(PostingCadence::Hourly);
    let asleep = AgentConfig { is_active: false, ..clockwork(PostingCadence::Hourly) };
    let (a, b) = (ProfileId::new("a"), ProfileId::new("b"));
    let slots = [
        AgentSlot { id: &a, config: &ready, last_activity: None },
        AgentSlot { id: &b, config: &asleep, last_activity: None },
    ];
    let clock = FixedClock(local(10, 0));
    assert_eq!(due_agents(&slots, &ctx_at(clock.now(), 0)), vec![a]);
}

#[test]
fn queue_is_bounded_and_never_duplicates_an_agent() {
    let mut queue = GenerationQueue::new(2, 1);
    let (a, b, c) = (ProfileId::new("a"), ProfileId::new("b"), ProfileId::new("c"));
    assert_eq!(queue.enqueue(a.clone()), EnqueueOutcome::Queued);
    assert_eq!(queue.enqueue(a.clone()), EnqueueOutcome::AlreadyQueued);
    assert_eq!(queue.enqueue(b.clone()), EnqueueOutcome::Queued);
    assert_eq!(queue.enqueue(c.clone()), EnqueueOutcome::Full);

    assert_eq!(queue.start_next(), Some(a.clone()));
    assert_eq!(queue.start_next(), None, "max one in flight");
    assert_eq!(queue.enqueue(a.clone()), EnqueueOutcome::InFlight);
    assert_eq!((queue.pending_len(), queue.in_flight_len()), (1, 1));

    queue.complete(&a);
    assert_eq!(queue.start_next(), Some(b.clone()));
    assert!(queue.is_in_flight(&b) && !queue.is_in_flight(&a));

    queue.enqueue(c.clone());
    queue.clear_pending();
    assert_eq!((queue.pending_len(), queue.in_flight_len()), (0, 1), "running work is kept");
}

#[test]
fn manual_start_rejects_an_agent_already_in_flight() {
    let mut queue = GenerationQueue::default();
    let a = ProfileId::new("a");
    queue.enqueue(a.clone());
    assert!(queue.start_now(&a));
    assert_eq!(queue.pending_len(), 0, "manual start removes the queued copy");
    assert!(!queue.start_now(&a));
    queue.complete(&a);
    assert!(queue.start_now(&a));
}

#[test]
fn latest_activity_keeps_each_authors_newest_post() {
    let author = crate::domain::Profile::new_agent("agent_x", "X", "@x", "Tester");
    let old = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).single().expect("valid");
    let new = old + Duration::hours(3);
    let post = |at| crate::domain::Post { created_at: at, author_id: author.id.clone(), ..sample_post() };
    let latest = latest_activity(&[post(new), post(old)]);
    assert_eq!(latest.get(&author.id), Some(&new));
}

fn sample_post() -> crate::domain::Post {
    use crate::domain::{NetworkId, PostId, Profile};
    crate::domain::Post {
        id: PostId::new("post_sample"),
        network_id: NetworkId::new("net_test"),
        author_id: Profile::new_primary("u1", "User", "@user").id,
        content: String::new(),
        created_at: Utc::now(),
        likes: 0,
        is_liked: false,
        replies_count: 0,
        mentions: vec![],
        linked_posts: vec![],
        reply_to_id: None,
        quote_id: None,
        repost_id: None,
        link_url: None,
        image_path: None,
        reposts_count: 0,
        is_reposted: false,
    }
}

#[test]
fn off_days_are_never_due() {
    use crate::domain::traits::{OptionSet, Weekday};
    // 2026-11-16 is a Monday.
    let mut cfg = clockwork(PostingCadence::Hourly);
    cfg.active_days = OptionSet::new([Weekday::Tuesday, Weekday::Sunday]);
    assert_eq!(decide(&cfg, None, local(12, 0)), Decision::OffDay);
    cfg.active_days = OptionSet::new([Weekday::Monday]);
    assert_eq!(decide(&cfg, None, local(12, 0)), Decision::Due);
    cfg.active_days = OptionSet::default();
    assert_eq!(decide(&cfg, None, local(12, 0)), Decision::OffDay);
}
