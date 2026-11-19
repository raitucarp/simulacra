//! Per-agent scheduling decision and the list of agents due on a heartbeat tick.

use super::cadence::cadence_rule;
use super::chronotype::posting_chance;
use super::rng::roll_percent;
use super::window::HourWindow;
use crate::domain::traits::Weekday;
use crate::domain::{AgentConfig, Post, ProfileId};
use jiff::{Timestamp, ZonedDateTime};
use std::collections::HashMap;

/// One agent as seen by the scheduler.
#[derive(Debug, Clone, Copy)]
pub struct AgentSlot<'a> {
    pub id: &'a ProfileId,
    pub config: &'a AgentConfig,
    /// When the agent last posted or was last dispatched, if ever.
    pub last_activity: Option<Timestamp>,
}

/// Inputs shared by every decision on one heartbeat tick.
#[derive(Debug, Clone)]
pub struct ScheduleContext {
    /// Current local time (its hour drives the hour windows).
    pub now: ZonedDateTime,
    /// Session seed for the spontaneity rolls.
    pub seed: u64,
    /// Monotonic heartbeat counter, so each tick rolls afresh.
    pub tick: u64,
}

/// Why an agent is, or is not, due on this tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// `is_active` is off.
    Inactive,
    /// The cadence is manual-only.
    ManualOnly,
    /// Today is not one of the agent's active days.
    OffDay,
    /// The local hour is outside the agent's active hours.
    OutsideActiveHours,
    /// The local hour is outside the cadence's slots (e.g. morning/evening).
    OutsideCadenceSlot,
    /// The minimum cadence interval has not elapsed since the last activity.
    CoolingDown,
    /// Eligible, but the spontaneity roll declined this tick.
    Declined,
    /// The agent should post now.
    Due,
}

/// Decides whether `slot` should post on the tick described by `ctx`.
pub fn evaluate(slot: &AgentSlot<'_>, ctx: &ScheduleContext) -> Decision {
    let cfg = slot.config;
    if !cfg.is_active {
        return Decision::Inactive;
    }
    let Some(rule) = cadence_rule(cfg.posting_cadence) else {
        return Decision::ManualOnly;
    };
    if !cfg.active_days.contains(Weekday::from_jiff(ctx.now.weekday())) {
        return Decision::OffDay;
    }
    let hour = ctx.now.hour() as u8;
    if !HourWindow::new(cfg.active_hours_start, cfg.active_hours_end).contains(hour) {
        return Decision::OutsideActiveHours;
    }
    if !rule.allows_hour(hour) {
        return Decision::OutsideCadenceSlot;
    }
    let now_utc = ctx.now.timestamp();
    if slot.last_activity.is_some_and(|last| now_utc.duration_since(last) < rule.min_interval()) {
        return Decision::CoolingDown;
    }
    let chance = posting_chance(cfg.spontaneity_rate, cfg.chronotype, hour);
    if roll_percent(ctx.seed, slot.id.as_str(), ctx.tick) < chance {
        Decision::Due
    } else {
        Decision::Declined
    }
}

/// Ids of the agents due on this tick, in slot order.
pub fn due_agents(slots: &[AgentSlot<'_>], ctx: &ScheduleContext) -> Vec<ProfileId> {
    slots
        .iter()
        .filter(|slot| evaluate(slot, ctx) == Decision::Due)
        .map(|slot| slot.id.clone())
        .collect()
}

/// The most recent post time of every author in `posts`.
pub fn latest_activity(posts: &[Post]) -> HashMap<ProfileId, Timestamp> {
    let mut latest: HashMap<ProfileId, Timestamp> = HashMap::new();
    for post in posts {
        latest
            .entry(post.author_id.clone())
            .and_modify(|at| *at = (*at).max(post.created_at))
            .or_insert(post.created_at);
    }
    latest
}
