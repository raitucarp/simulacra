//! The Review page entity: which screen shows, the running session (with the drill of a
//! connection map), quiz, map editor or post picker, and their text fields. Changes to the
//! deck go out as [`ReviewEvent`]s; the shell applies them and hands back [`ReviewData`].

mod drill;
mod editor;
mod fields;
mod keys;
mod picker;
mod quiz;
mod session;

pub use drill::DrillRun;
pub use editor::MapEditor;
pub use fields::PanelFields;
pub use picker::Picker;
#[cfg(test)]
pub use picker::matches_query;

use super::data::ReviewData;
use super::events::ReviewEvent;
use crate::domain::review::quiz::{ChoiceRun, OrderPuzzle, RecallRun};
use crate::domain::review::Session;
use crate::theme::ThemeTokens;
use crate::ui::features::review::learning::{LearningFields, LearningUi};
use gpui_kit::{Context, EventEmitter, FocusHandle, Subscription, Window};

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
    Items,
    /// Posts of the stream to learn or to map.
    Picker,
    /// A connection map being made or edited.
    MapEditor,
    /// The learning tools: tutor, goals, gaps and palace (sb E2).
    Learn,
}

impl Screen {
    /// The tabs of the page header (the other screens are entered from them).
    pub const TABS: [Self; 4] = [Self::Today, Self::Learn, Self::Stats, Self::Items];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Today | Self::Session | Self::Summary | Self::Quiz => "today",
            Self::Stats => "stats",
            Self::Items | Self::Picker | Self::MapEditor => "items",
            Self::Learn => "learn",
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
    pub(super) drill: Option<DrillRun>,
    pub(super) quiz: Option<QuizRunState>,
    pub(super) editor: Option<MapEditor>,
    pub(super) picker: Option<Picker>,
    pub(super) focus: FocusHandle,
    pub(super) fields: PanelFields,
    pub(in crate::ui::features::review) learning: LearningUi,
    pub(in crate::ui::features::review) learn_fields: LearningFields,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<ReviewEvent> for ReviewPanel {}

impl ReviewPanel {
    pub fn new(data: ReviewData, theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (fields, mut subscriptions) = PanelFields::new(window, cx);
        let (learn_fields, learn_subscriptions) = LearningFields::new(window, cx);
        subscriptions.extend(learn_subscriptions);
        Self {
            theme,
            data,
            screen: Screen::Today,
            session: Session::default(),
            drill: None,
            quiz: None,
            editor: None,
            picker: None,
            focus: cx.focus_handle(),
            fields,
            learning: LearningUi::new(),
            learn_fields,
            _subscriptions: subscriptions,
        }
    }

    /// Swaps in fresh deck data; a different network ends any session, quiz or editor.
    pub fn set_data(&mut self, data: ReviewData, cx: &mut Context<Self>) {
        if data.network_id != self.data.network_id {
            (self.session, self.drill, self.quiz, self.editor, self.picker) = (Session::default(), None, None, None, None);
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

    /// Whether an item is waiting for a reveal, a grade or a choice (the review keys are live).
    pub(super) fn keys_active(&self) -> bool {
        match (self.screen, &self.quiz) {
            (Screen::Session, _) => !self.session.is_finished(),
            (Screen::Quiz, Some(QuizRunState::Choice(run))) => !run.is_finished(),
            (Screen::Quiz, Some(QuizRunState::Recall(run))) => run.checked.is_some(),
            (Screen::Learn, _) => self.learning.walk().is_some_and(|walk| !walk.is_finished()),
            _ => false,
        }
    }
}
