//! The review keys: 1–4 grade an item (or pick an option), Space and Enter reveal (check a
//! drawn map, then go on with the grade it earned; or move on after a multiple-choice
//! answer).

use super::{QuizRunState, ReviewPanel, Screen};
use crate::domain::review::{Grade, Stage};
use crate::ui::features::review::keys::{GradeAgain, GradeEasy, GradeGood, GradeHard, RevealAnswer};
use gpui_kit::{Context, Window};

impl ReviewPanel {
    /// Grade (session, recall) or option (multiple choice) number `index` (zero-based).
    pub(in crate::ui::features::review) fn press_number(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(grade) = Grade::from_index(index) else { return };
        match (self.screen, &self.quiz) {
            (Screen::Session, _) => self.grade_session(grade, cx),
            (Screen::Quiz, Some(QuizRunState::Choice(_))) => self.pick_choice(index, cx),
            (Screen::Quiz, Some(QuizRunState::Recall(_))) => self.grade_recall(grade, window, cx),
            _ => {}
        }
    }

    /// Reveal (session) or next question (multiple choice).
    pub(in crate::ui::features::review) fn press_reveal(&mut self, cx: &mut Context<Self>) {
        match (self.screen, &self.quiz) {
            (Screen::Session, _) if self.session.stage == Stage::Revealed && self.drill.is_some() => self.grade_session(Grade::Good, cx),
            (Screen::Session, _) => self.reveal(cx),
            (Screen::Quiz, Some(QuizRunState::Choice(_))) => self.next_choice(cx),
            _ => {}
        }
    }

    pub(in crate::ui::features::review) fn on_again(&mut self, _: &GradeAgain, window: &mut Window, cx: &mut Context<Self>) {
        self.press_number(Grade::Again.index(), window, cx);
    }

    pub(in crate::ui::features::review) fn on_hard(&mut self, _: &GradeHard, window: &mut Window, cx: &mut Context<Self>) {
        self.press_number(Grade::Hard.index(), window, cx);
    }

    pub(in crate::ui::features::review) fn on_good(&mut self, _: &GradeGood, window: &mut Window, cx: &mut Context<Self>) {
        self.press_number(Grade::Good.index(), window, cx);
    }

    pub(in crate::ui::features::review) fn on_easy(&mut self, _: &GradeEasy, window: &mut Window, cx: &mut Context<Self>) {
        self.press_number(Grade::Easy.index(), window, cx);
    }

    pub(in crate::ui::features::review) fn on_reveal(&mut self, _: &RevealAnswer, _: &mut Window, cx: &mut Context<Self>) {
        self.press_reveal(cx);
    }
}
