//! Review-step validation: what blocks saving (errors) and what merely deserves a look (warnings).

mod config;
mod engine;
mod engine_parts;
mod handle;
mod kind;
mod name;

pub use handle::{suggest_handle, HANDLE_MAX_CHARS, HANDLE_MIN_CHARS};
pub use kind::IssueKind;
pub use name::{NAME_MAX_CHARS, NAME_MIN_CHARS};

use super::draft::AgentDraft;
use super::step::{steps_for, ForgeStep};
use crate::agent::credentials::CredentialStore;
use crate::domain::Profile;
use serde::{Deserialize, Serialize};

/// Interpolation argument holding a lower bound (`%{min}`).
pub const ARG_MIN: &str = "min";
/// Interpolation argument holding an upper bound (`%{max}`).
pub const ARG_MAX: &str = "max";
/// Interpolation argument holding the offending value (`%{value}`).
pub const ARG_VALUE: &str = "value";

/// How serious an issue is: errors block saving, warnings do not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Severity {
    Warning,
    Error,
}

/// One problem found in a draft, pinned to the step and field that can fix it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub step: ForgeStep,
    /// Serde path of the offending field (see `fields::mapped_fields`).
    pub field: &'static str,
    pub severity: Severity,
    pub kind: IssueKind,
    /// `(name, value)` pairs to interpolate into the message (see `ARG_*`).
    pub args: Vec<(&'static str, String)>,
}

impl ValidationIssue {
    pub(crate) fn new(step: ForgeStep, field: &'static str, severity: Severity, kind: IssueKind) -> Self {
        Self { step, field, severity, kind, args: Vec::new() }
    }

    #[must_use]
    pub(crate) fn with_arg(mut self, name: &'static str, value: impl ToString) -> Self {
        self.args.push((name, value.to_string()));
        self
    }

    pub fn message_key(&self) -> String {
        self.kind.message_key()
    }
}

/// Every issue in `draft`, checked against the other profiles in `existing`
/// (the draft's own id is ignored there), with no stored credentials.
pub fn validate(draft: &AgentDraft, existing: &[Profile]) -> Vec<ValidationIssue> {
    validate_with(draft, existing, &CredentialStore::default())
}

/// [`validate`] with the engine's credential references checked against `credentials`.
/// Only steps the draft's kind walks are reported.
pub fn validate_with(draft: &AgentDraft, existing: &[Profile], credentials: &CredentialStore) -> Vec<ValidationIssue> {
    let profile = draft.profile();
    let mut issues = Vec::new();
    name::check(&profile.name, &mut issues);
    handle::check(profile, existing, &mut issues);
    config::check(profile, &mut issues);
    engine::check(&profile.config.engine, credentials, &mut issues);
    let steps = steps_for(draft.kind());
    issues.retain(|issue| steps.contains(&issue.step));
    issues
}

/// Whether a draft with these issues may be saved (warnings are allowed).
pub fn can_save(issues: &[ValidationIssue]) -> bool {
    !issues.iter().any(|issue| issue.severity == Severity::Error)
}

/// The issues reported for `step`.
pub fn issues_for_step(issues: &[ValidationIssue], step: ForgeStep) -> Vec<&ValidationIssue> {
    issues.iter().filter(|issue| issue.step == step).collect()
}
