use super::{at, session_on};
use crate::domain::learning::bloom::BloomLevel;
use crate::domain::learning::tutor::progress::{mastery, misses_in_a_row, next_level, next_move, should_close, MAX_EXCHANGES};
use crate::domain::learning::tutor::prompt::{opening_prompt, summary_prompt, turn_prompt};
use crate::domain::learning::tutor::{gap_drafts, Assessment, SocraticMove, TurnReply, TutorPhase, TutorSession, TutorSource, TutorSummary, Verdict};
use crate::domain::review::{Anchor, AnchorSide};

fn assessed(verdict: Verdict, reached: Option<BloomLevel>, misconceptions: &[&str]) -> Assessment {
    Assessment { reached, verdict, right: vec!["a point".into()], misconceptions: misconceptions.iter().map(|m| m.to_string()).collect() }
}

fn turn(verdict: Verdict, question: &str) -> TurnReply {
    TurnReply { assessment: Some(assessed(verdict, None, &[])), question: question.into() }
}

/// A session past its opening with one answer waiting for assessment.
fn answered_once() -> TutorSession {
    session_on("p1").with_opening("What do you think freedom is?", at(0)).with_answer("Doing what I want.", at(0))
}

#[test]
fn the_phases_follow_the_dialogue() {
    let session = session_on("p1");
    assert_eq!(session.phase(), TutorPhase::NeedsQuestion);
    let session = session.with_opening("What do you think freedom is?", at(0));
    assert_eq!(session.phase(), TutorPhase::AwaitingAnswer);
    assert_eq!(session.exchanges[0].question.kind, SocraticMove::Elicit);
    assert_eq!(session.clone().with_answer("   ", at(0)).phase(), TutorPhase::AwaitingAnswer, "a blank answer is ignored");
    let session = session.with_answer("Doing what I want.", at(0));
    assert_eq!(session.phase(), TutorPhase::NeedsAssessment);
    let session = session.with_turn(turn(Verdict::Solid, "Can you explain that in your own words?"), at(1));
    assert_eq!(session.phase(), TutorPhase::AwaitingAnswer);
    assert_eq!(session.exchanges.len(), 2);
    assert_eq!(session.updated_at, at(1));
    let closed = session.close(at(2));
    assert_eq!(closed.phase(), TutorPhase::NeedsSummary);
    assert_eq!(closed.exchanges.len(), 1, "the unanswered question is dropped");
    let summary = TutorSummary { shown: vec!["x".into()], missing: vec![], review: vec!["Why?".into()], mastery: closed.mastery() };
    let done = closed.with_summary(summary, at(2));
    assert_eq!(done.phase(), TutorPhase::Finished);
    assert!(!done.needs_request());
}

#[test]
fn transitions_out_of_phase_change_nothing() {
    let fresh = session_on("p1");
    assert_eq!(fresh.clone().with_answer("early", at(0)), fresh);
    assert_eq!(fresh.clone().with_turn(turn(Verdict::Solid, "Q?"), at(0)), fresh);
    assert_eq!(fresh.clone().close(at(1)), fresh, "nothing answered, nothing to close");
    let waiting = fresh.with_opening("Q?", at(0));
    assert_eq!(waiting.clone().with_opening("Again?", at(0)), waiting);
}

#[test]
fn a_solid_answer_climbs_and_a_missing_one_steps_down_after_two() {
    let solid = assessed(Verdict::Solid, Some(BloomLevel::Understand), &[]);
    let partial = assessed(Verdict::Partial, None, &[]);
    let missing = assessed(Verdict::Missing, None, &[]);
    assert_eq!(next_level(BloomLevel::Understand, &solid, 0), BloomLevel::Apply);
    assert_eq!(next_level(BloomLevel::Understand, &partial, 0), BloomLevel::Understand);
    assert_eq!(next_level(BloomLevel::Apply, &missing, 1), BloomLevel::Apply, "first miss: probe at the same level");
    assert_eq!(next_level(BloomLevel::Apply, &missing, 2), BloomLevel::Understand);
    assert_eq!(next_level(BloomLevel::Create, &solid, 0), BloomLevel::Create);
}

#[test]
fn the_session_places_questions_by_the_rules() {
    let session = answered_once().with_turn(turn(Verdict::Solid, "Explain it?"), at(1));
    assert_eq!(session.exchanges[1].question.level, BloomLevel::Understand);
    assert_eq!(session.exchanges[1].question.kind, SocraticMove::Explain);
    let session = session.with_answer("It is acting from reasons.", at(1)).with_turn(turn(Verdict::Missing, "An example?"), at(1));
    assert_eq!(session.exchanges[2].question.level, BloomLevel::Understand);
    assert_eq!(session.exchanges[2].question.kind, SocraticMove::Example);
    let session = session.with_answer("Not sure.", at(1)).with_turn(turn(Verdict::Missing, "What does free mean?"), at(1));
    assert_eq!(misses_in_a_row(&session.exchanges[..2 + 1], BloomLevel::Understand), 2);
    assert_eq!(session.exchanges[3].question.level, BloomLevel::Remember, "two misses in a row step down");
    assert_eq!(session.exchanges[3].question.kind, SocraticMove::Example);
}

