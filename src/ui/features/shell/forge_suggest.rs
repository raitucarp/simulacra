//! Sending the forge's "suggest from context" requests (sb D1) like a post action: the shared
//! daily AI budget is checked and spent here, the request runs on the background executor
//! with the configured timeout and retries, and the answer goes back to the forge.

use super::view::AppView;
use crate::agent::post_actions::{run_action_blocking, ActionFailure, ActionJob};
use crate::agent::LlmClient;
use crate::domain::post_actions::{ActionFailureKind, RequestPolicy};
use crate::domain::spend::SpendEstimate;
use crate::ui::features::forge::view::{ForgeView, SuggestJob};
use gpui_kit::*;

impl AppView {
    /// Checks the key and the budget, then runs `job` and hands the answer to `forge`.
    pub(super) fn run_forge_suggest(&mut self, forge: &Entity<ForgeView>, job: SuggestJob, window: &mut Window, cx: &mut Context<Self>) {
        let generation = job.generation;
        if self.has_ai_key() && self.ask_for_forge(forge, &job, cx) {
            return;
        }
        match self.forge_suggest_job(job) {
            Ok(action) => {
                let task = cx.background_executor().spawn(async move { run_action_blocking(&action) });
                let forge = forge.downgrade();
                cx.spawn_in(window, async move |_, cx| {
                    let outcome = task.await;
                    let _ = forge.update(cx, |forge, cx| forge.suggestions_arrived(generation, outcome, cx));
                })
                .detach();
            }
            Err(failure) => forge.update(cx, |forge, cx| forge.suggestions_arrived(generation, Err(failure), cx)),
        }
    }

    /// Asks before spending on `job`; a confirm closed without sending stops the forge's request.
    fn ask_for_forge(&mut self, forge: &Entity<ForgeView>, job: &SuggestJob, cx: &mut Context<Self>) -> bool {
        let (run, stop, again) = (forge.clone(), forge.clone(), job.clone());
        let generation = job.generation;
        let resume: super::handlers::SpendResume = Box::new(move |this, window, cx| this.run_forge_suggest(&run, again, window, cx));
        let decline: super::handlers::SpendResume = Box::new(move |_, _, cx| {
            stop.update(cx, |forge, cx| forge.suggestions_arrived(generation, Err(ActionFailure::local(ActionFailureKind::Cancelled)), cx));
        });
        self.confirm_spend_or(SpendEstimate::one(), resume, Some(decline), cx)
    }

    /// The runner job of `job`, after spending its share of today's budget.
    fn forge_suggest_job(&mut self, job: SuggestJob) -> Result<ActionJob, ActionFailure> {
        let ai = self.state.preferences().ai.clone();
        let policy = RequestPolicy::from_prefs(&ai);
        let mut settings = self.state.llm_settings.clone();
        settings.max_tokens = settings.max_tokens.max(job.answer_tokens);
        let client = LlmClient::new(settings).with_timeout(policy.timeout);
        if !client.has_api_key() {
            return Err(ActionFailure::local(ActionFailureKind::NoKey));
        }
        self.charge_ai_request(&job.prompt, job.answer_tokens)?;
        Ok(ActionJob { client, prompt: job.prompt, policy, cancel: job.cancel })
    }
}
