//! Autonomous agent simulation: background LLM synthesis and reply insertion.

use super::super::view::AppView;
use crate::agent::{DialecticEngine, LlmClient};
use crate::domain::{Post, Profile, ProfileId};
use crate::state::AppState;
use gpui_kit::*;

impl AppView {
    /// Triggers an autonomous synthesis for a specific network member agent (or the first active agent in the current network).
    pub fn trigger_simulation_for_agent(
        &mut self,
        agent_id: Option<ProfileId>,
        cx: &mut Context<Self>,
    ) {
        let active_net = self.state.active_network_id.clone();
        let agent_opt = self.simulation_agent(agent_id.as_ref());
        let last_post_opt = self.state.current_network_posts().next().cloned();
        let (Some(agent), Some(last_post)) = (agent_opt, last_post_opt) else {
            return;
        };
        let client = LlmClient::new(self.state.llm_settings.clone());
        let rx = DialecticEngine::spawn_background_simulation(last_post, agent, active_net, client);
        cx.spawn(async |entity, cx| {
            if let Ok(reply) = rx.await {
                let _ = entity.update(cx, |this, cx| {
                    this.insert_simulated_reply(reply);
                    cx.notify();
                });
            }
        })
        .detach();
    }

    /// Triggers an autonomous dialectic heartbeat simulation in the current network.
    pub fn trigger_simulation(&mut self, cx: &mut Context<Self>) {
        self.trigger_simulation_for_agent(None, cx);
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

    /// Persists `reply`, bumps its parent's reply count and inserts it after the parent.
    fn insert_simulated_reply(&mut self, reply: Post) {
        let state = &mut self.state;
        let parent_idx_opt = reply
            .reply_to_id
            .as_ref()
            .and_then(|parent_id| state.post_index_by_id.get(parent_id).copied());

        if let (Some(parent_id), Some(parent_idx)) = (&reply.reply_to_id, parent_idx_opt) {
            if let Some(parent) = state.posts.get_mut(parent_idx) {
                parent.replies_count += 1;
                if let Some(ref db) = state.db {
                    let _ = db.update_post_replies_count(parent_id, parent.replies_count);
                }
            }
        }
        if let Some(ref db) = state.db {
            let _ = db.save_post(&reply);
        }
        let excerpt = AppState::compute_wikilink_excerpt(&reply);
        state.wikilink_cache.insert(reply.id.as_str().to_string(), excerpt);
        let insert_pos = parent_idx_opt.map_or(0, |idx| (idx + 1).min(state.posts.len()));
        state.posts.insert(insert_pos, reply);
        state.sync_post_index();
    }
}