#[test]
fn misconceptions_get_a_counterexample_then_a_contradiction() {
    let session = answered_once();
    let wrong = assessed(Verdict::Partial, None, &["confuses freedom with licence"]);
    let level = next_level(BloomLevel::Remember, &wrong, 0);
    assert_eq!(next_move(level, &session.exchanges[0], &wrong), SocraticMove::Counterexample);
    let mut probed = session.exchanges[0].clone();
    probed.question.kind = SocraticMove::Counterexample;
    assert_eq!(next_move(level, &probed, &wrong), SocraticMove::Contradiction);
    let partial = assessed(Verdict::Partial, None, &[]);
    assert_eq!(next_move(BloomLevel::Remember, &session.exchanges[0], &partial), SocraticMove::Define);
}

#[test]
fn a_missing_assessment_reads_as_partial() {
    let session = answered_once().with_turn(TurnReply { assessment: None, question: "Say more?".into() }, at(1));
    assert_eq!(session.exchanges[0].assessment.as_ref().map(|a| a.verdict), Some(Verdict::Partial));
    assert_eq!(session.exchanges[1].question.level, BloomLevel::Remember);
}

#[test]
fn mastery_is_the_highest_level_shown() {
    let mut session = answered_once().with_turn(turn(Verdict::Solid, "Explain?"), at(1));
    assert_eq!(mastery(&session.exchanges), Some(BloomLevel::Remember));
    session = session.with_answer("Acting from understanding.", at(1)).with_turn(TurnReply { assessment: Some(assessed(Verdict::Partial, Some(BloomLevel::Apply), &[])), question: "Q?".into() }, at(1));
    assert_eq!(session.mastery(), Some(BloomLevel::Apply), "a partial answer counts what it reached");
    assert_eq!(mastery(&[]), None);
}

#[test]
fn the_dialogue_closes_after_enough_answers_or_at_the_top() {
    let mut session = answered_once();
    for _ in 0..MAX_EXCHANGES - 1 {
        session = session.with_turn(turn(Verdict::Partial, "Again?"), at(1)).with_answer("an answer", at(1));
    }
    assert!(!should_close(&session.exchanges));
    let closed = session.with_turn(turn(Verdict::Partial, "One more?"), at(1));
    assert_eq!(closed.phase(), TutorPhase::NeedsSummary, "the tenth assessment closes it");
    let mut top = answered_once();
    top.exchanges[0].question.level = BloomLevel::Create;
    assert_eq!(top.with_turn(turn(Verdict::Solid, "More?"), at(1)).phase(), TutorPhase::NeedsSummary);
}

#[test]
fn prompts_quote_the_material_and_state_the_rules() {
    let session = answered_once();
    let opening = opening_prompt(&session_on("p1"), "English");
    assert!(opening.system.contains("never by lecturing") && opening.system.contains("Write every text value in English."));
    assert!(opening.user.contains("<material title=\"Freedom\">") && opening.user.contains("Freedom is understanding necessity."));
    let turn = turn_prompt(&session, "German");
    assert!(turn.user.contains("<learner>Doing what I want.</learner>"));
    assert!(turn.user.contains("if solid, aim at \"understand\""), "{}", turn.user);
    assert!(turn.system.contains("\"verdict\": \"solid|partial|missing\""));
    assert!(turn.system.contains("German"));
    assert!(summary_prompt(&session, "English").system.contains("\"review\""));
}

#[test]
fn gap_items_are_anchored_to_the_source() {
    let summary = TutorSummary { shown: vec![], missing: vec!["the role of causes".into()], review: vec!["What makes an action free?".into(), "Why do causes matter?".into()], mastery: None };
    let mut session = answered_once();
    session.summary = Some(summary.clone());
    let drafts = gap_drafts(&session, &[1, 7]);
    assert_eq!(drafts.len(), 1, "an index out of range is skipped");
    assert_eq!(drafts[0].anchor, Anchor::Post { id: "p1".into() });
    assert_eq!(drafts[0].side, AnchorSide::Back);
    assert_eq!(drafts[0].written, "Why do causes matter?");
    session.source = TutorSource::Concept { name: "freedom".into() };
    assert!(gap_drafts(&session, &[0]).is_empty(), "a typed concept has no source to anchor to");
    session.source = TutorSource::Document { id: "doc".into() };
    assert!(matches!(&gap_drafts(&session, &[0])[0].anchor, Anchor::Passage { document_id, text, .. } if document_id == "doc" && text.contains("necessity")));
}
