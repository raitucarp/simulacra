//! Agents & Simulation: autonomous posting, its pace and quiet hours, concurrency and how
//! deep reply chains may go.

use super::super::bounds::{Bounds, HOUR_OF_DAY};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Autonomous posts per hour across all agents.
pub const POSTS_PER_HOUR: Bounds = Bounds::new(1, 120, 1);
/// Generations running at once.
pub const MAX_CONCURRENT: Bounds = Bounds::new(1, 4, 1);
/// Replies an agent chain may stack under one post.
pub const REPLY_DEPTH: Bounds = Bounds::new(1, 20, 1);
const DEFAULT_POSTS_PER_HOUR: u32 = 12;
const DEFAULT_QUIET_START: u32 = 22;
const DEFAULT_QUIET_END: u32 = 7;
const DEFAULT_MAX_CONCURRENT: u32 = 1;
const DEFAULT_REPLY_DEPTH: u32 = 6;

/// Simulation preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgentPrefs {
    /// The autonomous scheduler switch (off until the user turns it on).
    pub autonomous_posting: bool,
    /// Consumed by a later wave.
    pub posts_per_hour: u32,
    /// No autonomous posts between `quiet_start` and `quiet_end` (local hours).
    pub quiet_hours: bool,
    pub quiet_start: u32,
    pub quiet_end: u32,
    /// Generations running at once.
    pub max_concurrent: u32,
    /// Consumed by a later wave.
    pub reply_depth_limit: u32,
}

impl Default for AgentPrefs {
    fn default() -> Self {
        Self {
            autonomous_posting: false,
            posts_per_hour: DEFAULT_POSTS_PER_HOUR,
            quiet_hours: false,
            quiet_start: DEFAULT_QUIET_START,
            quiet_end: DEFAULT_QUIET_END,
            max_concurrent: DEFAULT_MAX_CONCURRENT,
            reply_depth_limit: DEFAULT_REPLY_DEPTH,
        }
    }
}

impl AgentPrefs {
    /// Clamps the pace, hours, concurrency and depth into range.
    pub fn validated(self) -> Self {
        Self {
            posts_per_hour: POSTS_PER_HOUR.clamp_u32(self.posts_per_hour),
            quiet_start: HOUR_OF_DAY.clamp_u32(self.quiet_start),
            quiet_end: HOUR_OF_DAY.clamp_u32(self.quiet_end),
            max_concurrent: MAX_CONCURRENT.clamp_u32(self.max_concurrent),
            reply_depth_limit: REPLY_DEPTH.clamp_u32(self.reply_depth_limit),
            ..self
        }
    }

    /// Whether local `hour` falls in the quiet window, which may wrap past midnight (equal
    /// start and end hours mean no quiet window).
    pub fn is_quiet_at(&self, hour: u32) -> bool {
        let (start, end) = (self.quiet_start, self.quiet_end);
        self.quiet_hours
            && match start.cmp(&end) {
                Ordering::Less => (start..end).contains(&hour),
                Ordering::Greater => hour >= start || hour < end,
                Ordering::Equal => false,
            }
    }
}
