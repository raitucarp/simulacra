//! N4: answering the closing card of a finite feed: the answer goes out as the user's
//! one-line post and is kept as today's answer, so the card shows it until tomorrow.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::domain::finite_feed::one_line_answer;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::feed::EndCard;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

impl AppView {
    /// Posts the closing card's answer as a one-line post (nothing when it is blank).
    pub(in crate::ui::features::shell) fn answer_feed_day(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(line) = one_line_answer(&self.feed.learned_input.read(cx).value()) else { return };
        match self.state.submit_post(&line) {
            Ok(id) => {
                self.feed.day.answer = Some(id);
                self.save_feed_day();
                self.feed.learned_input.update(cx, |input, cx| input.set_value("", window, cx));
                self.notify_data(t!("feed.end.posted").to_string(), window, cx);
            }
            Err(err) => self.notify_data(publish_error_message(&err), window, cx),
        }
        cx.notify();
    }

    /// What the closing card shows and does.
    pub(in crate::ui::features::shell) fn end_card(&self, bind: &Binder) -> EndCard {
        let answer = self.feed.day.answer.as_ref().and_then(|id| self.state.get_post(id)).map(|post| (post.id.clone(), crate::domain::views::rapid_log::one_line(post)));
        EndCard { input: self.feed.learned_input.clone(), answer, on_answer: Rc::new(bind.act(Self::answer_feed_day)), on_open: Rc::new(bind.with(Self::select_post)) }
    }
}
