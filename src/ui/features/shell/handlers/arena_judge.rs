//! Judging a debate (sb F1): the judge (a chosen agent through its own engine, or the global
//! model) scores every debater on the rubric; then the transcript is mapped with the thinking
//! tools' argument extraction, so the page can show which claims still stand. The debate is
//! saved judged (with or without a map) and every debater's beliefs are updated from it.

use super::super::arena_host::{ArenaRun, RunStep};
use super::super::view::AppView;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::arena::debate::{effective_judge, judge_prompt, judge_task, parse_verdict, turn_posts, Debate, Judge, Verdict, JUDGE_ANSWER_TOKENS};
use crate::domain::post_actions::ActionFailureKind;
use crate::domain::thinking::argument::{extraction_prompt, parse_extraction, ArgumentMap, EXTRACT_ANSWER_TOKENS};
use crate::domain::ProfileId;
use crate::ui::features::arena::RunState;
use gpui_kit::*;

impl AppView {
    /// Sends the judge's request for `debate`.
    pub(super) fn judge_debate(&mut self, debate: &Debate, cx: &mut Context<Self>) {
        let judge = effective_judge(&debate.setup);
        let names = |id: &ProfileId| self.name_of(id);
        let cancel = CancelToken::new();
        let call = match &judge {
            Judge::Agent(agent) => match self.profile_of(agent).cloned() {
                Some(profile) => {
                    let user = judge_task(&debate.setup, &debate.turns, &names, &self.language_of(Some(&profile)));
                    self.agent_call(&profile, user, JUDGE_ANSWER_TOKENS, cancel.clone())
                }
                None => Err(ActionFailure::local(ActionFailureKind::Rejected)),
            },
            Judge::GlobalModel => {
                let prompt = judge_prompt(&debate.setup, &debate.turns, &names, &self.language_of(None));
                self.global_call(prompt, JUDGE_ANSWER_TOKENS, cancel.clone())
            }
        };
        match call {
            Ok(call) => {
                let ticket = self.arena.next_ticket();
                self.arena.run = Some(ArenaRun { ticket, cancel, state: RunState::Judging(debate.id.clone()), step: RunStep::Judge });
                self.spawn_arena_call(call, ticket, Self::finish_judge, cx);
            }
            Err(failure) => self.arena_failed(failure),
        }
    }

    /// The running debate, while `ticket` is current.
    fn judged_debate(&self, ticket: u64) -> Option<Debate> {
        let run = self.arena.run.as_ref().filter(|run| run.ticket == ticket)?;
        let RunState::Judging(id) = &run.state else { return None };
        self.arena.debates.iter().find(|debate| &debate.id == id).cloned()
    }

    fn finish_judge(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let Some(debate) = self.judged_debate(ticket) else { return };
        let judge = effective_judge(&debate.setup);
        match outcome.and_then(|raw| parse_verdict(&raw, &debate.setup, judge).ok_or(ActionFailure::local(ActionFailureKind::Empty))) {
            Ok(verdict) => self.map_debate(&debate, verdict, cx),
            Err(failure) => self.arena_failed(failure),
        }
        self.arena_changed(cx);
    }

    /// Maps the transcript's arguments; without a key or budget the verdict is kept alone.
    fn map_debate(&mut self, debate: &Debate, verdict: Verdict, cx: &mut Context<Self>) {
        let posts = turn_posts(debate, &|id| self.name_of(id));
        let prompt = extraction_prompt(&posts, &self.language_of(None));
        let cancel = CancelToken::new();
        match self.global_call(prompt, EXTRACT_ANSWER_TOKENS, cancel.clone()) {
            Ok(call) => {
                let ticket = self.arena.next_ticket();
                let state = RunState::Judging(debate.id.clone());
                self.arena.run = Some(ArenaRun { ticket, cancel, state, step: RunStep::Map(verdict) });
                self.spawn_arena_call(call, ticket, Self::finish_map, cx);
            }
            Err(_) => self.close_debate(debate, verdict, None, cx),
        }
    }

    fn finish_map(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let Some(debate) = self.judged_debate(ticket) else { return };
        let Some(RunStep::Map(verdict)) = self.arena.run.as_ref().map(|run| run.step.clone()) else { return };
        let posts = turn_posts(&debate, &|id| self.name_of(id));
        let map = outcome.ok().and_then(|raw| parse_extraction(&raw, &posts)).map(|extraction| extraction.map);
        self.close_debate(&debate, verdict, map, cx);
        self.arena_changed(cx);
    }

    /// Saves the judged debate, ends the run and lets every debater revise its beliefs.
    fn close_debate(&mut self, debate: &Debate, verdict: Verdict, map: Option<ArgumentMap>, cx: &mut Context<Self>) {
        let judged = debate.with_verdict(verdict, map, jiff::Timestamp::now());
        self.arena.run = None;
        self.save_debate(judged.clone());
        for debater in &judged.setup.debaters {
            self.update_beliefs_from_debate(&debater.agent, &judged, cx);
        }
    }
}
