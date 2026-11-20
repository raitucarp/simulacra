//! State rules per entry kind.
//!
//! - **Note**: no states at all.
//! - **Event**: open, done, scheduled (moved to another date) or dropped. Events are not
//!   migrated or delegated: an event happens at its time or is moved or cancelled.
//! - **Task**: every state.
//!
//! Moves between allowed states:
//! - An open entry may move to any other allowed state.
//! - A pending entry (migrated, scheduled, delegated) may be reopened, finished or dropped.
//! - A settled entry (done, dropped) may only be reopened.
//! - Staying in the same state is not a transition.

use super::kinds::{EntryState, EntrySymbol};

const TASK_STATES: [EntryState; 6] = EntryState::ALL;
const EVENT_STATES: [EntryState; 4] =
    [EntryState::Open, EntryState::Done, EntryState::Scheduled, EntryState::Dropped];

/// The states `entry` may carry, `Open` first.
pub fn allowed_states(entry: EntrySymbol) -> &'static [EntryState] {
    match entry {
        EntrySymbol::Task => &TASK_STATES,
        EntrySymbol::Event => &EVENT_STATES,
        EntrySymbol::Note => &[],
    }
}

/// Whether `entry` may carry `state`.
pub fn state_allowed(entry: EntrySymbol, state: EntryState) -> bool {
    allowed_states(entry).contains(&state)
}

/// The state a new entry starts in: open for tasks and events, none for notes.
pub fn initial_state(entry: EntrySymbol) -> Option<EntryState> {
    allowed_states(entry).first().copied()
}

/// Whether an `entry` in state `from` may move to `to` (see the module rules).
pub fn can_transition(entry: EntrySymbol, from: EntryState, to: EntryState) -> bool {
    if from == to || !state_allowed(entry, from) || !state_allowed(entry, to) {
        return false;
    }
    match from {
        EntryState::Open => true,
        EntryState::Done | EntryState::Dropped => to == EntryState::Open,
        EntryState::Migrated | EntryState::Scheduled | EntryState::Delegated => {
            matches!(to, EntryState::Open | EntryState::Done | EntryState::Dropped)
        }
    }
}
