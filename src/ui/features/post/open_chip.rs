//! N18: a question says whether it is still open or answered, under its author. A locked post
//! never shows one (it cannot be made a question; a forged one shows only its placeholder).

use crate::domain::post::open_question::{question_state, QuestionState};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// Where `post` stands as a question, as its chip shows it (`None`: no question, or locked).
pub(crate) fn shown_question(post: &Post) -> Option<QuestionState> {
    question_state(post).filter(|_| !post.is_locked())
}

/// The chip; `None` for every other post.
pub(super) fn render_open_chip(post: &Post, theme: &ThemeTokens) -> Option<Div> {
    let (icon, colour, key) = match shown_question(post)? {
        QuestionState::Open => (LucideIcon::HelpCircle, theme.accent_brand, "posts.open_question.chip_open"),
        QuestionState::Answered => (LucideIcon::CircleCheck, theme.accent_success, "posts.open_question.chip_answered"),
    };
    let chip = div().flex().items_center().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(colour).child(Icon::new(icon).xsmall().text_color(colour)).child(t!(key).to_string());
    Some(chip)
}
