//! Pure autonomous-posting scheduler core (no UI, no I/O).
//!
//! Given each agent's [`crate::domain::AgentConfig`], the current local time and the
//! time of its last activity, decides which agents are due to post. Every input that
//! varies at runtime (the clock, the random seed, the heartbeat tick) is injected so
//! decisions are fully deterministic under test.
//!
//! - [`window`]: hour-of-day windows, including windows that wrap past midnight.
//! - [`cadence`]: posting cadence → minimum interval and allowed hours.
//! - [`chronotype`]: chronotype peak hours and the spontaneity-based posting chance.
//! - [`rng`]: a seeded, stateless percentage roll.
//! - [`clock`]: the injectable wall clock.
//! - [`planner`]: the per-agent decision and the due-agent list.
//! - [`queue`]: the bounded generation queue with per-agent in-flight tracking.
//! - [`backoff`]: one notice per distinct LLM failure and a growing pause after failures.
//! - [`preview`]: a simulated week of posts for one config (the Agent Forge calendar).

mod backoff;
mod cadence;
mod chronotype;
mod clock;
mod planner;
mod preview;
mod queue;
mod rng;
mod window;

#[cfg(test)]
mod tests;

pub use backoff::{backoff_delay, FailureCause, FailureGate, FailureGates, BACKOFF_BASE_MINUTES, BACKOFF_MAX_MINUTES};
pub use cadence::{cadence_rule, CadenceRule};
pub use chronotype::{peak_window, posting_chance};
pub use clock::{Clock, FixedClock, SystemClock};
pub use planner::{due_agents, evaluate, latest_activity, AgentSlot, Decision, ScheduleContext};
pub use preview::{simulate_week, WeekPreview, DAYS_PER_WEEK};
pub use queue::{EnqueueOutcome, GenerationQueue, DEFAULT_MAX_IN_FLIGHT, DEFAULT_QUEUE_CAPACITY};
pub use rng::roll_percent;
pub use window::{HourWindow, HOURS_PER_DAY};

/// How often the live heartbeat asks the scheduling core which agents are due (the
/// forge's week preview replays ticks at the same spacing).
pub const HEARTBEAT_INTERVAL: std::time::Duration = std::time::Duration::from_secs(60);

/// Upper bound of every percent-like value (spontaneity, chance, rolls are below it).
pub const PERCENT_MAX: u8 = 100;
