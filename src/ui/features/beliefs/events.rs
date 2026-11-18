//! What the Beliefs tab asks the shell to do.

use crate::domain::beliefs::{BeliefAction, BeliefSource};
use crate::domain::ProfileId;

/// A request from the Beliefs tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeliefsEvent {
    /// Pin, edit or forget one belief.
    Act(BeliefAction),
    /// Switch the agent's automatic updates on or off.
    SetUpdates(ProfileId, bool),
    /// Let the agent read the thread open in the inspector and update its beliefs.
    ReadOpenThread(ProfileId),
    /// Open where a belief came from.
    OpenSource(BeliefSource),
}
