//! N13: "Still true?" from a claim's strip: confirm it (it comes back later), or ask the AI
//! whether it still holds. Revising and retracting go through Edit (N13) and N14.

use crate::domain::PostId;

/// One interaction with a "Still true?" check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckEvent {
    /// The claim still holds.
    Confirm(PostId),
    /// Asks the AI engine whether it still holds.
    Recheck(PostId),
}
