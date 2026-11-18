//! The Review page entity: which screen shows, the running session or quiz, and the typed
//! answer field. Changes to the deck go out as [`ReviewEvent`]s; the shell applies them and
//! hands back fresh [`ReviewData`].

mod keys;
mod quiz;
mod session;

use super::data::ReviewData;
use super::events::ReviewEvent;
use crate::domain::review::quiz::{ChoiceRun, OrderPuzzle, RecallRun};
use crate::domain::review::Session;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{AppContext, Context, Entity, EventEmitter, FocusHandle, Subscription, Window};
use rust_i18n::t;

/// The page's screens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    /// What is due, the start button and the quiz modes.
    #[default]
    Today,
    Session,
    /// After a session.
    Summary,
    Quiz,
    Stats,
    Cards,
}

impl Screen {
    /// The tabs of the page header (the other screens are entered from them).
    pub const TABS: [Self; 3] = [Self::Today, Self::Stats, Self::Cards];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Today | Self::Session | Self::Summary | Self::Quiz => "today",
            Self::Stats => "stats",
            Self::Cards => "cards",
        }
    }
}

/// The running quiz.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuizRunState {
    Choice(ChoiceRun),
    Recall(RecallRun),
    Order { puzzle: OrderPuzzle, solved: usize, rounds: usize },
    /// Not enough material: the locale key of the reason.
    Unavailable(&'static str),
}

/// The Review page.
pub struct ReviewPanel {
    pub(super) theme: ThemeTokens,
    pub(super) data: ReviewData,
    pub(super) screen: Screen,
    pub(super) session: Session,
    pub(super) quiz: Option<QuizRunState>,
    pub(super) focus: FocusHandle,
    pub(super) answer_input: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<ReviewEvent> for ReviewPanel {}

impl ReviewPanel {
    pub fn new(data: ReviewData, theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let placeholder = t!("review.recall.placeholder").to_string();
        let answer_input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let subscription = cx.subscribe_in(&answer_input, window, |this: &mut Self, _, event: &InputEvent, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                this.check_recall(window, cx);
            }
        });
        Self {
            theme,
            data,
            screen: Screen::Today,
            session: Session::default(),
            quiz: None,
            focus: cx.focus_handle(),
            answer_input,
            _subscriptions: vec![subscription],
        }
    }

    /// Swaps in fresh deck data; a different network ends any session or quiz.
    pub fn set_data(&mut self, data: ReviewData, cx: &mut Context<Self>) {
        if data.network_id != self.data.network_id {
            self.session = Session::default();
            self.quiz = None;
            self.screen = Screen::Today;
        }
        self.data = data;
        cx.notify();
    }

    /// Adopts a changed theme.
    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }

    /// Shows `screen` (leaving a session or quiz keeps its answers, already saved).
    pub fn show(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.screen = screen;
        cx.notify();
    }

    /// Whether a card is waiting for a reveal, a grade or a choice (the review keys are live).
    pub(super) fn keys_active(&self) -> bool {
        match (self.screen, &self.quiz) {
            (Screen::Session, _) => !self.session.is_finished(),
            (Screen::Quiz, Some(QuizRunState::Choice(run))) => !run.is_finished(),
            (Screen::Quiz, Some(QuizRunState::Recall(run))) => run.checked.is_some(),
            _ => false,
        }
    }
}
