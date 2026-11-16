//! Display-name rules.

use super::{IssueKind, Severity, ValidationIssue, ARG_MAX, ARG_MIN};
use crate::state::forge::fields::TextField;

/// Shortest accepted display name, in characters (after trimming).
pub const NAME_MIN_CHARS: usize = 2;
/// Longest accepted display name, in characters (after trimming).
pub const NAME_MAX_CHARS: usize = 48;

/// A name is junk when it has no letters or repeats one letter (`"aaaa"`, `"x x x"`).
fn is_junk(name: &str) -> bool {
    let mut letters = name.chars().filter(|c| c.is_alphabetic()).flat_map(char::to_lowercase);
    let Some(first) = letters.next() else { return true };
    let mut repeated = true;
    let mut count = 1;
    for letter in letters {
        repeated &= letter == first;
        count += 1;
    }
    repeated && count >= NAME_MIN_CHARS
}

pub(super) fn check(raw: &str, issues: &mut Vec<ValidationIssue>) {
    let field = TextField::Name;
    let name = raw.trim();
    let length = name.chars().count();
    let kind = if name.is_empty() {
        IssueKind::NameRequired
    } else if length < NAME_MIN_CHARS {
        IssueKind::NameTooShort
    } else if length > NAME_MAX_CHARS {
        IssueKind::NameTooLong
    } else if is_junk(name) {
        IssueKind::NameJunk
    } else {
        return;
    };
    let issue = ValidationIssue::new(field.step(), field.path(), Severity::Error, kind);
    issues.push(issue.with_arg(ARG_MIN, NAME_MIN_CHARS).with_arg(ARG_MAX, NAME_MAX_CHARS));
}
