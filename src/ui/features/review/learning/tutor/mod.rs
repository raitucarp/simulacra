//! The tutor tool: start a dialogue (on a post or thread from the picker, a vault document
//! or a typed concept), the dialogue itself, and its summary with review items for the gaps.

mod dialogue;
pub(in crate::ui::features::review) mod ladder;
mod start;
mod summary;

use super::super::events::{PickerMode, ReviewEvent};
use super::super::panel::ReviewPanel;
use super::events::LearningEvent;
use crate::domain::learning::{SessionId, TutorSession, TutorSource};
use crate::ui::features::review::panel::PanelFields;
use gpui_kit::component::input::InputState;
use gpui_kit::{App, Context, Entity, Window};

/// The trimmed text of a one-line field, which is then emptied.
pub(in crate::ui::features::review) fn take_text(input: &Entity<InputState>, window: &mut Window, cx: &mut App) -> String {
    let text = PanelFields::text(input, cx);
    input.update(cx, |state, cx| state.set_value("", window, cx));
    text
}

impl ReviewPanel {
    pub(in crate::ui::features::review) fn emit_learning(&mut self, event: LearningEvent, cx: &mut Context<Self>) {
        cx.emit(ReviewEvent::Learning(event));
    }

    /// The session shown, if it still exists.
    pub(in crate::ui::features::review) fn open_session(&self) -> Option<&TutorSession> {
        self.learning.tutor_open.as_ref().and_then(|id| self.data.learning.records.session(id))
    }

    /// Enter in the concept field (or its button): a dialogue on the typed concept.
    pub fn start_tutor_on_concept(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = take_text(&self.learn_fields.concept, window, cx);
        if !name.is_empty() {
            self.start_tutor(TutorSource::Concept { name }, cx);
        }
    }

    /// Asks the shell to start a dialogue on `source`; it opens once the session exists.
    pub fn start_tutor(&mut self, source: TutorSource, cx: &mut Context<Self>) {
        self.learning.tool = super::LearnTool::Tutor;
        self.learning.tutor_open = None;
        self.show(super::super::panel::Screen::Learn, cx);
        self.emit_learning(LearningEvent::StartTutor(source), cx);
    }

    /// Shows session `id` (the shell calls this once a new session is stored).
    pub fn show_session(&mut self, id: SessionId, cx: &mut Context<Self>) {
        self.learning.tool = super::LearnTool::Tutor;
        self.learning.tutor_open = Some(id);
        self.learning.tutor_unticked.clear();
        self.learning.tutor_all_exchanges = false;
        self.show(super::super::panel::Screen::Learn, cx);
    }

    /// Sends the typed answer to the open question.
    pub fn send_answer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(id) = self.learning.tutor_open.clone() else { return };
        let text = self.learn_fields.answer.read(cx).value().trim().to_string();
        if text.is_empty() {
            return;
        }
        self.learn_fields.answer.update(cx, |state, cx| state.set_value("", window, cx));
        self.emit_learning(LearningEvent::AnswerTutor(id, text), cx);
    }

    /// Ticks or unticks summary question `index` for a review item.
    pub(in crate::ui::features::review) fn toggle_gap(&mut self, index: usize, cx: &mut Context<Self>) {
        let unticked = &mut self.learning.tutor_unticked;
        match unticked.iter().position(|i| *i == index) {
            Some(at) => {
                unticked.remove(at);
            }
            None => unticked.push(index),
        }
        cx.notify();
    }

    /// Makes review items from the ticked summary questions.
    pub(in crate::ui::features::review) fn create_gap_items(&mut self, cx: &mut Context<Self>) {
        let Some(session) = self.open_session() else { return };
        let count = session.summary.as_ref().map_or(0, |summary| summary.review.len());
        let chosen: Vec<usize> = (0..count).filter(|i| !self.learning.tutor_unticked.contains(i)).collect();
        if !chosen.is_empty() {
            let id = session.id.clone();
            self.emit_learning(LearningEvent::CreateGapItems(id, chosen), cx);
        }
    }

    /// Opens the post picker to choose what the tutor talks about.
    pub(in crate::ui::features::review) fn pick_tutor_post(&mut self, cx: &mut Context<Self>) {
        cx.emit(ReviewEvent::OpenPicker(PickerMode::Tutor));
    }
}
