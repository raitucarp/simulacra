//! Plain state records of the forge: what it was opened with, which screen shows, the start
//! screen, Mnemo, and the transient notice.

use crate::agent::credentials::CredentialStore;
use crate::agent::{LlmSettings, PromptContext};
use crate::domain::{ArchetypeClass, NetworkId, Palette, PaletteCategory, Profile};
use crate::state::forge::{AgentDraft, ForgeKind};

/// Everything the forge needs to know about the rest of the app, captured when it opens.
#[derive(Debug, Clone)]
pub struct ForgeContext {
    /// Every stored profile (for handle uniqueness and name suggestions).
    pub existing: Vec<Profile>,
    /// Network new drafts belong to.
    pub network_id: NetworkId,
    /// Provider settings for "describe them in words".
    pub llm: LlmSettings,
    /// Whether Mnemo starts hidden.
    pub guide_hidden: bool,
    /// The active network's name and grounding sources, for compiled-prompt previews.
    pub prompt: PromptContext,
    /// Stored keys, for the Engine step's pickers, readiness badges and probes.
    pub credentials: CredentialStore,
}

/// How the forge was opened.
#[derive(Debug, Clone)]
pub enum ForgeOpen {
    /// A new profile of this kind; `autosave` is an unfinished draft to offer first.
    New { kind: ForgeKind, autosave: Option<Box<AgentDraft>> },
    /// An existing profile, opened at Origin.
    Edit(Box<Profile>),
}

/// The page the forge shows.
#[derive(Debug, Clone)]
pub enum ForgeScreen {
    /// "Resume your unfinished agent?" with the stored draft.
    Resume(Box<AgentDraft>),
    /// "Choose your class".
    Start,
    /// The step-by-step creator.
    Steps,
    /// The new thinker joined the network.
    Celebrate,
}

/// Mnemo the guide owl.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GuideState {
    pub hidden: bool,
    /// True from entering a step until its first edit (Mnemo points at the page).
    pub fresh_step: bool,
    /// Waiting for the AI to read a description.
    pub thinking: bool,
}

/// Where "describe them in words" stands.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum DescribeStatus {
    #[default]
    Idle,
    Waiting,
    /// Nothing typed yet.
    Empty,
    /// No API key configured.
    NoKey,
    /// The request or the reply failed; the message is display-safe.
    Failed(String),
}

/// State of the "choose a palette" screen.
#[derive(Debug, Clone)]
pub struct StartState {
    pub kind: ForgeKind,
    /// Palette whose preview shows (hovered or clicked).
    pub preview: Option<&'static Palette>,
    /// Gallery shelf shown (`None`: every category).
    pub category: Option<PaletteCategory>,
    /// Gallery search text.
    pub search: String,
    /// The class of the last dice roll, while its result shows.
    pub rolled: Option<ArchetypeClass>,
    /// Whether the describe panel is open.
    pub describing: bool,
    pub describe_text: String,
    pub describe_status: DescribeStatus,
}

impl StartState {
    pub fn new(kind: ForgeKind) -> Self {
        Self {
            kind,
            preview: None,
            category: None,
            search: String::new(),
            rolled: None,
            describing: false,
            describe_text: String::new(),
            describe_status: DescribeStatus::Idle,
        }
    }
}

/// A short message shown above the options until dismissed or replaced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notice {
    pub text: String,
    /// Extra lines (e.g. skipped fields of a description).
    pub details: Vec<String>,
    pub is_error: bool,
    /// The draft the notice describes (e.g. right after a palette apply); once the draft
    /// differs (undo, redo, a later edit) the notice is stale and goes away.
    pub anchor: Option<Box<Profile>>,
}

impl Notice {
    pub fn info(text: impl Into<String>, details: Vec<String>) -> Self {
        Self { text: text.into(), details, is_error: false, anchor: None }
    }

    /// Ties the notice to `draft`: it only stays while the working copy still equals it.
    pub fn anchored(self, draft: &Profile) -> Self {
        Self { anchor: Some(Box::new(draft.clone())), ..self }
    }

    /// Whether an anchored notice no longer matches the working copy.
    pub fn is_stale_for(&self, draft: &Profile) -> bool {
        self.anchor.as_deref().is_some_and(|anchor| anchor != draft)
    }

    pub fn error(text: impl Into<String>) -> Self {
        Self { text: text.into(), details: Vec::new(), is_error: true, anchor: None }
    }
}
