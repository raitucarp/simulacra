//! The flashcard session: start with today's queue, reveal, grade, finish with a summary.

use super::{ReviewPanel, Screen};
use crate::domain::review::queue::build_queue;
use crate::domain::review::schedule::preview;
use crate::domain::review::{Answer, Card, Grade, ReviewMode, Session, Stage};
use crate::ui::features::review::events::ReviewEvent;
use gpui_kit::{Context, Window};

impl ReviewPanel {
    /// Starts a session over today's queue and takes the keyboard.
    pub fn start_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let data = &self.data;
        let queue = build_queue(&data.cards, &data.log, data.today, data.limits);
        if queue.is_empty() {
            return;
        }
        self.session = Session::new(queue);
        self.screen = Screen::Session;
        self.focus.focus(window, cx);
        cx.notify();
    }

    /// The card being shown.
    pub(in crate::ui::features::review) fn session_card(&self) -> Option<&Card> {
        self.session.current().and_then(|id| self.data.card(id))
    }

    /// The four intervals of the current card, for its grade buttons.
    pub(in crate::ui::features::review) fn session_intervals(&self) -> Option<[u32; 4]> {
        self.session_card().map(|card| preview(&card.schedule, self.data.today, &self.data.policy))
    }

    pub(in crate::ui::features::review) fn reveal(&mut self, cx: &mut Context<Self>) {
        if self.session.stage == Stage::Prompt && !self.session.is_finished() {
            self.session = std::mem::take(&mut self.session).reveal();
            cx.notify();
        }
    }

    /// Grades the revealed card; a card missing from the deck (deleted meanwhile) is skipped.
    pub(in crate::ui::features::review) fn grade_session(&mut self, grade: Grade, cx: &mut Context<Self>) {
        if self.session.stage != Stage::Revealed {
            return;
        }
        if let Some(card_id) = self.session.current().cloned() {
            let answer = Answer { card_id, grade, mode: ReviewMode::Flashcard, confidence: None, correct: None };
            cx.emit(ReviewEvent::Answered(answer));
        }
        let data = &self.data;
        let next = std::mem::take(&mut self.session).grade(grade);
        let queue = next.queue.iter().filter(|id| data.card(id).is_some()).cloned().collect();
        self.session = Session { queue, ..next };
        if self.session.is_finished() {
            self.screen = Screen::Summary;
        }
        cx.notify();
    }

    /// Ends the session early and shows what was done.
    pub fn end_session(&mut self, cx: &mut Context<Self>) {
        self.screen = if self.session.answers.is_empty() { Screen::Today } else { Screen::Summary };
        self.session = Session { queue: Vec::new(), ..std::mem::take(&mut self.session) };
        cx.notify();
    }
}
