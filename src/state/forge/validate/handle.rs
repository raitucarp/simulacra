//! `@handle` rules (one optional leading `@`, like the editor's handle field) and suggestions.

use super::{IssueKind, Severity, ValidationIssue, ARG_MAX, ARG_MIN, ARG_VALUE};
use crate::domain::profile::HANDLE_PREFIX;
use crate::domain::{Profile, ProfileId};
use crate::state::forge::fields::TextField;

/// Shortest accepted handle body (without the `@`).
pub const HANDLE_MIN_CHARS: usize = 2;
/// Longest accepted handle body (without the `@`).
pub const HANDLE_MAX_CHARS: usize = 30;
/// The only non-alphanumeric character allowed in a handle.
const HANDLE_JOINER: char = '_';
/// Name punctuation that becomes a joiner in a suggested handle (besides whitespace).
const SLUG_SEPARATORS: [char; 3] = ['_', '-', '.'];
/// Handle stem used when a name yields no usable characters.
const FALLBACK_STEM: &str = "thinker";
/// First numeric suffix tried when a suggested handle is taken (`name_2`).
const FIRST_SUFFIX: usize = 2;

/// The handle without one optional leading `@`, trimmed.
fn body(raw: &str) -> &str {
    let trimmed = raw.trim();
    trimmed.strip_prefix(HANDLE_PREFIX).unwrap_or(trimmed)
}

fn is_handle_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == HANDLE_JOINER
}

/// Whether another profile (not `own`) already uses `handle_body`, ignoring case.
fn is_taken(handle_body: &str, own: &ProfileId, existing: &[Profile]) -> bool {
    let wanted = handle_body.to_lowercase();
    existing
        .iter()
        .any(|p| &p.id != own && body(&p.handle).to_lowercase() == wanted)
}

pub(super) fn check(profile: &Profile, existing: &[Profile], issues: &mut Vec<ValidationIssue>) {
    let field = TextField::Handle;
    let handle = body(&profile.handle);
    let length = handle.chars().count();
    let kind = if handle.is_empty() {
        IssueKind::HandleRequired
    } else if !handle.chars().all(is_handle_char) {
        IssueKind::HandleFormat
    } else if !(HANDLE_MIN_CHARS..=HANDLE_MAX_CHARS).contains(&length) {
        IssueKind::HandleLength
    } else if is_taken(handle, &profile.id, existing) {
        IssueKind::HandleTaken
    } else {
        return;
    };
    let issue = ValidationIssue::new(field.step(), field.path(), Severity::Error, kind);
    let issue = issue.with_arg(ARG_MIN, HANDLE_MIN_CHARS).with_arg(ARG_MAX, HANDLE_MAX_CHARS);
    issues.push(issue.with_arg(ARG_VALUE, format!("{HANDLE_PREFIX}{handle}")));
}

/// Lower-case `a_b` slug of `name` limited to handle characters and length.
fn slug(name: &str) -> String {
    let mut out = String::new();
    for c in name.trim().chars().flat_map(char::to_lowercase) {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if (c.is_whitespace() || SLUG_SEPARATORS.contains(&c)) && !out.ends_with(HANDLE_JOINER) {
            out.push(HANDLE_JOINER);
        }
    }
    let trimmed: String = out.trim_matches(HANDLE_JOINER).chars().take(HANDLE_MAX_CHARS).collect();
    let trimmed = trimmed.trim_end_matches(HANDLE_JOINER).to_string();
    if trimmed.chars().count() < HANDLE_MIN_CHARS {
        FALLBACK_STEM.to_string()
    } else {
        trimmed
    }
}

/// A valid `@handle` derived from `name` that no profile in `existing` uses (other than `own`).
pub fn suggest_handle(name: &str, own: &ProfileId, existing: &[Profile]) -> String {
    let stem = slug(name);
    let mut candidate = stem.clone();
    let mut suffix = FIRST_SUFFIX;
    while is_taken(&candidate, own, existing) {
        let tail = format!("{HANDLE_JOINER}{suffix}");
        let room = HANDLE_MAX_CHARS.saturating_sub(tail.len());
        candidate = format!("{}{tail}", stem.chars().take(room).collect::<String>());
        suffix += 1;
    }
    format!("{HANDLE_PREFIX}{candidate}")
}
