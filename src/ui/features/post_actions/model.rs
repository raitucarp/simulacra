//! What the actions sheet shows, and the events it sends to its host.

use crate::domain::post_actions::{ActionKind, ActionOutput, ActionRequest, AudiencePreset};
use crate::domain::PostId;
use crate::i18n::Language;
use crate::state::semantic::{HitTarget, SemanticHit};

/// Where an action run stands.
#[derive(Debug, Clone, PartialEq)]
pub enum ActionStatus {
    /// No action picked yet (opened from the palette).
    Picking,
    /// The action needs a choice first (a concept, a typed audience or language).
    NeedsChoice,
    Running,
    Done(ActionOutput),
    /// The localized reason.
    Failed(String),
}

/// "Search concept" extras: vault full-text hits and the thoughts nearest in meaning.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConceptHits {
    pub vault: Vec<SemanticHit>,
    pub nearby: Vec<SemanticHit>,
}

/// The open actions sheet.
#[derive(Debug, Clone, PartialEq)]
pub struct ActionSheet {
    pub post_id: PostId,
    /// Short excerpt of the thought.
    pub source: String,
    pub request: ActionRequest,
    /// Concept candidates of the thought.
    pub concepts: Vec<String>,
    pub status: ActionStatus,
    pub hits: Option<ConceptHits>,
}

impl ActionSheet {
    /// The kind on screen, unless the user is still picking one.
    pub fn active_kind(&self) -> Option<ActionKind> {
        (self.status != ActionStatus::Picking).then_some(self.request.kind)
    }

    /// The finished answer, if any.
    pub fn output(&self) -> Option<&ActionOutput> {
        match &self.status {
            ActionStatus::Done(output) => Some(output),
            _ => None,
        }
    }
}

/// A choice clicked in the sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionChoice {
    Audience(AudiencePreset),
    Language(Language),
    Concept(String),
}

/// Everything the sheet asks its host to do.
#[derive(Debug, Clone, PartialEq)]
pub enum SheetEvent {
    Pick(ActionKind),
    Choose(ActionChoice),
    /// Use the typed audience, language or concept.
    ApplyTyped,
    Rerun,
    Cancel,
    Copy,
    PostAsReply,
    SaveToVault,
    OpenHit(HitTarget),
    Close,
}
