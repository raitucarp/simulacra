//! Sending the forge's "suggest from context" requests (sb D1) like a post action: the shared
//! daily AI budget is checked and spent here, the request runs on the background executor
//! with the configured timeout and retries, and the answer goes back to the forge.

use super::view::AppView;
use crate::agent::post_actions::{run_action_blocking, ActionFailure, ActionJob};
use crate::agent::LlmClient;
use crate::domain::embedding::{estimate_tokens, BudgetLimits};
use crate::domain::post_actions::{ActionFailureKind, RequestPolicy};
use crate::ui::features::forge::view::{ForgeView, SuggestJob};
use gpui_kit::*;

/// Local calendar day key of the AI budget (shared with embeddings and post actions).
const DAY_FORMAT: &str = "%Y-%m-%d";

impl AppView {
    /// Checks the key and the budget, then runs `job` and hands the answer to `forge`.
    pub(super) fn run_forge_suggest(&mut self, forge: &Entity<ForgeView>, job: SuggestJob, window: &mut Window, cx: &mut Context<Self>) {
        let generation = job.generation;
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
        let today = chrono::Local::now().format(DAY_FORMAT).to_string();
        let tokens = estimate_tokens([job.prompt.system.as_str(), job.prompt.user.as_str()]).saturating_add(job.answer_tokens);
        if !self.semantic.usage.allows(BudgetLimits::from_prefs(&ai), &today, 1, tokens) {
            return Err(ActionFailure::local(ActionFailureKind::Budget));
        }
        self.semantic.usage = self.semantic.usage.recorded(&today, 1, tokens);
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_embedding_usage(&self.semantic.usage);
        }
        Ok(ActionJob { client, prompt: job.prompt, policy, cancel: job.cancel })
    }
}
