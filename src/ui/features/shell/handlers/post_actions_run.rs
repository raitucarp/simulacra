//! Running a post action (sb C2): a remembered answer is shown at once; otherwise the daily AI
//! budget is checked, the prompt built and sent on the background executor with the
//! configured timeout and retries, and the answer parsed when it lands (if still current).

use super::super::view::AppView;
use crate::agent::post_actions::{run_action_blocking, ActionFailure, ActionJob, CancelToken};
use crate::agent::LlmClient;
use crate::domain::embedding::{estimate_tokens, BudgetLimits};
use crate::domain::post_actions::{build_prompt, parse_output, ActionFailureKind, ActionKind, ActionPrompt, RequestPolicy};
use crate::domain::PostId;
use crate::ui::features::post_actions::{failure_text, ActionStatus};
use gpui_kit::*;

/// Local calendar day key of the AI budget (shared with embedding requests).
const DAY_FORMAT: &str = "%Y-%m-%d";

/// What a finished run belongs to.
struct RunKey {
    generation: u64,
    post_id: PostId,
    request_key: String,
    kind: ActionKind,
}

impl AppView {
    /// Runs the sheet's request; `fresh` skips a remembered answer (re-run).
    pub(super) fn run_post_action(&mut self, fresh: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.post_actions.cancel_running();
        let Some(sheet) = self.post_actions.sheet.clone() else { return };
        let request = sheet.request.clone();
        let hits = (request.kind == ActionKind::SearchConcept && request.is_complete()).then(|| self.concept_hits(&sheet.post_id, &request.concept));
        let remembered = self.post_actions.answers.get(&(sheet.post_id.clone(), request.cache_key())).filter(|_| !fresh).cloned();
        let status = match (request.is_complete(), remembered) {
            (false, _) => ActionStatus::NeedsChoice,
            (true, Some(output)) => ActionStatus::Done(output),
            (true, None) => match self.start_action_job(&sheet.post_id, window, cx) {
                Ok(()) => ActionStatus::Running,
                Err(failure) => ActionStatus::Failed(failure_text(failure.kind, &failure.detail)),
            },
        };
        if let Some(open) = self.post_actions.sheet.as_mut() {
            open.status = status;
            open.hits = hits;
        }
        cx.notify();
    }

    /// Checks the budget and the key, then sends the request in the background.
    fn start_action_job(&mut self, post_id: &PostId, window: &mut Window, cx: &mut Context<Self>) -> Result<(), ActionFailure> {
        let request = self.post_actions.sheet.as_ref().map(|sheet| sheet.request.clone()).ok_or(ActionFailure::local(ActionFailureKind::Empty))?;
        let context = self.action_context(post_id).ok_or(ActionFailure::local(ActionFailureKind::Empty))?;
        let prompt = build_prompt(&request, &context);
        let ai = self.state.preferences().ai.clone();
        let policy = RequestPolicy::from_prefs(&ai);
        let mut settings = self.state.llm_settings.clone();
        settings.max_tokens = settings.max_tokens.max(request.kind.answer_tokens());
        let client = LlmClient::new(settings).with_timeout(policy.timeout);
        if !client.has_api_key() {
            return Err(ActionFailure::local(ActionFailureKind::NoKey));
        }
        self.charge_ai_request(&prompt, request.kind.answer_tokens())?;
        let cancel = CancelToken::new();
        self.post_actions.cancel = Some(cancel.clone());
        let key = RunKey { generation: self.post_actions.generation, post_id: post_id.clone(), request_key: request.cache_key(), kind: request.kind };
        self.spawn_action_job(ActionJob { client, prompt, policy, cancel }, key, window, cx);
        Ok(())
    }

    /// Charges one request of `prompt` (with an answer of up to `answer_tokens`) to today's AI
    /// budget, shared by post actions, embeddings and the thinking tools (sb E1).
    pub(in crate::ui::features::shell) fn charge_ai_request(&mut self, prompt: &ActionPrompt, answer_tokens: u32) -> Result<(), ActionFailure> {
        let ai = self.state.preferences().ai.clone();
        let today = chrono::Local::now().format(DAY_FORMAT).to_string();
        let tokens = estimate_tokens([prompt.system.as_str(), prompt.user.as_str()]).saturating_add(answer_tokens);
        if !self.semantic.usage.allows(BudgetLimits::from_prefs(&ai), &today, 1, tokens) {
            return Err(ActionFailure::local(ActionFailureKind::Budget));
        }
        self.semantic.usage = self.semantic.usage.recorded(&today, 1, tokens);
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_embedding_usage(&self.semantic.usage);
        }
        Ok(())
    }

    fn spawn_action_job(&mut self, job: ActionJob, key: RunKey, window: &mut Window, cx: &mut Context<Self>) {
        let task = cx.background_executor().spawn(async move { run_action_blocking(&job) });
        cx.spawn_in(window, async move |this, cx| {
            let outcome = task.await;
            let _ = this.update_in(cx, |this, _, cx| this.finish_post_action(key, outcome, cx));
        })
        .detach();
    }

    /// Shows a finished answer (or why there is none) unless a newer run replaced it.
    fn finish_post_action(&mut self, key: RunKey, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        if key.generation != self.post_actions.generation {
            return;
        }
        self.post_actions.cancel = None;
        let status = match outcome.map(|text| parse_output(key.kind, &text)) {
            Ok(output) if output.is_empty() => ActionStatus::Failed(failure_text(ActionFailureKind::Empty, "")),
            Ok(output) => {
                self.post_actions.answers.insert((key.post_id.clone(), key.request_key), output.clone());
                ActionStatus::Done(output)
            }
            Err(failure) => ActionStatus::Failed(failure_text(failure.kind, &failure.detail)),
        };
        if let Some(sheet) = self.post_actions.sheet.as_mut().filter(|sheet| sheet.post_id == key.post_id) {
            sheet.status = status;
        }
        cx.notify();
    }
}
