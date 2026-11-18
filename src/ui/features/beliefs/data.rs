//! What the shell hands the Beliefs tab: the agent, its beliefs ranked by salience with each
//! one's credence series and recent revisions, the update switch and today's count, and
//! whether an update is running or a thread is open to read.

use crate::domain::beliefs::{Belief, BeliefSettings, Revision};
use crate::domain::ProfileId;

/// Revisions shown under an opened belief.
pub const HISTORY_ROWS: usize = 6;

/// One belief as the tab lists it.
#[derive(Debug, Clone, PartialEq)]
pub struct BeliefRow {
    pub belief: Belief,
    /// Credence after each revision (0–1), oldest first.
    pub series: Vec<f32>,
    /// Newest first, at most [`HISTORY_ROWS`].
    pub history: Vec<Revision>,
}

/// Everything the tab shows.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BeliefsData {
    pub agent: Option<ProfileId>,
    pub agent_name: String,
    pub rows: Vec<BeliefRow>,
    pub settings: BeliefSettings,
    /// Updates made today, and the daily cap.
    pub used_today: u32,
    pub daily_cap: u32,
    pub updating: bool,
    /// Whether a thread is open in the inspector (so "Read the open thread" can run).
    pub thread_open: bool,
    pub notice: Option<String>,
}
