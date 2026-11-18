//! The Arena's entry points in the shell (sb F1): palette commands, the post actions "Debate
//! this" and "Run a thought experiment on this", the profile's Profile / Beliefs switch, and
//! the page's requests.

use super::super::view::AppView;
use crate::domain::arena::debate::Debate;
use crate::domain::arena::lab::Experiment;
use crate::domain::arena::ArenaId;
use crate::domain::post_actions::ActionKind;
use crate::domain::PostId;
use crate::state::NavigationTab;
use crate::ui::features::arena::{ArenaEvent, ArenaPanel, ArenaTab};
use crate::ui::features::command_palette::CommandId;
use crate::ui::features::roster::ProfileTab;
use gpui_kit::*;

impl AppView {
    /// Go to the Arena, a new debate, a new experiment, or either on the open thought.
    pub(in crate::ui::features::shell) fn open_arena(&mut self, command: CommandId, window: &mut Window, cx: &mut Context<Self>) {
        let open = self.feed.selected_thread_post_id.clone();
        match (command, open) {
            (CommandId::DebateOpenThought, Some(post)) => self.open_arena_tool(post, ActionKind::Debate, window, cx),
            (CommandId::ExperimentOpenThought, Some(post)) => self.open_arena_tool(post, ActionKind::ThoughtExperiment, window, cx),
            (CommandId::NewThoughtExperiment | CommandId::ExperimentOpenThought, _) => {
                self.select_nav(NavigationTab::Arena, window, cx);
                self.arena.panel.update(cx, |panel, cx| panel.show(ArenaTab::Lab, cx));
            }
            (CommandId::NewDebate | CommandId::DebateOpenThought, _) => {
                self.select_nav(NavigationTab::Arena, window, cx);
                self.arena.panel.update(cx, |panel, cx| panel.new_debate_tab(cx));
            }
            _ => self.select_nav(NavigationTab::Arena, window, cx),
        }
    }

    /// "…" › Actions › Debate this / Run a thought experiment on this.
    pub(in crate::ui::features::shell) fn open_arena_tool(&mut self, post_id: PostId, kind: ActionKind, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(&self.state.action_target(&post_id)).cloned() else { return };
        self.select_nav(NavigationTab::Arena, window, cx);
        self.arena.panel.update(cx, |panel, cx| match kind {
            ActionKind::ThoughtExperiment => panel.prefill_premise(post.content.clone(), window, cx),
            _ => panel.prefill_motion(post.id.clone(), post.content.clone(), window, cx),
        });
    }

    /// The selected agent's overview shows its profile or its beliefs.
    pub(crate) fn switch_profile_tab(&mut self, tab: ProfileTab, _: &mut Window, cx: &mut Context<Self>) {
        self.arena.profile_tab = tab;
        self.arena_changed(cx);
    }

    pub(in crate::ui::features::shell) fn handle_arena_event(&mut self, _: &Entity<ArenaPanel>, event: &ArenaEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event.clone() {
            ArenaEvent::StartDebate(setup) => self.start_debate(setup, cx),
            ArenaEvent::ContinueDebate(id) if self.arena.run.is_none() => self.advance_debate(&id, cx),
            ArenaEvent::ContinueDebate(_) => {}
            ArenaEvent::Cancel => self.arena.stop(),
            ArenaEvent::PublishDebate(id) => self.publish_debate(&id, window, cx),
            ArenaEvent::DeleteDebate(id) => self.delete_arena_record::<Debate>(&id),
            ArenaEvent::SaveExperiment { id, scenario, agents, preset } => self.submit_experiment(id, scenario, agents, preset, false, cx),
            ArenaEvent::RunExperiment { id, scenario, agents, preset } => self.submit_experiment(id, scenario, agents, preset, true, cx),
            ArenaEvent::ResumeExperiment(id) if self.arena.run.is_none() => self.advance_experiment(&id, cx),
            ArenaEvent::ResumeExperiment(_) => {}
            ArenaEvent::PublishExperiment(id) => self.publish_experiment(&id, window, cx),
            ArenaEvent::DeleteExperiment(id) => self.delete_arena_record::<Experiment>(&id),
            ArenaEvent::OpenPost(id) => self.reveal_post(id, window, cx),
        }
        self.arena_changed(cx);
    }

    /// Deletes a saved debate or experiment.
    fn delete_arena_record<T: crate::storage::db::ArenaRecord>(&mut self, id: &ArenaId) {
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.delete_arena_record::<T>(id);
        }
        self.arena.debates.retain(|debate| &debate.id != id);
        self.arena.experiments.retain(|experiment| &experiment.id != id);
    }

    /// Marks the records changed so the page is rebuilt on the next frame.
    pub(in crate::ui::features::shell) fn arena_changed(&mut self, cx: &mut Context<Self>) {
        self.arena.version += 1;
        cx.notify();
    }
}
