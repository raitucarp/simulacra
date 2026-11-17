//! Engine-step UI state (open section, the editable sample answer, the two probes) and the
//! edits its controls make: every control changes a copy of the draft's engine, applied as
//! one undo step (typing in one section merges until the field loses focus).

use super::{ForgeHandle, ForgeView};
use crate::agent::credentials::CredentialStore;
use crate::domain::EngineConfig;
use crate::state::forge::{DraftEdit, EngineSection, Severity};
use crate::ui::features::forge::widgets::ValueCallback;
use gpui_kit::*;
use std::path::PathBuf;

/// A background check ("Test call", "Test image") and where it stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Probe<T> {
    Idle,
    Waiting,
    Done(T),
    /// Display-safe message (keys redacted).
    Failed(String),
}

impl<T> Default for Probe<T> {
    fn default() -> Self {
        Self::Idle
    }
}

/// What a test call brought back: the raw reply of a custom request, and the text read from
/// it (or why none could be read).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallReply {
    pub raw: Option<String>,
    pub text: Result<String, String>,
}

/// State of the Engine step.
#[derive(Debug, Clone, Default)]
pub struct EngineUi {
    pub section: EngineSection,
    /// The hand-edited sample answer of the preview (`None` = generated from the schema).
    pub sample: Option<String>,
    pub call: Probe<CallReply>,
    pub image: Probe<PathBuf>,
}

impl ForgeView {
    /// The draft's engine.
    pub(crate) fn engine(&self) -> &EngineConfig {
        &self.draft.profile().config.engine
    }

    /// Applies `change` to a copy of the engine as one edit (`typing` merges keystrokes).
    pub(crate) fn edit_engine(&mut self, typing: Option<EngineSection>, change: impl FnOnce(&mut EngineConfig), cx: &mut Context<Self>) {
        let mut engine = self.engine().clone();
        change(&mut engine);
        self.apply_edit(DraftEdit::Engine(Box::new(engine), typing), cx);
    }

    /// Opens `section` of the Engine step.
    pub(crate) fn open_engine_section(&mut self, section: EngineSection, cx: &mut Context<Self>) {
        self.engine.section = section;
        cx.notify();
    }

    /// Whether `section` has an issue of `severity`.
    pub(crate) fn section_has(&self, section: EngineSection, severity: Severity) -> bool {
        self.issues.iter().any(|issue| issue.field == section.issue_field() && issue.severity == severity)
    }

    /// Replaces the credentials after Settings › AI changed them, and re-checks the draft.
    pub fn set_credentials(&mut self, credentials: &CredentialStore, cx: &mut Context<Self>) {
        if &self.context.credentials != credentials {
            self.context.credentials = credentials.clone();
            self.revalidate();
            cx.notify();
        }
    }
}

impl ForgeHandle {
    /// A callback editing the engine with `set` (a card pick: one undo step each).
    pub fn engine<A: 'static>(&self, set: impl Fn(&mut EngineConfig, A) + 'static) -> ValueCallback<A> {
        self.with(move |this, arg, _, cx| this.edit_engine(None, |engine| set(engine, arg), cx))
    }

    /// A callback editing the engine with `set` while typing in `section`.
    pub fn engine_typing<A: 'static>(&self, section: EngineSection, set: impl Fn(&mut EngineConfig, A) + 'static) -> ValueCallback<A> {
        self.with(move |this, arg, _, cx| this.edit_engine(Some(section), |engine| set(engine, arg), cx))
    }
}
