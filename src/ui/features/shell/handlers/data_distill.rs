//! N15: distilling a thread into the vault from a post's menu. Plain distilling writes the
//! document at once; "with a summary" first asks the AI engine (after the spend check, through
//! the post actions runner) for a summary of the posts that may go to the vault, never a
//! locked one or one kept from some readers, then writes the document with it on top.

use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::post::distill::prompt::{clean_summary, summary_prompt, SUMMARY_TOKENS};
use crate::domain::post::distill::DistillWords;
use crate::domain::spend::SpendEstimate;
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
            DistillEvent::Summarized(id) => self.ask_distill_summary(id, cx),
        }
    }

    fn distill(&mut self, id: &PostId, summary: Option<&str>, cx: &mut Context<Self>) {
        let text = match self.state.distill_thread(id, words, summary, format_date) {
            Ok(distilled) => t!("posts.distill.done", title = distilled.title).to_string(),
            Err(err) => publish_error_message(&err),
        };
        self.notify_soon(text, cx);
    }

    /// The posts of `id`'s thread that may go to the vault, as (author, text).
    fn distillable_lines(&self, id: &PostId) -> Vec<(String, String)> {
        let posts = self.state.thread_of(id);
        posts.into_iter().filter(|post| !post.is_locked() && post.extras.audience.is_none()).map(|post| (self.state.author_of(post).name.clone(), post.distill_text())).collect()
    }

    fn ask_distill_summary(&mut self, id: PostId, cx: &mut Context<Self>) {
        if self.data.ideas.distilling.is_some() {
            return;
        }
        let lines = self.distillable_lines(&id);
        let resume = id.clone();
        if lines.is_empty() || self.confirm_spend(SpendEstimate::one(), Box::new(move |this, _, cx| this.ask_distill_summary(resume, cx)), cx) {
            return;
        }
        let language = self.state.preferences().general.language.english_label();
        match self.thinking_job(summary_prompt(&lines, language), SUMMARY_TOKENS, CancelToken::new()) {
            Ok(job) => {
                self.data.ideas.ticket += 1;
                self.data.ideas.distilling = Some(id);
                self.spawn_thinking_job(job, self.data.ideas.ticket, Self::finish_distill_summary, cx);
            }
            Err(failure) => self.notify_soon(t!("posts.distill.no_summary", reason = failure_message(&failure)).to_string(), cx),
        }
    }

    fn finish_distill_summary(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let Some(id) = self.data.ideas.distilling.take().filter(|_| ticket == self.data.ideas.ticket) else { return };
        match outcome.map(|answer| clean_summary(&answer)) {
            Ok(Some(summary)) => self.distill(&id, Some(&summary), cx),
            Ok(None) => self.notify_soon(t!("posts.distill.no_summary", reason = tr("posts.distill.empty")).to_string(), cx),
            Err(failure) => self.notify_soon(t!("posts.distill.no_summary", reason = failure_message(&failure)).to_string(), cx),
        }
        cx.notify();
    }
}
