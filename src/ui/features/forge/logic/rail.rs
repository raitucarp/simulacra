//! What the left step rail shows for each step: number, current marker, ✓ or issue count.

use crate::state::forge::{AgentDraft, ForgeStep, Severity, ValidationIssue};

/// The badge of one rail entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RailStatus {
    /// Not finished yet (or never opened).
    Pending,
    /// Visited and free of errors.
    Complete,
    /// Visited, with this many errors to fix.
    Issues(usize),
}

/// One entry of the step rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RailItem {
    pub step: ForgeStep,
    /// 1-based position among the steps this kind walks.
    pub number: usize,
    pub is_current: bool,
    pub status: RailStatus,
}

/// Number of blocking issues reported for `step`.
fn errors_on(issues: &[ValidationIssue], step: ForgeStep) -> usize {
    issues.iter().filter(|issue| issue.step == step && issue.severity == Severity::Error).count()
}

/// Rail entries for every step the draft's kind walks, in order. Errors only show on steps
/// the user has already opened, so the rail never nags about pages they have not seen.
pub fn rail_items(draft: &AgentDraft, issues: &[ValidationIssue]) -> Vec<RailItem> {
    draft
        .steps()
        .iter()
        .enumerate()
        .map(|(at, &step)| {
            let progress = draft.progress(step);
            let errors = errors_on(issues, step);
            let status = if progress.visited && errors > 0 {
                RailStatus::Issues(errors)
            } else if progress.complete {
                RailStatus::Complete
            } else {
                RailStatus::Pending
            };
            RailItem { step, number: at + 1, is_current: draft.step() == step, status }
        })
        .collect()
}
