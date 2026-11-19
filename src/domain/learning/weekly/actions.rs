//! Next actions the weekly review suggests: review what comes due, then one action per weak
//! concept chosen by its weakest dimension (write about it, find a source, or test it with
//! the tutor), varied so one kind of action is not repeated. Ticked actions become tasks of
//! a goal.

use crate::domain::learning::gaps::{GapDimension, GapEntry};

/// Weak concepts that get an action.
pub const WEAK_CONCEPTS: usize = 3;
/// Actions suggested at most.
pub const MAX_ACTIONS: usize = 5;

/// One suggested action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextAction {
    /// Review the items coming due this week.
    ReviewDue { count: usize },
    /// Talk the concept through with the tutor.
    AskTutor { concept: String },
    WritePost { concept: String },
    /// Bring a source on it into the vault.
    ImportSource { concept: String },
}

impl NextAction {
    /// Stable key (locale keys).
    pub const fn key(&self) -> &'static str {
        match self {
            Self::ReviewDue { .. } => "review_due",
            Self::AskTutor { .. } => "ask_tutor",
            Self::WritePost { .. } => "write_post",
            Self::ImportSource { .. } => "import_source",
        }
    }

    /// The concept the action is about, if any.
    pub fn concept(&self) -> Option<&str> {
        match self {
            Self::ReviewDue { .. } => None,
            Self::AskTutor { concept } | Self::WritePost { concept } | Self::ImportSource { concept } => Some(concept),
        }
    }

    /// The action that strengthens `entry`'s weakest dimension.
    pub fn for_gap(entry: &GapEntry) -> Self {
        Self::for_dimension(entry.weakest(), entry.concept.clone())
    }

    /// The action that strengthens `dimension` of `concept`.
    pub fn for_dimension(dimension: GapDimension, concept: String) -> Self {
        match dimension {
            GapDimension::Writing | GapDimension::Connections => Self::WritePost { concept },
            GapDimension::Sources => Self::ImportSource { concept },
            GapDimension::Retention | GapDimension::Mastery | GapDimension::Recency => Self::AskTutor { concept },
        }
    }
}

/// The action for `entry`: its weakest dimension's, unless that kind of action is already
/// suggested, then the next weakest dimension's (so the week mixes writing, sources and the
/// tutor rather than repeating one).
fn varied_action(entry: &GapEntry, taken: &[NextAction]) -> NextAction {
    let mut dimensions: Vec<GapDimension> = GapDimension::ALL.to_vec();
    dimensions.sort_by(|a, b| entry.score(*a).total_cmp(&entry.score(*b)));
    dimensions
        .into_iter()
        .map(|dimension| NextAction::for_dimension(dimension, entry.concept.clone()))
        .find(|action| !taken.iter().any(|t| t.key() == action.key()))
        .unwrap_or_else(|| NextAction::for_gap(entry))
}

/// The week's suggestions: due reviews first, then an action per weak concept.
pub fn suggest_actions(due_week: usize, weakest: &[GapEntry]) -> Vec<NextAction> {
    let review = (due_week > 0).then_some(NextAction::ReviewDue { count: due_week });
    let actions = weakest.iter().take(WEAK_CONCEPTS).fold(review.into_iter().collect::<Vec<_>>(), |mut taken, entry| {
        let action = varied_action(entry, &taken);
        taken.push(action);
        taken
    });
    actions.into_iter().take(MAX_ACTIONS).collect()
}
