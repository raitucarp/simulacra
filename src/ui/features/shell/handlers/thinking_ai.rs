//! AI requests of the thinking tools (sb E1), all through the post actions runner: the job
//! is built with the configured client and the request policy (timeout, retries), after the
//! key and the shared daily AI budget allow it, and runs on the background executor.

use super::super::view::AppView;
use crate::agent::post_actions::{run_action_blocking, ActionFailure, ActionJob, CancelToken};
use crate::agent::LlmClient;
use crate::domain::post_actions::{ActionFailureKind, ActionPrompt, RequestPolicy};
use crate::ui::features::post_actions::failure_text;
use gpui_kit::*;

/// A finished request lands here (only if its ticket is still current).
pub(super) type Finish = fn(&mut AppView, u64, Result<String, ActionFailure>, &mut Context<AppView>);

impl AppView {
    /// Whether an AI request could be made at all (a key is configured).
    pub(in crate::ui::features::shell) fn has_ai_key(&self) -> bool {
        LlmClient::new(self.state.llm_settings.clone()).has_api_key()
    }

    /// A ready job for `prompt`, or why it cannot run (no key, budget spent).
    pub(super) fn thinking_job(&mut self, prompt: ActionPrompt, answer_tokens: u32, cancel: CancelToken) -> Result<ActionJob, ActionFailure> {
        let policy = RequestPolicy::from_prefs(&self.state.preferences().ai);
        let mut settings = self.state.llm_settings.clone();
        settings.max_tokens = settings.max_tokens.max(answer_tokens);
        let client = LlmClient::new(settings).with_timeout(policy.timeout);
        if !client.has_api_key() {
            return Err(ActionFailure::local(ActionFailureKind::NoKey));
        }
        self.charge_ai_request(&prompt, answer_tokens)?;
        Ok(ActionJob { client, prompt, policy, cancel })
    }

    /// Runs `job` in the background and hands its outcome to `finish` with `ticket`.
    pub(super) fn spawn_thinking_job(&mut self, job: ActionJob, ticket: u64, finish: Finish, cx: &mut Context<Self>) {
        let task = cx.background_executor().spawn(async move { run_action_blocking(&job) });
        cx.spawn(async move |this, cx| {
            let outcome = task.await;
            let _ = this.update(cx, |this, cx| finish(this, ticket, outcome, cx));
        })
        .detach();
    }
}

/// A failure as the user reads it.
pub(super) fn failure_message(failure: &ActionFailure) -> String {
    failure_text(failure.kind, &failure.detail)
}
