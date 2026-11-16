//! Runtime state of the autonomous scheduler: the switch, generation queue, heartbeat
//! task, the most recent LLM failure and the failure dedupe / backoff gate.

use crate::agent::schedule::{FailureGate, GenerationQueue};
use crate::agent::LlmFailure;
use crate::domain::ProfileId;
use chrono::{DateTime, Utc};
use gpui_kit::Task;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// How often the heartbeat asks the scheduling core which agents are due.
pub const HEARTBEAT_INTERVAL: Duration = crate::agent::schedule::HEARTBEAT_INTERVAL;

/// Scheduler bookkeeping owned by the root view.
pub struct SchedulerState {
    /// Global on/off switch (persisted in the settings table).
    pub enabled: bool,
    /// Bounded queue of agents awaiting or undergoing generation.
    pub queue: GenerationQueue,
    /// When each agent was last dispatched, so a failed run still respects its cadence.
    pub last_dispatch: HashMap<ProfileId, DateTime<Utc>>,
    /// Heartbeat counter feeding the spontaneity rolls.
    pub tick: u64,
    /// Per-session seed for the spontaneity rolls.
    pub seed: u64,
    /// The most recent LLM failure, shown in the settings General pane.
    pub last_error: Option<LlmFailure>,
    /// Which failure notices were shown, and the backoff after failed generations.
    pub failures: FailureGate,
    /// The running heartbeat loop; dropping it stops the heartbeat.
    pub(super) heartbeat: Option<Task<()>>,
}

impl SchedulerState {
    /// Fresh state with the persisted switch value and a time-derived seed.
    pub(super) fn new(enabled: bool) -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|elapsed| elapsed.as_nanos() as u64)
            .unwrap_or_default();
        Self {
            enabled,
            queue: GenerationQueue::default(),
            last_dispatch: HashMap::new(),
            tick: 0,
            seed,
            last_error: None,
            failures: FailureGate::default(),
            heartbeat: None,
        }
    }

    /// The later of the agent's last post and its last dispatch.
    pub fn last_activity(&self, id: &ProfileId, last_post: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
        let dispatched = self.last_dispatch.get(id).copied();
        last_post.max(dispatched)
    }
}
