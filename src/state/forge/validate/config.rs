//! Rules over `AgentConfig`: languages, schedule window, custom template, biography.

use super::{IssueKind, Severity, ValidationIssue, ARG_MAX, ARG_VALUE};
use crate::domain::agent_config::TEMPLATE_PLACEHOLDERS;
use crate::domain::{Profile, PromptAssemblyStrategy};
use crate::state::forge::fields::{CountField, HourField, TextField, MAX_HOUR};
use std::ops::RangeInclusive;

/// Ages outside this range are allowed but flagged.
const USUAL_AGES: RangeInclusive<u16> = 13..=110;
const PLACEHOLDER_OPEN: &str = "{{";
const PLACEHOLDER_CLOSE: &str = "}}";

fn text_issue(field: TextField, severity: Severity, kind: IssueKind) -> ValidationIssue {
    ValidationIssue::new(field.step(), field.path(), severity, kind)
}

pub(super) fn check(profile: &Profile, issues: &mut Vec<ValidationIssue>) {
    let cfg = &profile.config;
    if cfg.primary_language.trim().is_empty() {
        issues.push(text_issue(TextField::PrimaryLanguage, Severity::Error, IssueKind::LanguageRequired));
    }
    if cfg.bio.trim().is_empty() {
        issues.push(text_issue(TextField::Bio, Severity::Warning, IssueKind::BioEmpty));
    }
    if !USUAL_AGES.contains(&cfg.age) {
        let age = CountField::Age;
        let issue = ValidationIssue::new(age.step(), age.path(), Severity::Warning, IssueKind::AgeUnusual);
        issues.push(issue.with_arg(ARG_VALUE, cfg.age));
    }
    check_hours(profile, issues);
    if cfg.assembly_strategy == PromptAssemblyStrategy::CustomTemplate {
        check_template(&cfg.prompt_template, issues);
    }
}

fn check_hours(profile: &Profile, issues: &mut Vec<ValidationIssue>) {
    let cfg = &profile.config;
    for field in HourField::ALL {
        if *field.get(profile) > MAX_HOUR {
            let issue = ValidationIssue::new(field.step(), field.path(), Severity::Error, IssueKind::HoursOutOfRange);
            issues.push(issue.with_arg(ARG_MAX, MAX_HOUR));
        }
    }
    if cfg.is_active && cfg.active_hours_start == cfg.active_hours_end {
        let field = HourField::ActiveHoursEnd;
        let issue = ValidationIssue::new(field.step(), field.path(), Severity::Warning, IssueKind::HoursAroundTheClock);
        issues.push(issue);
    }
}

/// Every `{{…}}` token in `template`, plus whether an opening `{{` was left unclosed.
pub(crate) fn placeholders(template: &str) -> (Vec<&str>, bool) {
    let mut found = Vec::new();
    let mut rest = template;
    while let Some(open) = rest.find(PLACEHOLDER_OPEN) {
        let after = &rest[open..];
        let Some(close) = after.find(PLACEHOLDER_CLOSE) else { return (found, true) };
        let end = close + PLACEHOLDER_CLOSE.len();
        found.push(&after[..end]);
        rest = &after[end..];
    }
    (found, false)
}

fn check_template(template: &str, issues: &mut Vec<ValidationIssue>) {
    let field = TextField::PromptTemplate;
    if template.trim().is_empty() {
        issues.push(text_issue(field, Severity::Error, IssueKind::TemplateEmpty));
        return;
    }
    let (tokens, unclosed) = placeholders(template);
    for token in tokens.into_iter().filter(|token| !TEMPLATE_PLACEHOLDERS.contains(token)) {
        let issue = text_issue(field, Severity::Error, IssueKind::TemplateUnknownPlaceholder);
        issues.push(issue.with_arg(ARG_VALUE, token));
    }
    if unclosed {
        issues.push(text_issue(field, Severity::Warning, IssueKind::TemplateUnclosed));
    }
}
