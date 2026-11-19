//! A card's schedule and how one review moves it: the next memory state, the interval for
//! the desired retention, and the four intervals a learner can choose between.

use super::day::{add_days, days_between};
use super::fsrs::{self, Memory, Params};
use super::Grade;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

/// Longest interval, in days (a hundred years).
pub const MAXIMUM_INTERVAL: u32 = 36_500;
/// Interval of a card that is due again in the same session.
pub const SAME_DAY: u32 = 0;
/// Shortest interval after a successful review.
const MINIMUM_INTERVAL: u32 = 1;
const PERCENT: f64 = 100.0;

/// Where a card is in its life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    /// Never reviewed.
    #[default]
    New,
    /// Seen, but not yet recalled on a later day.
    Learning,
    /// Recalled at spaced intervals.
    Review,
    /// Forgotten after being learnt.
    Relearning,
}

/// The persisted schedule of one card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemSchedule {
    pub phase: Phase,
    pub memory: Option<Memory>,
    /// Review day the card is due on.
    pub due: Date,
    pub last_review: Option<Date>,
    /// Days between the last review and `due`.
    pub interval: u32,
    pub reps: u32,
    pub lapses: u32,
}

impl ItemSchedule {
    /// A card nobody has reviewed, available from `today`.
    pub fn new_item(today: Date) -> Self {
        Self { phase: Phase::New, memory: None, due: today, last_review: None, interval: 0, reps: 0, lapses: 0 }
    }

    pub fn is_new(&self) -> bool {
        self.phase == Phase::New
    }

    /// Whether the card should be reviewed on `today` (new cards are introduced separately).
    pub fn is_due(&self, today: Date) -> bool {
        !self.is_new() && self.due <= today
    }

    /// Probability of recalling the card on `today`, once it has a memory.
    pub fn retrievability(&self, today: Date, params: &Params) -> Option<f64> {
        let (memory, last) = (self.memory?, self.last_review?);
        Some(fsrs::retrievability(params, f64::from(days_between(last, today)), memory.stability))
    }
}

/// The weights, the desired retention and the interval cap a deck is scheduled with.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Policy {
    pub params: Params,
    /// Target probability of recall when a card comes due (0–1).
    pub desired_retention: f64,
    pub maximum_interval: u32,
}

impl Policy {
    /// Default weights at `percent` desired retention (Settings › Learning).
    pub fn with_retention_percent(percent: u32) -> Self {
        Self { params: Params::default(), desired_retention: f64::from(percent) / PERCENT, maximum_interval: MAXIMUM_INTERVAL }
    }

    /// Whole days until `memory` reaches the desired retention, within the bounds.
    fn interval_for(&self, memory: Memory) -> u32 {
        let days = fsrs::interval_days(&self.params, memory.stability, self.desired_retention).round();
        (days.max(f64::from(MINIMUM_INTERVAL)) as u32).min(self.maximum_interval)
    }
}

/// The memory after answering `grade` on `today`.
pub fn next_memory(schedule: &ItemSchedule, grade: Grade, today: Date, params: &Params) -> Memory {
    let (Some(memory), Some(last)) = (schedule.memory, schedule.last_review) else {
        return Memory { stability: fsrs::initial_stability(params, grade), difficulty: fsrs::initial_difficulty(params, grade) };
    };
    let difficulty = fsrs::next_difficulty(params, memory.difficulty, grade);
    let elapsed = days_between(last, today);
    let stability = if elapsed == 0 {
        fsrs::same_day_stability(params, memory.stability, grade)
    } else {
        let r = fsrs::retrievability(params, f64::from(elapsed), memory.stability);
        match grade {
            Grade::Again => fsrs::lapse_stability(params, memory.difficulty, memory.stability, r),
            _ => fsrs::recall_stability(params, memory.difficulty, memory.stability, r, grade),
        }
    };
    Memory { stability, difficulty }
}

/// Intervals of the four grades, kept in order (Hard ≤ Good < Easy) as the buttons promise.
pub fn preview(schedule: &ItemSchedule, today: Date, policy: &Policy) -> [u32; 4] {
    let raw = Grade::ALL.map(|grade| match grade {
        Grade::Again => SAME_DAY,
        _ => policy.interval_for(next_memory(schedule, grade, today, &policy.params)),
    });
    let good = raw[Grade::Good.index()].max(raw[Grade::Hard.index()]);
    let hard = raw[Grade::Hard.index()].min(good);
    let easy = raw[Grade::Easy.index()].max(good + 1).min(policy.maximum_interval.max(good));
    [SAME_DAY, hard, good, easy]
}

/// The schedule after answering `grade` on `today`.
pub fn review(schedule: &ItemSchedule, grade: Grade, today: Date, policy: &Policy) -> ItemSchedule {
    let memory = next_memory(schedule, grade, today, &policy.params);
    let interval = preview(schedule, today, policy)[grade.index()];
    let was_learnt = matches!(schedule.phase, Phase::Review | Phase::Relearning);
    let phase = match (grade, was_learnt) {
        (Grade::Again, true) => Phase::Relearning,
        (Grade::Again, false) => Phase::Learning,
        _ => Phase::Review,
    };
    let lapses = schedule.lapses + u32::from(grade == Grade::Again && schedule.phase == Phase::Review);
    ItemSchedule {
        phase,
        memory: Some(memory),
        due: add_days(today, interval),
        last_review: Some(today),
        interval,
        reps: schedule.reps + 1,
        lapses,
    }
}
