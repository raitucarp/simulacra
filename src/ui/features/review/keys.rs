//! The review session's keys: reveal (Space or Enter) and the four grades (1–4, which also
//! pick a multiple-choice option). Their bindings are rows of the central keymap
//! (`crate::ui::keymap`), active only while a card waits for one of them.

use gpui_kit::actions;

actions!(review, [GradeAgain, GradeHard, GradeGood, GradeEasy, RevealAnswer]);

/// Key context of the review card while it waits for a reveal, a grade or a choice (absent
/// while the learner types an answer, so digits reach the text field).
pub const REVIEW_KEY_CONTEXT: &str = "ReviewSession";
