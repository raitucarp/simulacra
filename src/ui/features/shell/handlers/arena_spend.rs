//! Asking before the Arena spends (sb P16): a debate or an experiment states every request it
//! still needs (turns or cells, the judging, the belief updates after it) before it starts or
//! continues, and "Read the open thread" its one extraction. Each resumes once accepted.

use super::super::view::AppView;
use crate::domain::arena::debate::{setup_problem, Debate, DebateSetup};
use crate::domain::arena::ArenaId;
use crate::domain::spend::{debate_requests, experiment_requests, SpendEstimate};
use crate::domain::ProfileId;
use gpui_kit::*;

impl AppView {
    /// "Start debate", after the confirm.
    pub(super) fn begin_debate(&mut self, setup: DebateSetup, cx: &mut Context<Self>) {
        if setup_problem(&setup).is_some() || self.arena.run.is_some() {
            return;
        }
        let estimate = SpendEstimate::requests(debate_requests(&Debate::new(ArenaId::random(), setup.clone(), chrono::Utc::now())));
        let again = setup.clone();
        if !self.confirm_spend(estimate, Box::new(move |this, _, cx| this.begin_debate(again, cx)), cx) {
            self.start_debate(setup, cx);
        }
        self.arena_changed(cx);
    }

    /// "Continue" on a stopped debate, after the confirm.
    pub(super) fn continue_debate(&mut self, id: ArenaId, cx: &mut Context<Self>) {
        let Some(requests) = self.arena.debates.iter().find(|debate| debate.id == id).map(debate_requests) else { return };
        let again = id.clone();
        if requests == 0 || !self.confirm_spend(SpendEstimate::requests(requests), Box::new(move |this, _, cx| this.continue_debate(again, cx)), cx) {
            self.advance_debate(&id, cx);
        }
        self.arena_changed(cx);
    }

    /// Runs (or resumes) experiment `id`, after the confirm.
    pub(super) fn run_experiment(&mut self, id: ArenaId, cx: &mut Context<Self>) {
        let Some(requests) = self.arena.experiments.iter().find(|experiment| experiment.id == id).map(experiment_requests) else { return };
        let again = id.clone();
        if requests == 0 || !self.confirm_spend(SpendEstimate::requests(requests), Box::new(move |this, _, cx| this.run_experiment(again, cx)), cx) {
            self.advance_experiment(&id, cx);
        }
        self.arena_changed(cx);
    }

    /// "Read the open thread", after the confirm.
    pub(super) fn read_thread_asking(&mut self, agent: ProfileId, cx: &mut Context<Self>) {
        let again = agent.clone();
        if !self.confirm_spend(SpendEstimate::one(), Box::new(move |this, _, cx| this.read_thread_asking(again, cx)), cx) {
            self.read_open_thread(&agent, cx);
        }
        self.arena_changed(cx);
    }
}
