//! Autonomous scheduler heartbeat: asks the pure scheduling core which agents are due,
//! queues them, and feeds the bounded generation queue into the simulation pipeline.

use super::super::scheduler_state::HEARTBEAT_INTERVAL;
use super::super::view::AppView;
use crate::agent::runtime::EnginePlan;
use crate::agent::JobKind;
use crate::agent::schedule::{due_agents, latest_activity, AgentSlot, Clock, ScheduleContext, SystemClock};
use crate::domain::{AuthoredPost, NetworkId, Profile, ProfileId};
use crate::state::recommend::Recommender;
use jiff::Timestamp;
use gpui_kit::*;

impl AppView {
    /// Starts the heartbeat loop; it runs for the view's lifetime. Every tick publishes due
    /// scheduled posts (Y2, also once right away); agents' turns are a no-op while the
    /// scheduler switch is off.
    pub(in crate::ui::features::shell) fn start_scheduler_heartbeat(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let task = cx.spawn_in(window, async move |entity, cx| {
            let _ = entity.update_in(cx, |this, window, cx| this.post_lifecycle_tick(&SystemClock, window, cx));
            loop {
                cx.background_executor().timer(HEARTBEAT_INTERVAL).await;
                let ticked = entity.update_in(cx, |this, window, cx| {
                    this.post_lifecycle_tick(&SystemClock, window, cx);
                    this.scheduler_tick(window, cx);
                });
                if ticked.is_err() {
                    break;
                }
            }
        });
        self.scheduler.heartbeat = Some(task);
    }

    /// Follows a changed switch. Switching on runs a heartbeat immediately instead of waiting
    /// a full interval; switching off drops queued (not yet started) work.
    pub(super) fn on_scheduler_switched(&mut self, enabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.scheduler.enabled = enabled;
        if enabled {
            self.scheduler_tick(window, cx);
        } else {
            self.scheduler.queue.clear_pending();
        }
        cx.notify();
    }

    /// One heartbeat: enqueue every due agent, then start queued generations. Paused during
    /// quiet hours and while no API key is configured anywhere (one notice); agents whose
    /// engine is backing off after failures, or has no key, are skipped when their turn comes.
    fn scheduler_tick(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.scheduler.enabled || self.state.preferences().agents.is_quiet_at(crate::time::local_now().hour() as u32) {
            return;
        }
        if !self.state.llm_settings.has_api_key() && !self.state.credentials.any_secret() {
            self.report_scheduler_needs_key(window, cx);
            return;
        }
        self.scheduler.tick = self.scheduler.tick.wrapping_add(1);
        for id in self.due_agent_ids(&SystemClock) {
            let _ = self.scheduler.queue.enqueue(id);
        }
        self.pump_generation_queue(window, cx);
    }

    /// Agents the scheduling core says are due at `clock`'s current time.
    fn due_agent_ids(&self, clock: &dyn Clock) -> Vec<ProfileId> {
        let last_posts = latest_activity(&self.state.posts);
        let slots: Vec<AgentSlot<'_>> = self
            .state
            .user_profiles
            .iter()
            .filter(|profile| profile.is_agent())
            .map(|profile| AgentSlot {
                id: &profile.id,
                config: &profile.config,
                last_activity: self.scheduler.last_activity(&profile.id, last_posts.get(&profile.id).copied()),
            })
            .collect();
        let ctx = ScheduleContext { now: clock.now(), seed: self.scheduler.seed, tick: self.scheduler.tick };
        due_agents(&slots, &ctx)
    }

    /// Starts queued generations while a concurrency slot is free; an agent whose engine has
    /// no key or is backing off after failures is dropped from the queue (it is re-queued
    /// when next due).
    pub(super) fn pump_generation_queue(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        while let Some(id) = self.scheduler.queue.start_next() {
            match self.scheduled_job(&id).filter(|(agent, _, _)| self.engine_allowed(agent)) {
                Some((agent, parent, network)) => {
                    self.scheduler.last_dispatch.insert(id.clone(), Timestamp::now());
                    self.spawn_generation(agent, JobKind::Reply(parent), network, window, cx);
                }
                None => self.scheduler.queue.complete(&id),
            }
        }
    }

    /// Whether `agent`'s engine has a key and is not backing off.
    fn engine_allowed(&self, agent: &Profile) -> bool {
        let (key, ready) = EnginePlan::gate_of(&agent.config.engine, &self.state.llm_settings, &self.state.credentials);
        ready && self.scheduler.failures.allows_generation(&key, Timestamp::now())
    }

    /// The agent, the post it should answer and its network, if it has anything to answer:
    /// the recent post closest to what the agent writes and answers (post embeddings), or the
    /// newest post by someone else while its interests are not embedded yet.
    fn scheduled_job(&self, id: &ProfileId) -> Option<(Profile, AuthoredPost, NetworkId)> {
        let agent = self.state.user_profiles.iter().find(|p| &p.id == id && p.is_agent())?;
        let network = agent.network_id.clone().unwrap_or_else(|| self.state.active_network_id.clone());
        let model = self.embedding_config().model;
        let recommender = Recommender { state: &self.state, index: &self.semantic.index, model: &model, now: Timestamp::now() };
        let by_interest = recommender.agent_answer(agent, &network).and_then(|post| self.state.get_post(&post));
        let newest = || self.state.posts.iter().filter(|post| post.network_id == network && post.author_id != agent.id && !post.is_locked()).max_by_key(|post| post.created_at);
        let parent = by_interest.or_else(newest)?;
        Some((agent.clone(), self.state.authored(parent), network))
    }
}
