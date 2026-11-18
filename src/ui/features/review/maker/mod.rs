//! The card maker: a sheet that turns a post, a vault document or a typed sentence into
//! flashcards. Pick a sentence, choose basic, cloze (click words to hide them) or concept,
//! add tags, or ask the AI for a draft; every card cites its source.

mod actions;
pub mod fields;
mod footer;
mod render;
mod sections;

pub use fields::{MakerKind, MakerRequest, MakerSentence};

use crate::domain::review::{CardDraft, DraftKind};
use crate::domain::review::card::DraftProblem;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{AppContext, Context, Entity, EventEmitter, FocusHandle, Subscription, Window};
use rust_i18n::t;
use std::collections::BTreeSet;

/// Where the AI draft stands.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AiStatus {
    #[default]
    Idle,
    Waiting,
    /// How many cards it added to the list.
    Added(usize),
    Failed(String),
}

/// What the maker asks the shell to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MakerEvent {
    /// Save these cards (the maker closes once the shell confirms).
    Save(Vec<CardDraft>),
    /// Draft cards about this text with the AI.
    Draft(String),
}

/// The card maker sheet.
pub struct FlashcardMaker {
    pub(super) theme: ThemeTokens,
    pub(super) request: Option<MakerRequest>,
    /// The sentence the fields started from.
    pub(super) chosen: Option<usize>,
    pub(super) kind: MakerKind,
    pub(super) front: Entity<InputState>,
    pub(super) back: Entity<InputState>,
    pub(super) tags: Entity<InputState>,
    /// Word indices of the cloze field turned into deletions.
    pub(super) picked: BTreeSet<usize>,
    /// Cards added so far, saved together.
    pub(super) pending: Vec<CardDraft>,
    pub(super) problem: Option<DraftProblem>,
    pub(super) ai: AiStatus,
    pub(super) focus: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<MakerEvent> for FlashcardMaker {}

impl FlashcardMaker {
    pub fn new(theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = |key: &str, window: &mut Window, cx: &mut Context<Self>| {
            let placeholder = t!(key).to_string();
            cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
        };
        let (front, back, tags) = (input("review.maker.front_hint", window, cx), input("review.maker.back_hint", window, cx), input("review.maker.tags_hint", window, cx));
        let watch = cx.subscribe(&front, |this: &mut Self, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                this.picked.clear();
                this.problem = None;
                cx.notify();
            }
        });
        Self {
            theme,
            request: None,
            chosen: None,
            kind: MakerKind::default(),
            front,
            back,
            tags,
            picked: BTreeSet::new(),
            pending: Vec::new(),
            problem: None,
            ai: AiStatus::Idle,
            focus: cx.focus_handle(),
            _subscriptions: vec![watch],
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

    /// The fields' current draft.
    pub(super) fn current_draft(&self, cx: &Context<Self>) -> DraftKind {
        let (front, back) = (self.front.read(cx).value().to_string(), self.back.read(cx).value().to_string());
        fields::draft_of(self.kind, &front, &back, &self.picked)
    }
}
