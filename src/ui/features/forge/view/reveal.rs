//! When a validation issue shows. A field that is merely still empty is not flagged while
//! the user is filling in its step for the first time: red on a field nobody has had the
//! chance to fill reads as a scolding. It is flagged once they leave the step (Next, Back
//! or the rail), and the Review step lists every issue. Wrong values show at once.

use super::ForgeView;
use crate::state::forge::ValidationIssue;

impl ForgeView {
    /// Whether `issue` shows on its field and in the step rail now.
    pub(crate) fn reveals(&self, issue: &ValidationIssue) -> bool {
        !issue.kind.is_blank() || self.left_steps.contains(&issue.step)
    }

    /// The issues shown on fields and in the step rail.
    pub(crate) fn shown_issues(&self) -> Vec<ValidationIssue> {
        self.issues.iter().filter(|issue| self.reveals(issue)).cloned().collect()
    }
}
