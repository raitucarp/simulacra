//! The quizzes: the shell prepares the content, the page runs it and reports each answer.

use super::{QuizRunState, ReviewPanel, Screen};
use crate::domain::review::quiz::{ChoiceRun, QuizKind, RecallRun};
use crate::domain::review::{Answer, Confidence, Grade};
use crate::ui::features::review::events::{QuizContent, ReviewEvent};
use gpui_kit::{Context, Window};

type Step = (QuizRunState, Option<Answer>);

impl ReviewPanel {
    /// Asks the shell for a quiz of `kind`.
    pub(in crate::ui::features::review) fn request_quiz(&mut self, kind: QuizKind, cx: &mut Context<Self>) {
        cx.emit(ReviewEvent::RequestQuiz(kind));
    }

    /// Starts the quiz the shell prepared.
    pub fn begin_quiz(&mut self, content: QuizContent, window: &mut Window, cx: &mut Context<Self>) {
        let (solved, rounds) = match &self.quiz {
            Some(QuizRunState::Order { solved, rounds, .. }) if matches!(content, QuizContent::Order(_)) => (*solved, *rounds),
            _ => (0, 0),
        };
        self.quiz = Some(match content {
            QuizContent::Choice(questions) => QuizRunState::Choice(ChoiceRun::new(questions)),
            QuizContent::Recall(items) => QuizRunState::Recall(RecallRun::new(items)),
            QuizContent::Order(puzzle) => QuizRunState::Order { puzzle, solved, rounds },
            QuizContent::Unavailable(reason) => QuizRunState::Unavailable(reason),
        });
        self.screen = Screen::Quiz;
        self.fields.answer.update(cx, |input, cx| input.set_value("", window, cx));
        match self.quiz {
            Some(QuizRunState::Recall(_)) => self.fields.answer.update(cx, |input, cx| input.focus(window, cx)),
            _ => self.focus.focus(window, cx),
        }
        cx.notify();
    }

    /// Applies `step` to the running quiz and reports the answer it produced, if any.
    fn step_quiz(&mut self, step: impl FnOnce(QuizRunState) -> Step, cx: &mut Context<Self>) -> bool {
        let Some(quiz) = self.quiz.take() else { return false };
        let (quiz, answer) = step(quiz);
        self.quiz = Some(quiz);
        cx.notify();
        answer.map(|answer| cx.emit(ReviewEvent::Answered(answer))).is_some()
    }

    pub(in crate::ui::features::review) fn set_confidence(&mut self, confidence: Confidence, cx: &mut Context<Self>) {
        self.step_quiz(|quiz| match quiz {
            QuizRunState::Choice(run) => (QuizRunState::Choice(run.with_confidence(confidence)), None),
            other => (other, None),
        }, cx);
    }

    pub(in crate::ui::features::review) fn pick_choice(&mut self, option: usize, cx: &mut Context<Self>) {
        self.step_quiz(|quiz| match quiz {
            QuizRunState::Choice(run) => {
                let (run, answer) = run.pick(option);
                (QuizRunState::Choice(run), answer)
            }
            other => (other, None),
        }, cx);
    }

    pub(in crate::ui::features::review) fn next_choice(&mut self, cx: &mut Context<Self>) {
        self.step_quiz(|quiz| match quiz {
            QuizRunState::Choice(run) => (QuizRunState::Choice(run.next()), None),
            other => (other, None),
        }, cx);
    }

    /// Compares the typed answer with the current card's (Enter in the field).
    pub(in crate::ui::features::review) fn check_recall(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let typed = self.fields.answer.read(cx).value().to_string();
        let Some(QuizRunState::Recall(run)) = &self.quiz else { return };
        let expected = run.current().and_then(|id| self.data.item(id)).map(|item| self.data.answer_text(item)).unwrap_or_default();
        self.step_quiz(|quiz| match quiz {
            QuizRunState::Recall(run) => (QuizRunState::Recall(run.check(&typed, &expected)), None),
            other => (other, None),
        }, cx);
        self.focus.focus(window, cx);
    }

    pub(in crate::ui::features::review) fn grade_recall(&mut self, grade: Grade, window: &mut Window, cx: &mut Context<Self>) {
        let answered = self.step_quiz(|quiz| match quiz {
            QuizRunState::Recall(run) => {
                let (run, answer) = run.grade(grade);
                (QuizRunState::Recall(run), answer)
            }
            other => (other, None),
        }, cx);
        if answered {
            self.fields.answer.update(cx, |input, cx| {
                input.set_value("", window, cx);
                input.focus(window, cx);
            });
        }
    }

    /// Moves the piece at `slot` up (`up`) or down in the ordering puzzle.
    pub(in crate::ui::features::review) fn move_piece(&mut self, slot: usize, up: bool, cx: &mut Context<Self>) {
        self.step_quiz(|quiz| match quiz {
            QuizRunState::Order { puzzle, solved, rounds } => {
                let puzzle = if up { puzzle.move_up(slot) } else { puzzle.move_down(slot) };
                (QuizRunState::Order { puzzle, solved, rounds }, None)
            }
            other => (other, None),
        }, cx);
    }

    pub(in crate::ui::features::review) fn check_order(&mut self, cx: &mut Context<Self>) {
        self.step_quiz(|quiz| match quiz {
            QuizRunState::Order { puzzle, solved, rounds } if !puzzle.checked => {
                let puzzle = puzzle.check();
                let perfect = usize::from(puzzle.score().is_perfect());
                (QuizRunState::Order { puzzle, solved: solved + perfect, rounds: rounds + 1 }, None)
            }
            other => (other, None),
        }, cx);
    }
}
