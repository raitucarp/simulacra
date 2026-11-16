//! Autonomous scheduler heartbeat: asks the pure scheduling core which agents are due,
//! queues them, and feeds the bounded generation queue into the simulation pipeline.

use super::super::scheduler_state::HEARTBEAT_INTERVAL;
use super::super::view::AppView;
use crate::agent::schedule::{due_agents, latest_activity, AgentSlot, Clock, ScheduleContext, SystemClock};
use crate::domain::{NetworkId, Post, Profile, ProfileId};
use chrono::Utc;
use gpui_kit::*;

impl AppView {
    /// Starts the heartbeat loop; it runs for the view's lifetime and is a no-op while
    /// the scheduler switch is off.
    pub(in crate::ui::features::shell) fn start_scheduler_heartbeat(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let task = cx.spawn_in(window, async move |entity, cx| loop {
            cx.background_executor().timer(HEARTBEAT_INTERVAL).await;
            let ticked = entity.update_in(cx, |this, window, cx| this.scheduler_tick(window, cx));
            if ticked.is_err() {
                break;
            }
        });
        self.scheduler.heartbeat = Some(task);
    }

    /// Turns the global scheduler switch on or off and persists it. Switching on runs a
    /// heartbeat immediately instead of waiting a full interval; switching off drops
    /// queued (not yet started) work.
    pub(crate) fn set_scheduler_enabled(&mut self, enabled: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.scheduler.enabled = enabled;
        self.state.set_scheduler_enabled(enabled);
        if enabled {
            self.scheduler_tick(window, cx);
        } else {
            self.scheduler.queue.clear_pending();
        }
        cx.notify();
    }

    /// One heartbeat: enqueue every due agent, then start queued generations.
    fn scheduler_tick(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.scheduler.enabled {
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

    /// Starts queued generations while a concurrency slot is free.
    pub(super) fn pump_generation_queue(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        while let Some(id) = self.scheduler.queue.start_next() {
            self.scheduler.last_dispatch.insert(id.clone(), Utc::now());
            match self.scheduled_job(&id) {
                Some((agent, parent, network)) => self.spawn_generation(agent, parent, network, window, cx),
                None => self.scheduler.queue.complete(&id),
            }
        }
    }

    /// The agent, the post it should answer and its network, if it has anything to answer.
    fn scheduled_job(&self, id: &ProfileId) -> Option<(Profile, Post, NetworkId)> {
        let agent = self.state.user_profiles.iter().find(|p| &p.id == id && p.is_agent())?;
        let network = agent.network_id.clone().unwrap_or_else(|| self.state.active_network_id.clone());
        let parent = self
            .state
            .posts
            .iter()
            .filter(|post| post.network_id == network && post.author.id != agent.id)
            .max_by_key(|post| post.created_at)?;
        Some((agent.clone(), parent.clone(), network))
    }
}
