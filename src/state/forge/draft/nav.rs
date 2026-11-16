//! Moving between wizard steps and tracking which steps were visited / completed.

use super::{AgentDraft, StepProgress};
use crate::state::forge::step::{steps_for, ForgeStep};
use crate::state::forge::validate::{ValidationIssue, Severity};

impl AgentDraft {
    /// The step currently shown.
    pub fn step(&self) -> ForgeStep {
        self.step
    }

    /// The steps this draft's kind walks through.
    pub fn steps(&self) -> &'static [ForgeStep] {
        steps_for(self.kind)
    }

    /// Visited / complete state of `step`.
    pub fn progress(&self, step: ForgeStep) -> StepProgress {
        self.progress.get(step.index()).copied().unwrap_or_default()
    }

    fn progress_mut(&mut self, step: ForgeStep) -> Option<&mut StepProgress> {
        self.progress.get_mut(step.index())
    }

    /// Opens `step` (marking it visited); ignored for steps this kind does not walk.
    pub fn go_to(&mut self, step: ForgeStep) -> bool {
        if !self.steps().contains(&step) {
            return false;
        }
        self.history.seal();
        self.step = step;
        if let Some(progress) = self.progress_mut(step) {
            progress.visited = true;
        }
        true
    }

    /// Marks the current step complete and opens the next one; `false` on the last step.
    pub fn advance(&mut self) -> bool {
        let current = self.step;
        let Some(next) = current.next(self.kind) else { return false };
        self.set_complete(current, true);
        self.go_to(next)
    }

    /// Opens the previous step; `false` on the first step.
    pub fn back(&mut self) -> bool {
        self.step.previous(self.kind).is_some_and(|previous| self.go_to(previous))
    }

    pub fn set_complete(&mut self, step: ForgeStep, complete: bool) {
        if let Some(progress) = self.progress_mut(step) {
            progress.complete = complete;
        }
    }

    /// Re-derives completion: a visited step is complete when it has no errors in `issues`.
    pub fn sync_completion(&mut self, issues: &[ValidationIssue]) {
        for &step in self.steps() {
            let has_error = issues
                .iter()
                .any(|issue| issue.step == step && issue.severity == Severity::Error);
            let visited = self.progress(step).visited;
            self.set_complete(step, visited && !has_error);
        }
    }

    /// Restores invariants after deserializing (progress length, step valid for the kind).
    pub(super) fn normalize_navigation(&mut self) {
        self.progress.resize(ForgeStep::ALL.len(), StepProgress::default());
        if !self.steps().contains(&self.step) {
            self.step = ForgeStep::Origin;
        }
    }
}
