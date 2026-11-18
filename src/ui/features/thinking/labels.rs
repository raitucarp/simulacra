//! Localized labels and theme colours of the thinking tools' vocabularies: node roles, edge
//! kinds, standings, credence outcomes, tension verdicts and atomic-note findings.

use crate::domain::thinking::argument::{EdgeKind, NodeRole, Standing};
use crate::domain::thinking::atomic::Finding;
use crate::domain::thinking::credence::Outcome;
use crate::domain::thinking::tensions::Verdict;
use crate::theme::ThemeTokens;
use gpui_kit::Rgba;
use rust_i18n::t;

/// The translation of `prefix.key`.
pub fn keyed(prefix: &str, key: &str) -> String {
    let full = format!("{prefix}.{key}");
    t!(full.as_str()).to_string()
}

/// The translation of `key`.
pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

pub fn role_label(role: NodeRole) -> String {
    keyed("thinking.role", role.key())
}

pub fn edge_label(kind: EdgeKind) -> String {
    keyed("thinking.edge", kind.key())
}

pub fn standing_label(standing: Standing) -> String {
    keyed("thinking.standing", standing.key())
}

pub fn outcome_label(outcome: Outcome) -> String {
    keyed("thinking.outcome", outcome.key())
}

pub fn verdict_label(verdict: Verdict) -> String {
    keyed("thinking.verdict", verdict.key())
}

/// A finding's explanation, with its numbers and words filled in.
pub fn finding_text(finding: &Finding) -> String {
    let key = format!("thinking.atomic.finding.{}", finding.key());
    match finding {
        Finding::MultipleClaims { count } => t!(key.as_str(), count = count).to_string(),
        Finding::TooShort { chars } | Finding::TooLong { chars } => t!(key.as_str(), chars = chars).to_string(),
        Finding::MultipleTopics { topics } => t!(key.as_str(), topics = topics.join(", ")).to_string(),
        Finding::DanglingPronoun { word } => t!(key.as_str(), word = word).to_string(),
        Finding::NoLink | Finding::NoSource => t!(key.as_str()).to_string(),
    }
}

/// The colour that marks a role (chips, swatches, the list).
pub fn role_colour(role: NodeRole, theme: &ThemeTokens) -> Rgba {
    match role {
        NodeRole::Claim => theme.accent_brand,
        NodeRole::Ground => theme.accent_user,
        NodeRole::Warrant => theme.accent_alt,
        NodeRole::Backing => theme.info,
        NodeRole::Qualifier => theme.accent_dialectic,
        NodeRole::Rebuttal => theme.warning,
    }
}

pub fn edge_colour(kind: EdgeKind, theme: &ThemeTokens) -> Rgba {
    match kind {
        EdgeKind::Support => theme.accent_success,
        EdgeKind::Attack => theme.accent_danger,
    }
}

pub fn standing_colour(standing: Standing, theme: &ThemeTokens) -> Rgba {
    match standing {
        Standing::Accepted => theme.accent_success,
        Standing::Rejected => theme.accent_danger,
        Standing::Undecided => theme.warning,
    }
}

pub fn outcome_colour(outcome: Option<Outcome>, theme: &ThemeTokens) -> Rgba {
    match outcome {
        None => theme.accent_brand,
        Some(Outcome::True) => theme.accent_success,
        Some(Outcome::False) => theme.accent_danger,
        Some(Outcome::Ambiguous) => theme.text_muted,
    }
}

pub fn verdict_colour(verdict: Option<Verdict>, theme: &ThemeTokens) -> Rgba {
    match verdict {
        Some(Verdict::Contradicts) => theme.accent_danger,
        Some(Verdict::Compatible) => theme.accent_success,
        Some(Verdict::Unrelated) | None => theme.text_muted,
    }
}
