//! The user's controls on the profile's Beliefs tab, pure: pin or unpin, edit the wording or
//! the credence, forget. Edits and forgetting are logged as revisions by the user; pinning
//! changes nothing the agent believes, so it is not logged.

use super::book::{BeliefBook, BeliefChange};
use super::matching::tidy;
use super::model::{BeliefId, BeliefSource, Credence, RevisionCause};
use super::update::{user_revision, MAX_PROPOSITION_CHARS};
use jiff::Timestamp;

/// What the user does to one belief.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeliefAction {
    TogglePin(BeliefId),
    /// New wording (blank keeps the old one) and credence.
    Edit { id: BeliefId, proposition: String, credence: Credence },
    Forget(BeliefId),
}

impl BeliefAction {
    pub fn id(&self) -> &BeliefId {
        match self {
            Self::TogglePin(id) | Self::Forget(id) => id,
            Self::Edit { id, .. } => id,
        }
    }
}

/// The change `action` makes in `book` at `now` (empty when the belief is gone or nothing
/// would change).
pub fn apply_user_action(book: &BeliefBook, action: &BeliefAction, now: Timestamp) -> BeliefChange {
    let Some(belief) = book.belief(action.id()).cloned() else { return BeliefChange::default() };
    match action {
        BeliefAction::TogglePin(_) => BeliefChange { upserted: vec![super::model::Belief { pinned: !belief.pinned, ..belief }], ..BeliefChange::default() },
        BeliefAction::Forget(_) => {
            let revision = user_revision(&belief, Some(belief.credence), None, RevisionCause::UserForget, now);
            BeliefChange { removed: vec![belief.id.clone()], revisions: vec![revision], ..BeliefChange::default() }
        }
        BeliefAction::Edit { proposition, credence, .. } => {
            let wording: String = tidy(proposition).chars().take(MAX_PROPOSITION_CHARS).collect();
            let wording = if wording.is_empty() { belief.proposition.clone() } else { wording };
            if wording == belief.proposition && *credence == belief.credence {
                return BeliefChange::default();
            }
            let edited = super::model::Belief { proposition: wording, credence: *credence, source: BeliefSource::user(), updated_at: now, ..belief.clone() };
            let revision = user_revision(&edited, Some(belief.credence), Some(*credence), RevisionCause::UserEdit, now);
            BeliefChange { upserted: vec![edited], revisions: vec![revision], ..BeliefChange::default() }
        }
    }
}
