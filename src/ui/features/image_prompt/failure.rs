//! Runner and engine failures as the sheet's [`IdeasFailure`].

use super::model::IdeasFailure;
use crate::agent::post_actions::ActionFailure;
use crate::domain::post_actions::ActionFailureKind;

/// Words in an image engine error that mean it could not be reached.
const NETWORK_MARKERS: [&str; 2] = ["request error", "timed out"];

/// The sheet's failure for a runner failure.
pub fn from_action(failure: ActionFailure) -> Option<IdeasFailure> {
    match failure.kind {
        ActionFailureKind::Cancelled => None,
        ActionFailureKind::NoKey => Some(IdeasFailure::NoTextKey),
        ActionFailureKind::Budget => Some(IdeasFailure::Budget),
        ActionFailureKind::Network | ActionFailureKind::Timeout => Some(IdeasFailure::Offline),
        _ => Some(IdeasFailure::Failed(failure.detail)),
    }
}

/// The sheet's failure for an image engine error.
pub fn from_image_error(error: String) -> IdeasFailure {
    let lower = error.to_lowercase();
    if NETWORK_MARKERS.iter().any(|marker| lower.contains(marker)) { IdeasFailure::Offline } else { IdeasFailure::Failed(error) }
}
