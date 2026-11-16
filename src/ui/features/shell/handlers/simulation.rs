//! Autonomous agent simulation: background LLM synthesis and reply insertion.
//!
//! Both the manual "run agent" action and the scheduler heartbeat go through
//! [`AppView::spawn_generation`], so every generation reports LLM failures and frees
//! its slot in the scheduler's generation queue when it finishes.

use super::super::view::AppView;
use crate::agent::{DialecticEngine, LlmClient, SimulationOutcome};
use crate::domain::{AuthoredPost, NetworkId, Post, Profile, ProfileId};
use gpui_kit::*;
use tokio::sync::oneshot::error::RecvError;

impl AppView {
    /// Triggers an autonomous synthesis for a specific network member agent (or the
    /// first active agent in the current network), replying to the network's top post.
    pub fn trigger_simulation_for_agent(
        &mut self,
        agent_id: Option<ProfileId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let parent = self.state.current_network_posts().next().map(|p| self.state.authored(p));
        let (Some(agent), Some(parent)) = (self.simulation_agent(agent_id.as_ref()), parent) else {
            return;
        };
        if !self.scheduler.queue.start_now(&agent.id) {
            return;
        }
        let network = self.state.active_network_id.clone();
        self.spawn_generation(agent, parent, network, window, cx);
    }

    /// Triggers an autonomous dialectic heartbeat simulation in the current network.
    pub fn trigger_simulation(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.trigger_simulation_for_agent(None, window, cx);
    }

    /// Generates `agent`'s reply to `parent` off the UI thread, then inserts it, reports
    /// any LLM failure and releases the agent's queue slot. The caller must already
    /// have marked the agent in flight.
    pub(super) fn spawn_generation(
        &mut self,
        agent: Profile,
        parent: AuthoredPost,
        network: NetworkId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let agent_id = agent.id.clone();
        let client = LlmClient::new(self.state.llm_settings.clone());
        let rx = DialecticEngine::spawn_background_simulation(parent, agent, network, client);
        cx.spawn_in(window, async move |entity, cx| {
            let outcome = rx.await;
            let _ = entity.update_in(cx, |this, window, cx| {
                this.finish_generation(&agent_id, outcome, window, cx);
            });
        })
        .detach();
    }

    /// Applies a finished generation and starts the next queued one.
    fn finish_generation(
        &mut self,
        agent_id: &ProfileId,
        outcome: Result<SimulationOutcome, RecvError>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.scheduler.queue.complete(agent_id);
        match outcome {
            Ok(SimulationOutcome { reply, llm_error }) => {
                if let Some(failure) = llm_error {
                    self.report_llm_failure(failure, window, cx);
                }
                self.insert_simulated_reply(reply, window, cx);
            }
            Err(_) => self.report_worker_failure(window, cx),
        }
        self.pump_generation_queue(window, cx);
        cx.notify();
    }

    /// The requested agent, or else the first active (then first) agent of the network.
    fn simulation_agent(&self, agent_id: Option<&ProfileId>) -> Option<Profile> {
        if let Some(target_id) = agent_id {
            return self
                .state
                .user_profiles
                .iter()
                .find(|p| &p.id == target_id && p.is_agent())
                .cloned();
        }
        let agents = self.state.current_network_agents();
        agents
            .iter()
            .find(|p| p.config.is_active)
            .or_else(|| agents.first())
            .map(|p| (*p).clone())
    }

    /// Persists `reply` and inserts it after its parent, reporting a failed vault write.
    fn insert_simulated_reply(&mut self, reply: Post, window: &mut Window, cx: &mut Context<Self>) {
        if let Err(err) = self.state.insert_reply_after_parent(reply) {
            self.report_storage_failure(&err, window, cx);
        }
    }
}
