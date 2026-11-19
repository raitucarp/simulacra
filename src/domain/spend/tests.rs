//! Asking before spending: when a confirm is due, what an accepted one allows, and the
//! request counts the confirm states.

use super::*;
use crate::domain::arena::debate::{Debate, DebateFormat, DebateSetup, Debater, Judge, Side};
use crate::domain::arena::lab::{CellAnswer, Experiment, Scenario, Stance, Variable};
use crate::domain::arena::ArenaId;
use crate::domain::ProfileId;

const ASK: bool = true;
const DONT_ASK: bool = false;

fn now() -> jiff::Timestamp {
    "2026-11-18T10:00:00Z".parse::<jiff::Timestamp>().expect("time")
}

#[test]
fn a_fresh_session_asks_only_when_the_preference_is_on() {
    let consent = SpendConsent::default();
    assert!(consent.must_ask(ASK));
    assert!(!consent.must_ask(DONT_ASK));
    assert!(!consent.allows(ASK), "nothing may be charged before a confirm");
    assert!(consent.allows(DONT_ASK), "with the preference off every request may be charged");
}

#[test]
fn an_accepted_confirm_lets_its_action_start_once_and_send_what_it_stated() {
    let consent = SpendConsent::default().granted(SpendEstimate::requests(2), false);
    assert!(!consent.must_ask(ASK), "the action the confirm was for starts without asking again");
    let running = consent.started();
    assert!(running.must_ask(ASK), "the next action asks again");
    assert!(running.allows(ASK));
    let once = running.spent();
    assert!(once.allows(ASK));
    assert!(!once.spent().allows(ASK), "no more than the confirm stated");
}

#[test]
fn a_new_confirm_replaces_what_was_left_over() {
    let left = SpendConsent::default().granted(SpendEstimate::requests(5), false).started().spent();
    let next = left.granted(SpendEstimate::one(), false).started().spent();
    assert!(!next.allows(ASK), "earlier leftovers never add to a new confirm");
}

#[test]
fn remembering_the_choice_stops_every_later_confirm_this_session() {
    let consent = SpendConsent::default().granted(SpendEstimate::one(), true).started().spent().spent();
    assert!(consent.is_remembered());
    assert!(!consent.must_ask(ASK));
    assert!(consent.allows(ASK));
    assert!(consent.granted(SpendEstimate::one(), false).is_remembered(), "a later confirm never forgets it");
}

#[test]
fn an_estimate_always_counts_at_least_one_request() {
    assert_eq!(SpendEstimate::requests(0).requests, 1);
    assert_eq!(SpendEstimate::one().requests, 1);
    assert_eq!(SpendEstimate::requests(7).requests, 7);
}

fn debate() -> Debate {
    let debaters = [("a", Side::For), ("b", Side::Against)].into_iter().map(|(id, side)| Debater { agent: ProfileId::new(id), side }).collect();
    let setup = DebateSetup { motion: "Cities should ban private cars".into(), source_post: None, debaters, format: DebateFormat::Oxford, rounds: 1, moderator: None, judge: Judge::GlobalModel };
    Debate::new(ArenaId::new("d"), setup, now())
}

#[test]
fn a_debate_counts_its_open_slots_the_judging_and_one_update_per_debater() {
    let fresh = debate();
    let slots = fresh.total_turns() as u32;
    assert_eq!(debate_requests(&fresh), slots + 2 + 2);
    let spoken = fresh.with_turn("An opening.", now());
    assert_eq!(debate_requests(&spoken), slots - 1 + 2 + 2);
    let all = (0..slots).fold(fresh, |debate, _| debate.with_turn("A turn.", now()));
    assert_eq!(debate_requests(&all), 2 + 2, "only the judging and the updates are left");
}

fn experiment() -> Experiment {
    let means = Variable { name: "Means".into(), values: vec!["lever".into(), "push".into()] };
    let scenario = Scenario { title: "Trolley".into(), premise: "A runaway trolley.".into(), variables: vec![means], question: "Do you act?".into() };
    Experiment::new(ArenaId::new("x"), scenario, vec![ProfileId::new("a"), ProfileId::new("b")], None, now())
}

#[test]
fn an_experiment_counts_its_unanswered_cells_and_one_update_per_agent() {
    let fresh = experiment();
    assert_eq!(experiment_requests(&fresh), 2 * 2 + 2);
    let answer = |agent: &str, cell| CellAnswer { agent: ProfileId::new(agent), cell, stance: Stance::Yes, verdict: "v".into(), reasoning: String::new() };
    let one = fresh.with_answer(answer("a", 0), now());
    assert_eq!(experiment_requests(&one), 3 + 2);
    let done = [("b", 0), ("a", 1), ("b", 1)].into_iter().fold(one, |e, (agent, cell)| e.with_answer(answer(agent, cell), now()));
    assert_eq!(experiment_requests(&done), 0, "a finished grid sends nothing");
}
