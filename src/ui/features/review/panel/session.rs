//! The review session: start with today's queue, reveal (or draw and check a connection
//! map), grade, finish with a summary.

use super::{DrillRun, ReviewPanel, Screen};
use crate::domain::review::queue::build_queue;
use crate::domain::review::schedule::preview;
use crate::domain::review::{Answer, Grade, ReviewItem, ReviewMode, Session, Stage};
use crate::ui::features::review::events::ReviewEvent;
use gpui_kit::{Context, Window};

impl ReviewPanel {
    /// Starts a session over today's queue and takes the keyboard.
    pub fn start_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let data = &self.data;
        let queue = build_queue(&data.items, &data.log, data.today, data.limits);
        if queue.is_empty() {
            return;
        }
        self.session = Session::new(queue);
        self.screen = Screen::Session;
        self.prepare_drill();
        self.focus.focus(window, cx);
        cx.notify();
    }

    /// The item being shown.
    pub(in crate::ui::features::review) fn session_item(&self) -> Option<&ReviewItem> {
        self.session.current().and_then(|id| self.data.item(id))
    }

    /// The four intervals of the current item, for its grade buttons.
    pub(in crate::ui::features::review) fn session_intervals(&self) -> Option<[u32; 4]> {
        self.session_item().map(|item| preview(&item.schedule, self.data.today, &self.data.policy))
    }

    /// A fresh drill when the current item is a connection map (none otherwise).
    fn prepare_drill(&mut self) {
        let map_item = self.session_item().filter(|item| item.subject.map().is_some()).cloned();
        self.drill = map_item.as_ref().map(DrillRun::start);
    }

    /// Shows the answer, or checks the drawing of a map.
    pub(in crate::ui::features::review) fn reveal(&mut self, cx: &mut Context<Self>) {
        if self.session.stage != Stage::Prompt || self.session.is_finished() {
            return;
        }
        if self.drill.is_some() {
            self.check_drill(cx);
        }
        self.session = std::mem::take(&mut self.session).reveal();
        cx.notify();
    }

    /// Grades the revealed item (a map takes the grade its drawing earned); an item missing
    /// from the deck (deleted meanwhile) is skipped.
    pub(in crate::ui::features::review) fn grade_session(&mut self, chosen: Grade, cx: &mut Context<Self>) {
        if self.session.stage != Stage::Revealed {
            return;
        }
        let earned = self.drill.as_ref().and_then(|run| run.grade);
        let (grade, mode) = earned.map_or((chosen, ReviewMode::Review), |grade| (grade, ReviewMode::Connections));
        if let Some(item_id) = self.session.current().cloned() {
            cx.emit(ReviewEvent::Answered(Answer { item_id, grade, mode, confidence: None, correct: earned.map(Grade::is_recall) }));
        }
        let data = &self.data;
        let next = std::mem::take(&mut self.session).grade(grade);
        let queue = next.queue.iter().filter(|id| data.item(id).is_some()).cloned().collect();
        self.session = Session { queue, ..next };
        self.prepare_drill();
        if self.session.is_finished() {
            self.screen = Screen::Summary;
        }
        cx.notify();
    }

    /// Ends the session early and shows what was done.
    pub fn end_session(&mut self, cx: &mut Context<Self>) {
        self.screen = if self.session.answers.is_empty() { Screen::Today } else { Screen::Summary };
        self.session = Session { queue: Vec::new(), ..std::mem::take(&mut self.session) };
        self.drill = None;
        cx.notify();
    }
}
