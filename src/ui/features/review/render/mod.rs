//! The Review page: a header with its tabs above the current screen, in a centred column
//! (a post face, a connection map canvas and the picker are drawn here too).
//! The review keys are bound only while a card waits for them (see `keys_active`).

mod choice;
pub(in crate::ui::features::review) mod common;
mod drill;
mod empty;
pub mod face;
pub(in crate::ui::features::review) mod grades;
mod header;
mod items;
pub mod map;
mod map_editor;
mod map_lists;
mod order;
mod overview;
mod picker;
mod quiz_frame;
mod recall;
mod result;
mod session;
mod stats;
mod summary;

pub use empty::{REVIEW_DONE_ART, REVIEW_EMPTY_ART};

use super::keys::REVIEW_KEY_CONTEXT;
use super::panel::{ReviewPanel, Screen};
use crate::ui::responsive::LayoutClass;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const ROOT_ID: &str = "review-root";
const SCROLL_ID: &str = "review-scroll";

impl Render for ReviewPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.clone();
        let class = LayoutClass::of_window(window, &theme);
        let body = self.render_screen(class, cx);
        let column = div()
            .w_full()
            .max_w(theme.review_page_max_width)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.render_header(cx))
            .child(body);
        div()
            .id(ROOT_ID)
            .size_full()
            .track_focus(&self.focus)
            .when(self.keys_active(), |root| root.key_context(REVIEW_KEY_CONTEXT))
            .on_action(cx.listener(Self::on_again))
            .on_action(cx.listener(Self::on_hard))
            .on_action(cx.listener(Self::on_good))
            .on_action(cx.listener(Self::on_easy))
            .on_action(cx.listener(Self::on_reveal))
            .bg(theme.bg_app)
            .font_family(theme.font_family_ui.clone())
            .child(
                div()
                    .id(SCROLL_ID)
                    .size_full()
                    .overflow_y_scroll()
                    .flex()
                    .justify_center()
                    .items_start()
                    .px(if class.is_compact() { theme.panel_padding_compact } else { theme.panel_padding })
                    .py(theme.panel_padding)
                    .child(column),
            )
    }
}

impl ReviewPanel {
    fn render_screen(&self, class: LayoutClass, cx: &mut Context<Self>) -> AnyElement {
        match self.screen {
            Screen::Today => self.render_overview(cx).into_any_element(),
            Screen::Session => self.render_session(cx).into_any_element(),
            Screen::Summary => self.render_summary(cx).into_any_element(),
            Screen::Quiz => self.render_quiz(cx),
            Screen::Stats => self.render_stats(class).into_any_element(),
            Screen::Items => self.render_items(cx).into_any_element(),
            Screen::Picker => self.render_picker(cx).into_any_element(),
            Screen::MapEditor => self.render_map_editor(cx).into_any_element(),
            Screen::Learn => self.render_learning(class, cx),
        }
    }

    /// The running quiz's screen.
    fn render_quiz(&self, cx: &mut Context<Self>) -> AnyElement {
        use super::panel::QuizRunState;
        match &self.quiz {
            Some(QuizRunState::Choice(run)) => self.render_choice(run, cx).into_any_element(),
            Some(QuizRunState::Recall(run)) => self.render_recall(run, cx).into_any_element(),
            Some(QuizRunState::Order { puzzle, solved, rounds }) => self.render_order(puzzle, (*solved, *rounds), cx).into_any_element(),
            Some(QuizRunState::Unavailable(reason)) => self.render_unavailable(reason, cx).into_any_element(),
            None => self.render_overview(cx).into_any_element(),
        }
    }
}
