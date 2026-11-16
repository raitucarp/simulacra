//! A week of the scheduler, replayed for one config: every heartbeat tick of seven days is
//! run through [`evaluate`] (the same rules the live heartbeat uses), and each tick that
//! comes out [`Decision::Due`] is counted as a post in its day and hour. The Agent Forge
//! shows the result as a calendar strip of when the agent will likely post.

use super::planner::{evaluate, AgentSlot, Decision, ScheduleContext};
use super::window::HOURS_PER_DAY;
use crate::domain::{AgentConfig, ProfileId};
use chrono::{DateTime, Duration, Timelike, Utc};

/// Days in the previewed week.
pub const DAYS_PER_WEEK: usize = 7;
/// Hours in one previewed day.
const HOURS: usize = HOURS_PER_DAY as usize;

/// Simulated post counts per hour of each day of one week (day 0 is the first).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WeekPreview {
    pub posts: [[u16; HOURS]; DAYS_PER_WEEK],
}

impl WeekPreview {
    /// Posts on day `day`.
    pub fn day_total(&self, day: usize) -> u32 {
        self.posts.get(day).map_or(0, |hours| hours.iter().map(|&n| u32::from(n)).sum())
    }

    /// Posts in the whole week.
    pub fn total(&self) -> u32 {
        (0..DAYS_PER_WEEK).map(|day| self.day_total(day)).sum()
    }

    /// The largest count in any single hour (for shading).
    pub fn busiest_hour(&self) -> u16 {
        self.posts.iter().flatten().copied().max().unwrap_or_default()
    }
}

/// Replays one week of heartbeat ticks spaced `tick` apart for an agent with `cfg`, starting
/// at midnight, with the session `seed` driving the spontaneity rolls. A post resets the
/// cooldown, exactly as a dispatch does on the live heartbeat.
pub fn simulate_week(cfg: &AgentConfig, id: &ProfileId, tick: Duration, seed: u64) -> WeekPreview {
    let mut preview = WeekPreview::default();
    if tick <= Duration::zero() {
        return preview;
    }
    let start = DateTime::<Utc>::UNIX_EPOCH;
    let end = start + Duration::days(DAYS_PER_WEEK as i64);
    let mut last_activity = None;
    let (mut now, mut count) = (start, 0_u64);
    while now < end {
        let ctx = ScheduleContext { now: now.fixed_offset(), seed, tick: count };
        if evaluate(&AgentSlot { id, config: cfg, last_activity }, &ctx) == Decision::Due {
            last_activity = Some(now);
            let day = (now - start).num_days() as usize;
            if let Some(slot) = preview.posts.get_mut(day).and_then(|hours| hours.get_mut(now.hour() as usize)) {
                *slot += 1;
            }
        }
        now += tick;
        count += 1;
    }
    preview
}

#[cfg(test)]
mod tests;
