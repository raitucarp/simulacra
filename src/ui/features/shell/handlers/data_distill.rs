//! N15: distilling a thread into the vault from a post's menu: the document is written at
//! once, a locked post and one kept from some readers showing only a placeholder.

use super::super::view::AppView;
use crate::domain::post::distill::DistillWords;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{format_date, tr, DistillEvent};
use gpui_kit::*;
use rust_i18n::t;

fn words(count: usize) -> DistillWords {
    DistillWords {
        origin: t!("posts.distill.origin", count = count).to_string(),
        summary_heading: tr("posts.distill.summary"),
        open_post: tr("posts.distill.open_post"),
        retracted: tr("posts.distill.retracted"),
        corrected: tr("posts.distill.corrected"),
        untitled: tr("posts.distill.untitled"),
        locked: tr("posts.distill.locked"),
        restricted: tr("posts.distill.restricted"),
    }
}

impl AppView {
    /// Routes one distill event.
    pub(in crate::ui::features::shell) fn on_distill_event(&mut self, event: DistillEvent, cx: &mut Context<Self>) {
        match event {
            DistillEvent::Thread(id) => self.distill(&id, None, cx),
        }
    }

    fn distill(&mut self, id: &PostId, summary: Option<&str>, cx: &mut Context<Self>) {
        let text = match self.state.distill_thread(id, words, summary, format_date) {
            Ok(distilled) => t!("posts.distill.done", title = distilled.title).to_string(),
            Err(err) => publish_error_message(&err),
        };
        self.notify_soon(text, cx);
    }
}
