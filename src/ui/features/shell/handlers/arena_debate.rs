//! Running a debate (sb F1): a new debate is saved and its first slot spoken; each turn goes
//! through the speaker's own engine with the motion and the transcript (never anything
//! private), lands in the debate (saved at once), and the next slot follows until the judge.
//! A failure or a stop keeps every turn so far; Continue resumes at the next slot.

use super::super::arena_host::{ArenaRun, RunStep};
use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::arena::debate::{setup_problem, turn_prompt, Debate, DebateSetup, Phase, Slot, TURN_ANSWER_TOKENS};
use crate::domain::arena::ArenaId;
use crate::domain::post_actions::ActionFailureKind;
use crate::ui::features::arena::RunState;
use gpui_kit::*;

impl AppView {
    /// "Start debate": saves the debate of `setup` and speaks its first slot.
    pub(super) fn start_debate(&mut self, setup: DebateSetup, cx: &mut Context<Self>) {
        if setup_problem(&setup).is_some() || self.arena.run.is_some() {
            return;
        }
        let debate = Debate::new(ArenaId::random(), setup, chrono::Utc::now());
        let id = debate.id.clone();
        self.save_debate(debate);
        self.arena.panel.update(cx, |panel, cx| panel.open_debate(id.clone(), cx));
        self.advance_debate(&id, cx);
    }

    /// Speaks the next slot of debate `id`, or judges it once every slot spoke.
    pub(super) fn advance_debate(&mut self, id: &ArenaId, cx: &mut Context<Self>) {
        let Some(debate) = self.arena.debates.iter().find(|debate| &debate.id == id).cloned() else { return };
        self.arena.notice = None;
        match debate.phase() {
            Phase::Speaking(slot) => self.speak(&debate, slot, cx),
            Phase::Judging => self.judge_debate(&debate, cx),
            Phase::Done => self.arena.run = None,
        }
        self.arena_changed(cx);
    }

    /// Sends `slot`'s turn through its speaker's engine.
    fn speak(&mut self, debate: &Debate, slot: Slot, cx: &mut Context<Self>) {
        let Some(speaker) = self.profile_of(&slot.speaker).cloned() else {
            return self.arena_failed(ActionFailure::local(ActionFailureKind::Rejected));
        };
        let names = |id: &crate::domain::ProfileId| self.name_of(id);
        let user = turn_prompt(&debate.setup, &debate.turns, &slot, &names, &self.language_of(Some(&speaker)));
        let cancel = CancelToken::new();
        match self.agent_call(&speaker, user, TURN_ANSWER_TOKENS, cancel.clone()) {
            Ok(call) => {
                let ticket = self.arena.next_ticket();
                let state = RunState::Speaking(debate.id.clone(), slot.speaker.clone());
                self.arena.run = Some(ArenaRun { ticket, cancel, state, step: RunStep::Turn });
                self.spawn_arena_call(call, ticket, Self::finish_turn, cx);
            }
            Err(failure) => self.arena_failed(failure),
        }
    }

    /// A turn landed: kept in its debate, then the next slot.
    fn finish_turn(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let Some(RunState::Speaking(id, _)) = self.arena.run.as_ref().filter(|run| run.ticket == ticket).map(|run| run.state.clone()) else { return };
        match outcome {
            Ok(text) => {
                if let Some(debate) = self.arena.debates.iter().find(|debate| debate.id == id).cloned() {
                    self.save_debate(debate.with_turn(&text, chrono::Utc::now()));
                }
                self.advance_debate(&id, cx);
            }
            Err(failure) => {
                self.arena_failed(failure);
                self.arena_changed(cx);
            }
        }
    }

    /// Ends the run with `failure` said on the page (a stop says nothing).
    pub(super) fn arena_failed(&mut self, failure: ActionFailure) {
        self.arena.run = None;
        self.arena.notice = (failure.kind != ActionFailureKind::Cancelled).then(|| failure_message(&failure));
    }

    /// Keeps `debate` in memory (newest first) and queues it for the vault.
    pub(super) fn save_debate(&mut self, debate: Debate) {
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_arena_record(debate.clone());
        }
        self.arena.debates.retain(|kept| kept.id != debate.id);
        self.arena.debates.insert(0, debate);
    }
}
