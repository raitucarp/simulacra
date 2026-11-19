//! Wording of the Beliefs tab: credences, sources, revisions and times.

use crate::domain::beliefs::{BeliefSource, Credence, Revision, RevisionCause};
use jiff::Timestamp;
use rust_i18n::t;

/// How revisions show their time.
const STAMP_FORMAT: &str = "%Y-%m-%d %H:%M";

pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

/// A credence as `0.72`.
pub fn credence_text(credence: Credence) -> String {
    format!("{:.2}", credence.unit())
}

pub fn stamp(at: Timestamp) -> String {
    crate::time::local(at).strftime(STAMP_FORMAT).to_string()
}

/// "From a debate: Cars", or "Set by you".
pub fn source_text(source: &BeliefSource) -> String {
    let kind = tr(&format!("beliefs.origin.{}", source.origin.key()));
    if source.label.trim().is_empty() {
        kind
    } else {
        format!("{kind}: {}", source.label.trim())
    }
}

/// "0.70 → 0.40", "new at 0.70" or "forgotten (was 0.70)".
pub fn change_text(revision: &Revision) -> String {
    match (revision.old, revision.new) {
        (Some(old), Some(new)) => format!("{} → {}", credence_text(old), credence_text(new)),
        (None, Some(new)) => t!("beliefs.change.new", credence = credence_text(new)).to_string(),
        (Some(old), None) => t!("beliefs.change.gone", credence = credence_text(old)).to_string(),
        (None, None) => String::new(),
    }
}

/// Why it changed: the agent's reason, or the cause when the reason is not the agent's words.
pub fn reason_text(revision: &Revision) -> String {
    match (revision.cause, revision.reason.trim()) {
        (RevisionCause::Evidence, reason) if !reason.is_empty() => reason.to_string(),
        (cause, _) => tr(&format!("beliefs.cause.{}", cause.key())),
    }
}
