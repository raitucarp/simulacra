//! The review keys: 1–4 grade an item (or pick an option; after a checked map they go on
//! with the grade it earned), Space and Enter reveal (or check a drawn map, or move on after
//! a multiple-choice answer).

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
            (Screen::Learn, _) => self.grade_walk(grade, cx),
            _ => {}
        }
    }

    /// Reveal (session) or next question (multiple choice).
    pub(in crate::ui::features::review) fn press_reveal(&mut self, cx: &mut Context<Self>) {
        match (self.screen, &self.quiz) {
            // A checked map moves on with its button or a grade key, never with the key that checked
            // it (one press can arrive twice, as a key and as a character).
            (Screen::Session, _) if self.drill.is_some() && self.session.stage == Stage::Revealed => {}
            (Screen::Session, _) => self.reveal(cx),
            (Screen::Quiz, Some(QuizRunState::Choice(_))) => self.next_choice(cx),
            (Screen::Learn, _) => self.reveal_walk(cx),
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
