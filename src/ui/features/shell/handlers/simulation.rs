//! Autonomous agent simulation: background LLM synthesis and post insertion.
//!
//! Both the manual "run agent" action and the scheduler heartbeat go through
//! [`AppView::spawn_generation`], so every generation reports LLM failures and frees
//! its slot in the scheduler's generation queue when it finishes.

use super::super::view::AppView;
use crate::agent::{DialecticEngine, JobKind, SimulationOutcome};
use crate::domain::post::TriggerMode;
use crate::domain::{NetworkId, Post, Profile, ProfileId};
use gpui_kit::*;
use tokio::sync::oneshot::error::RecvError;

impl AppView {
    /// Triggers an autonomous synthesis for a specific network member agent (or the
    /// first active agent in the current network): by default a reply to the network's
    /// top post, or with [`TriggerMode::NewPost`] a new top-level post from its seed.
    /// Returns whether a generation started.
    pub fn trigger_simulation_for_agent(
        &mut self,
        agent_id: Option<ProfileId>,
        mode: TriggerMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let kind = self.trigger_job_kind(mode);
        let (Some(agent), Some(kind)) = (self.simulation_agent(agent_id.as_ref()), kind) else {
            return false;
        };
        if !self.scheduler.queue.start_now(&agent.id) {
            return false;
        }
        let network = self.state.active_network_id.clone();
        self.spawn_generation(agent, kind, network, window, cx);
        true
    }

    /// Triggers an autonomous dialectic heartbeat simulation in the current network.
    pub fn trigger_simulation(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.trigger_simulation_for_agent(None, TriggerMode::Reply, window, cx);
    }

    /// Generates `agent`'s post of `kind` (a reply or a new post) off the UI thread, then
    /// inserts it, reports any LLM failure and releases the agent's queue slot. The caller
    /// must already have marked the agent in flight.
    pub(super) fn spawn_generation(
        &mut self,
        agent: Profile,
        kind: JobKind,
        network: NetworkId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let agent_id = agent.id.clone();
        let job = self.engine_job(agent, kind, network);
        let rx = DialecticEngine::spawn_engine_simulation(job);
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
            // A5: a run the user cancelled while it was writing is dropped here.
            Ok(outcome) if self.lab.runs.finished(agent_id, Some(outcome.reply.id.clone()), false) => self.apply_generation(agent_id, outcome, window, cx),
            Ok(_) => {}
            Err(_) => {
                self.lab.runs.finished(agent_id, None, true);
                self.report_worker_failure(window, cx);
            }
        }
        self.pump_generation_queue(window, cx);
        self.pump_manual_run(agent_id, window, cx); // A5
        cx.notify();
    }

    /// Reports the outcome's failures, updates beliefs and inserts its posts and reactions.
    fn apply_generation(&mut self, agent_id: &ProfileId, outcome: SimulationOutcome, window: &mut Window, cx: &mut Context<Self>) {
        let SimulationOutcome { reply, thread, llm_error, gate_key, image_error, reactions } = outcome;
        let answered = reply.reply_to_id.clone(); // R8: the post its reactions go to
        let generated = llm_error.is_none(); // sb F1: only a model's reply updates beliefs
        match llm_error {
            Some(failure) => self.report_llm_failure(&gate_key, failure, window, cx),
            None => self.scheduler.failures.reset(&gate_key),
        }
        if let Some(err) = image_error {
            eprintln!("simulacra: image generation failed: {err}");
        }
        if generated {
            self.update_beliefs_from_reply(&reply, cx);
        }
        for post in std::iter::once(reply).chain(thread) {
            self.insert_simulated_reply(post, window, cx);
        }
        if answered.is_some_and(|post| self.state.react_as_agent(&post, agent_id, &reactions, jiff::Timestamp::now()) > 0) {
            self.feed.stream.remeasure();
        }
    }

    /// Persists `post` and inserts it after its parent (a new top-level post goes first),
    /// reporting a failed vault write.
    fn insert_simulated_reply(&mut self, post: Post, window: &mut Window, cx: &mut Context<Self>) {
        if let Err(err) = self.state.insert_reply_after_parent(post) {
            self.report_storage_failure(&err, window, cx);
        }
    }
}
