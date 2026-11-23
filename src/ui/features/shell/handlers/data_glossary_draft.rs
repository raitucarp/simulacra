//! N17: "Draft with AI" in the glossary sheet: after the spend check, the term and its post
//! (never a locked one) go through the post actions runner; the cleaned answer fills the
//! definition field, where the user can still edit it before saving.

use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::post::glossary::clean_term;
use crate::domain::post::glossary::prompt::{can_draft, clean_draft, definition_prompt, DEFINITION_TOKENS};
use crate::domain::spend::SpendEstimate;
use crate::ui::features::data::tr;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    fn glossary_problem(&mut self, text: String) {
        if let Some(dialog) = self.data.ideas.thoughts.glossary.as_mut() {
            dialog.problem = Some(text);
            dialog.drafting = false;
        }
    }

    /// Asks the AI engine to draft the definition of the sheet's term.
    pub(in crate::ui::features::shell) fn draft_definition(&mut self, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.thoughts.glossary.as_ref().filter(|dialog| !dialog.drafting) else { return };
        let Some(post) = self.state.get_post(&dialog.post_id).filter(|post| can_draft(post)).cloned() else { return }; // X2: never a locked post
        let Some(term) = clean_term(dialog.term.read(cx).value().as_ref()) else { return self.glossary_problem(tr("posts.glossary.term_first")) };
        if self.confirm_spend(SpendEstimate::one(), Box::new(|this, _, cx| this.draft_definition(cx)), cx) {
            return;
        }
        let language = self.state.preferences().general.language.english_label();
        match self.thinking_job(definition_prompt(&term, &post, language), DEFINITION_TOKENS, CancelToken::new()) {
            Ok(job) => {
                self.data.ideas.ticket += 1;
                if let Some(dialog) = self.data.ideas.thoughts.glossary.as_mut() {
                    (dialog.drafting, dialog.problem) = (true, None);
                }
                self.spawn_thinking_job(job, self.data.ideas.ticket, Self::finish_draft, cx);
            }
            Err(failure) => self.glossary_problem(t!("posts.glossary.draft_failed", reason = failure_message(&failure)).to_string()),
        }
        cx.notify();
    }

    fn finish_draft(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.thoughts.glossary.as_mut().filter(|dialog| dialog.drafting && ticket == self.data.ideas.ticket) else { return };
        dialog.drafting = false;
        match outcome.map(|answer| clean_draft(&answer)) {
            Ok(Some(text)) => {
                let (handle, field) = (self.spend.window, dialog.text.clone());
                cx.defer(move |cx| {
                    let _ = handle.update(cx, |_, window, cx| field.update(cx, |field, cx| field.set_value(text, window, cx)));
                });
            }
            Ok(None) => self.glossary_problem(t!("posts.glossary.draft_failed", reason = tr("posts.distill.empty")).to_string()),
            Err(failure) => self.glossary_problem(t!("posts.glossary.draft_failed", reason = failure_message(&failure)).to_string()),
        }
        cx.notify();
    }
}
