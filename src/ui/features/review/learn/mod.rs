//! The learn sheet: a small sheet over the post (or vault document) being learnt. The post
//! itself is one face; the learner writes the other: a question the post answers ("Add
//! question"), or the answer the post prompts ("Add answer"). For a document, a sentence or
//! paragraph is picked first and several passages can be saved together. An AI suggestion
//! of the written face goes through the configured LLM client.

mod actions;
mod body;
mod footer;
mod render;
pub mod request;

pub use request::{Granularity, LearnRequest, LearnSource, Passage};

use crate::domain::review::item::DraftProblem;
use crate::domain::review::{AnchorSide, ItemDraft, ItemId};
use crate::theme::ThemeTokens;
use gpui_kit::component::input::InputState;
use gpui_kit::{AppContext, Context, Entity, EventEmitter, FocusHandle, Window};
use rust_i18n::t;

/// Where the AI suggestion stands.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AiStatus {
    #[default]
    Idle,
    Waiting,
    Suggested,
    Failed(String),
}

/// What the sheet asks the shell to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LearnEvent {
    /// Save these drafts (replacing `existing` when editing); the sheet closes.
    Save { drafts: Vec<ItemDraft>, existing: Option<ItemId> },
    /// Suggest the written face for this text.
    Suggest { text: String, side: AnchorSide },
}

/// The learn sheet.
pub struct LearnSheet {
    pub(super) theme: ThemeTokens,
    pub(super) request: Option<LearnRequest>,
    pub(super) side: AnchorSide,
    pub(super) granularity: Granularity,
    /// The passage the fields are about.
    pub(super) chosen: Option<Passage>,
    pub(super) written: Entity<InputState>,
    pub(super) tags: Entity<InputState>,
    /// Passages added so far, saved together.
    pub(super) pending: Vec<ItemDraft>,
    pub(super) problem: Option<DraftProblem>,
    pub(super) ai: AiStatus,
    pub(super) focus: FocusHandle,
}

impl EventEmitter<LearnEvent> for LearnSheet {}

impl LearnSheet {
    pub fn new(theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = |key: &str, window: &mut Window, cx: &mut Context<Self>| {
            let placeholder = t!(key).to_string();
            cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
        };
        Self {
            theme,
            request: None,
            side: AnchorSide::default(),
            granularity: Granularity::default(),
            chosen: None,
            written: input("review.learn.written_hint", window, cx),
            tags: input("review.learn.tags_hint", window, cx),
            pending: Vec::new(),
            problem: None,
            ai: AiStatus::Idle,
            focus: cx.focus_handle(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.request.is_some()
    }

    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }
}
