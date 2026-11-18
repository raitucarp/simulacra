//! Arena requests in the shell (sb F1): an agent's call goes through its own engine (the
//! agent runtime) with its dossier and current beliefs as the system prompt; a judge without
//! a persona goes through the global model. Every call is charged to the shared daily AI
//! budget first and runs with the configured timeout, retries and the run's cancel flag.

use super::super::view::AppView;
use crate::agent::arena::{run_call_blocking, EngineCall};
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::agent::runtime::EnginePlan;
use crate::agent::{DialecticEngine, LlmClient};
use crate::domain::beliefs::beliefs_section;
use crate::domain::post_actions::{ActionFailureKind, ActionPrompt, RequestPolicy};
use crate::domain::{Profile, ProfileId};
use gpui_kit::*;

/// A landed Arena reply goes here (only while its ticket is current).
pub(super) type ArenaFinish = fn(&mut AppView, u64, Result<String, ActionFailure>, &mut Context<AppView>);

impl AppView {
    /// The profile `id`.
    pub(super) fn profile_of(&self, id: &ProfileId) -> Option<&Profile> {
        self.state.user_profiles.iter().find(|profile| &profile.id == id)
    }

    /// The display name of `id` (its id when the profile is gone).
    pub(super) fn name_of(&self, id: &ProfileId) -> String {
        self.profile_of(id).map(|profile| profile.name.clone()).unwrap_or_else(|| id.to_string())
    }

    /// The "Your current beliefs" section of `agent`'s prompt (empty without beliefs).
    pub(in crate::ui::features::shell) fn belief_dossier(&self, agent: &ProfileId) -> String {
        beliefs_section(&self.arena.book, agent, chrono::Utc::now())
    }

    /// `agent`'s full system prompt: its dossier in the active network, with its beliefs.
    fn agent_system_prompt(&self, agent: &Profile) -> String {
        let mut context = self.prompt_context(&self.state.active_network_id);
        context.beliefs = self.belief_dossier(&agent.id);
        DialecticEngine::system_prompt_in(agent, &context)
    }

    /// A call of `user` by `agent` through its own engine, charged to today's budget.
    pub(super) fn agent_call(&mut self, agent: &Profile, user: String, answer_tokens: u32, cancel: CancelToken) -> Result<EngineCall, ActionFailure> {
        let prompt = ActionPrompt { system: self.agent_system_prompt(agent), user };
        let policy = RequestPolicy::from_prefs(&self.state.preferences().ai);
        let mut global = self.state.llm_settings.clone();
        global.max_tokens = global.max_tokens.max(answer_tokens);
        let plan = EnginePlan::resolve(&agent.config.engine, &global, &self.state.credentials, self.state.media_dir());
        if !plan.is_ready() {
            return Err(ActionFailure::local(ActionFailureKind::NoKey));
        }
        self.charge_ai_request(&prompt, answer_tokens)?;
        Ok(EngineCall::new(plan, prompt, policy, cancel))
    }

    /// A call of `prompt` through the global model, charged to today's budget.
    pub(super) fn global_call(&mut self, prompt: ActionPrompt, answer_tokens: u32, cancel: CancelToken) -> Result<EngineCall, ActionFailure> {
        let policy = RequestPolicy::from_prefs(&self.state.preferences().ai);
        let mut settings = self.state.llm_settings.clone();
        settings.max_tokens = settings.max_tokens.max(answer_tokens);
        let client = LlmClient::new(settings).with_timeout(policy.timeout);
        if !client.has_api_key() {
            return Err(ActionFailure::local(ActionFailureKind::NoKey));
        }
        self.charge_ai_request(&prompt, answer_tokens)?;
        Ok(EngineCall::new(EnginePlan::from_client(client), prompt, policy, cancel))
    }

    /// Runs `call` in the background and hands its outcome to `finish` with `ticket`.
    pub(super) fn spawn_arena_call(&mut self, call: EngineCall, ticket: u64, finish: ArenaFinish, cx: &mut Context<Self>) {
        let task = cx.background_executor().spawn(async move { run_call_blocking(&call) });
        cx.spawn(async move |this, cx| {
            let outcome = task.await;
            let _ = this.update(cx, |this, cx| finish(this, ticket, outcome, cx));
        })
        .detach();
    }

    /// The language `agent` speaks (for the reply), or the user's interface language.
    pub(super) fn language_of(&self, agent: Option<&Profile>) -> String {
        agent
            .map(|profile| profile.config.primary_language.trim().to_string())
            .filter(|language| !language.is_empty())
            .unwrap_or_else(|| self.state.preferences().general.language.english_label().to_string())
    }
}
