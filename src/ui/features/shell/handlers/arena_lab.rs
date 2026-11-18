//! Running a thought experiment (sb F1): the experiment is saved, then each agent answers
//! each cell in turn through its own engine (one request at a time, each charged to the
//! daily budget), every answer saved as it lands. A reply without a readable stance is kept
//! as undecided with its first line, so one odd answer never stops the grid. When the grid
//! is complete, each agent's beliefs are updated from its answers.

use super::super::arena_host::{ArenaRun, RunStep};
use super::super::view::AppView;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::arena::lab::prompt::MAX_VERDICT_CHARS;
use crate::domain::arena::lab::{cell_prompt, cells, parse_cell_answer, scenario_problem, CellAnswer, Experiment, Scenario, Stance, CELL_ANSWER_TOKENS};
use crate::domain::arena::ArenaId;
use crate::domain::post_actions::ActionFailureKind;
use crate::domain::ProfileId;
use crate::ui::features::arena::RunState;
use gpui_kit::*;

impl AppView {
    /// Saves a new experiment (or replaces saved `id`, cleared) and, when `run`, starts it.
    pub(super) fn submit_experiment(&mut self, id: Option<ArenaId>, scenario: Scenario, agents: Vec<ProfileId>, preset: Option<String>, run: bool, cx: &mut Context<Self>) {
        if scenario_problem(&scenario).is_some() || agents.is_empty() || (run && self.arena.run.is_some()) {
            return;
        }
        let now = chrono::Utc::now();
        let experiment = match id.and_then(|id| self.arena.experiments.iter().find(|kept| kept.id == id).cloned()) {
            Some(kept) => Experiment { scenario, agents, preset, ..kept.cleared(now) },
            None => Experiment::new(ArenaId::random(), scenario, agents, preset, now),
        };
        let id = experiment.id.clone();
        self.save_experiment(experiment);
        self.arena.panel.update(cx, |panel, cx| panel.open_experiment(id.clone(), cx));
        if run {
            self.advance_experiment(&id, cx);
        }
        self.arena_changed(cx);
    }

    /// Asks the next agent the next unanswered cell of experiment `id`.
    pub(super) fn advance_experiment(&mut self, id: &ArenaId, cx: &mut Context<Self>) {
        let Some(experiment) = self.arena.experiments.iter().find(|kept| &kept.id == id).cloned() else { return };
        self.arena.notice = None;
        let Some((agent, cell)) = experiment.pending().into_iter().next() else {
            self.arena.run = None;
            return self.update_beliefs_from_experiment(&experiment, cx);
        };
        let Some(profile) = self.profile_of(&agent).cloned() else {
            return self.arena_failed(ActionFailure::local(ActionFailureKind::Rejected));
        };
        let grid = cells(&experiment.scenario);
        let user = cell_prompt(&experiment.scenario, &grid[cell], &self.language_of(Some(&profile)));
        let cancel = CancelToken::new();
        match self.agent_call(&profile, user, CELL_ANSWER_TOKENS, cancel.clone()) {
            Ok(call) => {
                let ticket = self.arena.next_ticket();
                let step = RunStep::Cell(agent, cell);
                self.arena.run = Some(ArenaRun { ticket, cancel, state: RunState::Answering(experiment.id.clone()), step });
                self.spawn_arena_call(call, ticket, Self::finish_cell, cx);
            }
            Err(failure) => self.arena_failed(failure),
        }
    }

    fn finish_cell(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let Some(run) = self.arena.run.as_ref().filter(|run| run.ticket == ticket) else { return };
        let (RunState::Answering(id), RunStep::Cell(agent, cell)) = (run.state.clone(), run.step.clone()) else { return };
        match outcome {
            Ok(raw) => {
                let (stance, verdict, reasoning) = parse_cell_answer(&raw).unwrap_or_else(|| (Stance::Undecided, first_line(&raw), String::new()));
                if let Some(experiment) = self.arena.experiments.iter().find(|kept| kept.id == id).cloned() {
                    self.save_experiment(experiment.with_answer(CellAnswer { agent, cell, stance, verdict, reasoning }, chrono::Utc::now()));
                }
                self.advance_experiment(&id, cx);
            }
            Err(failure) => self.arena_failed(failure),
        }
        self.arena_changed(cx);
    }

    /// Keeps `experiment` in memory (newest first) and queues it for the vault.
    pub(super) fn save_experiment(&mut self, experiment: Experiment) {
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_arena_record(experiment.clone());
        }
        self.arena.experiments.retain(|kept| kept.id != experiment.id);
        self.arena.experiments.insert(0, experiment);
    }
}

/// The first non-blank line of `raw`, shortened to a verdict.
fn first_line(raw: &str) -> String {
    raw.lines().map(str::trim).find(|line| !line.is_empty()).unwrap_or_default().chars().take(MAX_VERDICT_CHARS).collect()
}
