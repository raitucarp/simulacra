//! Keeping the Arena page and the Beliefs tab in step (sb F1): on every frame the theme, and
//! fresh data whenever the records, the network's agents, the key, today's budget, the open
//! thread or the selected agent changed.

use super::super::view::AppView;
use super::beliefs::today;
use crate::domain::beliefs::history::credence_series;
use crate::domain::beliefs::quota::DAILY_UPDATES_PER_AGENT;
use crate::domain::embedding::BudgetLimits;
use crate::domain::ProfileId;
use crate::ui::features::arena::{AgentChoice, ArenaData, OpenThread};
use crate::ui::features::beliefs::{BeliefRow, BeliefsData, HISTORY_ROWS};
use gpui_kit::*;

impl AppView {
    /// The agent the roster shows (the selection, else the network's first agent).
    pub(in crate::ui::features::shell) fn roster_agent(&self) -> Option<ProfileId> {
        let agents = self.state.current_network_agents();
        let selected = self.lab.selected_profile_id.as_ref().filter(|id| agents.iter().any(|agent| &agent.id == *id)).cloned();
        selected.or_else(|| agents.first().map(|agent| agent.id.clone()))
    }

    fn arena_data(&self) -> ArenaData {
        let agents = self.state.current_network_agents().into_iter().map(|agent| AgentChoice { id: agent.id.clone(), name: agent.name.clone(), handle: agent.display_handle() }).collect();
        let open_thread = self.feed.selected_thread_post_id.as_ref().and_then(|id| self.state.get_post(id)).map(|post| OpenThread { post: post.id.clone(), text: post.content.clone() });
        ArenaData {
            agents,
            debates: self.arena.debates.clone(),
            experiments: self.arena.experiments.clone(),
            run: self.arena.run.as_ref().map(|run| run.state.clone()).unwrap_or_default(),
            notice: self.arena.notice.clone(),
            has_key: self.state.llm_settings.has_api_key(),
            usage: self.semantic.usage.clone(),
            limits: Some(BudgetLimits::from_prefs(&self.state.preferences().ai)),
            today: today(),
            open_thread,
        }
    }

    fn beliefs_data(&self, agent: Option<ProfileId>) -> BeliefsData {
        let Some(agent) = agent else { return BeliefsData::default() };
        let book = &self.arena.book;
        let rows = book
            .ranked(&agent, chrono::Utc::now())
            .into_iter()
            .map(|belief| BeliefRow { belief: belief.clone(), series: credence_series(book, belief), history: book.history(&belief.id).rev().take(HISTORY_ROWS).cloned().collect() })
            .collect();
        let settings = self.arena.settings_of(&agent);
        BeliefsData {
            agent_name: self.name_of(&agent),
            used_today: settings.used_on(&today()),
            daily_cap: DAILY_UPDATES_PER_AGENT,
            updating: self.arena.updates.values().any(|update| update.agent == agent),
            thread_open: self.feed.selected_thread_post_id.is_some(),
            notice: self.arena.belief_notice.clone(),
            agent: Some(agent),
            rows,
            settings,
        }
    }

    /// Render hook: the theme, and fresh data when anything the views show changed.
    pub(in crate::ui::features::shell) fn sync_arena(&mut self, cx: &mut Context<Self>) {
        let theme = self.theme.clone();
        self.arena.panel.update(cx, |panel, cx| panel.set_theme(&theme, cx));
        self.arena.beliefs_panel.update(cx, |panel, cx| panel.set_theme(&theme, cx));
        let open = self.feed.selected_thread_post_id.as_ref().map(|id| id.to_string());
        let stamp = (self.arena.version, self.state.active_network_id.to_string(), self.state.user_profiles.len(), self.state.llm_settings.has_api_key(), self.semantic.usage.clone(), open, self.roster_agent());
        if self.arena.stamp.as_ref() == Some(&stamp) {
            return;
        }
        let (arena, beliefs) = (self.arena_data(), self.beliefs_data(stamp.6.clone()));
        self.arena.stamp = Some(stamp);
        self.arena.panel.update(cx, |panel, cx| panel.set_data(arena, cx));
        self.arena.beliefs_panel.update(cx, |panel, cx| panel.set_data(beliefs, cx));
    }
}
