//! Belief memory in the shell (sb F1): the cheap extraction step after an agent posts,
//! replies, debates, answers an experiment or reads a thread (through the post actions
//! runner and the shared daily budget, capped per agent per day and switchable per agent),
//! the merge of what it proposes, and the user's controls on the profile's Beliefs tab.

use super::super::arena_host::PendingUpdate;
use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::beliefs::dossier::DOSSIER_BELIEFS;
use crate::domain::beliefs::extract::{extraction_prompt, parse_proposals, Material, EXTRACT_ANSWER_TOKENS};
use crate::domain::beliefs::{apply_proposals, apply_user_action, BeliefChange, BeliefId, BeliefOrigin, BeliefSource};
use crate::domain::{PostId, ProfileId};
use crate::state::NavigationTab;
use crate::ui::features::beliefs::{BeliefsEvent, BeliefsPanel};
use gpui_kit::*;

/// Local calendar day key of the daily cap (as the AI budget counts days).
const DAY_FORMAT: &str = "%Y-%m-%d";

pub(super) fn today() -> String {
    chrono::Local::now().format(DAY_FORMAT).to_string()
}

impl AppView {
    /// Runs one extraction for `agent` over `material` read from `source`, when its updates
    /// are on and today's cap allows (both are quiet no-ops otherwise).
    pub(super) fn queue_belief_update(&mut self, agent: &ProfileId, source: BeliefSource, material: Material, cx: &mut Context<Self>) {
        let today = today();
        let settings = self.arena.settings_of(agent);
        let Some(profile) = self.profile_of(agent).filter(|profile| profile.is_agent() && settings.allows(&today)).cloned() else { return };
        let now = chrono::Utc::now();
        let current: Vec<_> = self.arena.book.ranked(agent, now).into_iter().take(DOSSIER_BELIEFS).collect();
        let prompt = extraction_prompt(&profile.name, &self.language_of(Some(&profile)), &current, &material);
        match self.thinking_job(prompt, EXTRACT_ANSWER_TOKENS, CancelToken::new()) {
            Ok(job) => {
                let counted = settings.counted(&today);
                if let Some(vault) = self.state.db.as_ref() {
                    let _ = vault.save_belief_settings(agent, counted.clone());
                }
                self.arena.settings.insert(agent.clone(), counted);
                let ticket = self.arena.next_ticket();
                self.arena.updates.insert(ticket, PendingUpdate { agent: agent.clone(), source });
                self.spawn_thinking_job(job, ticket, Self::finish_belief_update, cx);
            }
            Err(failure) => self.arena.belief_notice = Some(failure_message(&failure)),
        }
        self.arena_changed(cx);
    }

    fn finish_belief_update(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let Some(pending) = self.arena.updates.remove(&ticket) else { return };
        match outcome {
            Ok(raw) => {
                let change = apply_proposals(&self.arena.book, &pending.agent, &parse_proposals(&raw), &pending.source, chrono::Utc::now(), BeliefId::random);
                self.apply_belief_change(change);
                self.arena.belief_notice = None;
            }
            Err(failure) => self.arena.belief_notice = Some(failure_message(&failure)),
        }
        self.arena_changed(cx);
    }

    /// Applies `change` to the beliefs and queues it for the vault.
    fn apply_belief_change(&mut self, change: BeliefChange) {
        if change.is_empty() {
            return;
        }
        self.arena.book = self.arena.book.applied(&change);
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_belief_change(change);
        }
    }

    pub(in crate::ui::features::shell) fn handle_beliefs_event(&mut self, _: &Entity<BeliefsPanel>, event: &BeliefsEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event.clone() {
            BeliefsEvent::Act(action) => self.apply_belief_change(apply_user_action(&self.arena.book, &action, chrono::Utc::now())),
            BeliefsEvent::SetUpdates(agent, on) => {
                let settings = self.arena.settings_of(&agent).switched(on);
                if let Some(vault) = self.state.db.as_ref() {
                    let _ = vault.save_belief_settings(&agent, settings.clone());
                }
                self.arena.settings.insert(agent, settings);
            }
            BeliefsEvent::ReadOpenThread(agent) => self.read_open_thread(&agent, cx),
            BeliefsEvent::OpenSource(source) => self.open_belief_source(source, window, cx),
        }
        self.arena_changed(cx);
    }

    /// Opens a belief's source: its post or thread in the feed, its debate or experiment.
    fn open_belief_source(&mut self, source: BeliefSource, window: &mut Window, cx: &mut Context<Self>) {
        let id = crate::domain::arena::ArenaId::new(source.id.clone());
        match source.origin {
            BeliefOrigin::Post | BeliefOrigin::Reply | BeliefOrigin::Thread => self.reveal_post(PostId::new(source.id), window, cx),
            BeliefOrigin::Debate => {
                self.select_nav(NavigationTab::Arena, window, cx);
                self.arena.panel.update(cx, |panel, cx| panel.open_debate(id, cx));
            }
            BeliefOrigin::Experiment => {
                self.select_nav(NavigationTab::Arena, window, cx);
                self.arena.panel.update(cx, |panel, cx| panel.open_experiment(id, cx));
            }
            BeliefOrigin::User => {}
        }
    }
}
