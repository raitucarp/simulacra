//! Icons and localized labels of actions, groups, audiences and failures.

use crate::domain::post_actions::{ActionFailureKind, ActionGroup, ActionKind, AudiencePreset};
use crate::ui::icons::LucideIcon;
use rust_i18n::t;

const KIND_PREFIX: &str = "post_actions.kind";
const HINT_PREFIX: &str = "post_actions.hint";
const GROUP_PREFIX: &str = "post_actions.group";
const AUDIENCE_PREFIX: &str = "post_actions.audience";
const FAILURE_PREFIX: &str = "post_actions.failure";

/// The icon of an action.
pub fn kind_icon(kind: ActionKind) -> LucideIcon {
    match kind {
        ActionKind::SearchConcept => LucideIcon::Search,
        ActionKind::Explain => LucideIcon::GraduationCap,
        ActionKind::SummariseThread => LucideIcon::List,
        ActionKind::MapConcepts => LucideIcon::Network,
        ActionKind::Translate => LucideIcon::Languages,
        ActionKind::Steelman => LucideIcon::Shield,
        ActionKind::Strawman => LucideIcon::Scale,
        ActionKind::HiddenPremise => LucideIcon::Puzzle,
        ActionKind::Counterexample => LucideIcon::Swords,
        ActionKind::CognitiveBiases => LucideIcon::Brain,
        ActionKind::ClaimConfidence => LucideIcon::Gauge,
        ActionKind::Crux => LucideIcon::Target,
        ActionKind::Socratic => LucideIcon::HelpCircle,
        ActionKind::Analogies => LucideIcon::GitCompareArrows,
        ActionKind::FalsifiablePrediction => LucideIcon::FlaskConical,
        ActionKind::ContinueThought => LucideIcon::Feather,
    }
}

fn keyed(prefix: &str, key: &str) -> String {
    let full = format!("{prefix}.{key}");
    t!(full.as_str()).to_string()
}

/// The action's title ("Steelman").
pub fn kind_title(kind: ActionKind) -> String {
    keyed(KIND_PREFIX, kind.key())
}

/// One line on what the action does.
pub fn kind_hint(kind: ActionKind) -> String {
    keyed(HINT_PREFIX, kind.key())
}

pub fn group_title(group: ActionGroup) -> String {
    keyed(GROUP_PREFIX, group.key())
}

pub fn audience_label(audience: AudiencePreset) -> String {
    keyed(AUDIENCE_PREFIX, audience.key())
}

/// Why a run failed, for the user (`detail` is the redacted provider message).
pub fn failure_text(kind: ActionFailureKind, detail: &str) -> String {
    let key = format!("{FAILURE_PREFIX}.{}", kind.key());
    t!(key.as_str(), detail = detail).to_string()
}
