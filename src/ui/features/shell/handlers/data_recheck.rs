//! N13: "Ask the AI" on a claim due for a check: after the spend check, the claim (never a
//! locked one) goes through the post actions runner; the answer shows on the claim's strip.

use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::post::still_true::checkable;
use crate::domain::post::still_true::prompt::{clean_verdict, recheck_prompt, RECHECK_TOKENS};
use crate::domain::spend::SpendEstimate;
use crate::domain::PostId;
use crate::ui::features::data::tr;
use gpui_kit::*;
use rust_i18n::t;

/// How the prompt says when the claim was written.
const DATE_FORMAT: &str = "%Y-%m-%d";

impl AppView {
    /// Asks the AI engine whether the user's claim `id` still holds.
    pub(in crate::ui::features::shell) fn recheck_claim(&mut self, id: PostId, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(&id).filter(|post| checkable(post)) else { return }; // X2: never a locked post
        if self.data.ideas.rechecking.is_some() {
            return;
        }
        let (claim, written) = (post.content.clone(), crate::time::local(post.created_at).strftime(DATE_FORMAT).to_string());
        let resume = id.clone();
        if self.confirm_spend(SpendEstimate::one(), Box::new(move |this, _, cx| this.recheck_claim(resume, cx)), cx) {
            return;
        }
        let language = self.state.preferences().general.language.english_label();
        match self.thinking_job(recheck_prompt(&claim, &written, language), RECHECK_TOKENS, CancelToken::new()) {
            Ok(job) => {
                self.data.ideas.ticket += 1;
                self.data.ideas.rechecking = Some(id);
                self.spawn_thinking_job(job, self.data.ideas.ticket, Self::finish_recheck, cx);
            }
            Err(failure) => self.notify_soon(t!("posts.still_true.recheck_failed", reason = failure_message(&failure)).to_string(), cx),
        }
        cx.notify();
    }

    fn finish_recheck(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let Some(id) = self.data.ideas.rechecking.take().filter(|_| ticket == self.data.ideas.ticket) else { return };
        match outcome.map(|answer| clean_verdict(&answer)) {
            Ok(Some(verdict)) => {
                self.data.ideas.rechecks.retain(|(post, _)| post != &id);
                self.data.ideas.rechecks.push((id, t!("posts.still_true.verdict", answer = verdict).to_string()));
            }
            Ok(None) => self.notify_soon(t!("posts.still_true.recheck_failed", reason = tr("posts.distill.empty")).to_string(), cx),
            Err(failure) => self.notify_soon(t!("posts.still_true.recheck_failed", reason = failure_message(&failure)).to_string(), cx),
        }
        cx.notify();
    }
}
