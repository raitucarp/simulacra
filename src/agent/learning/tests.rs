//! The tutor's requests against the local mock server only (no live provider, no real key).

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::post_actions::CancelToken;
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::learning::tutor::machine::SessionStart;
use crate::domain::learning::{SessionId, TutorSession, TutorSource, Verdict};
use crate::domain::post_actions::{ActionFailureKind, RequestPolicy};
use chrono::Utc;
use serde_json::json;
use std::time::Duration;

const TEST_KEY: &str = "test-key-not-real";

fn answer(text: &str) -> MockReply {
    MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": text } }] }))
}

fn session() -> TutorSession {
    let start = SessionStart { network_id: crate::domain::NetworkId::new("net"), source: TutorSource::Concept { name: "necessity".into() }, title: "Necessity".into(), material: "Necessity: what could not be otherwise.".into() };
    TutorSession::start(SessionId("tutor_t".into()), start, Utc::now())
}

fn job(url: &str, request: TutorRequest, session: &TutorSession) -> ActionJob {
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: TEST_KEY.into(), model: "mock-model".into(), ..LlmSettings::default() };
    let timeout = Duration::from_secs(5);
    ActionJob { client: LlmClient::new(settings).with_endpoint(url).with_timeout(timeout), prompt: request.prompt(session, "English"), policy: RequestPolicy { timeout, max_retries: 0 }, cancel: CancelToken::new() }
}

#[test]
fn an_opening_question_comes_back_parsed() {
    let server = MockServer::start(vec![answer("{\"question\": \"What do you think necessity means?\"}")]);
    let session = session();
    let reply = run_tutor_blocking(job(&server.url, TutorRequest::Opening, &session), TutorRequest::Opening, None).expect("reply");
    assert_eq!(reply, TutorReply::Turn(TurnReply { assessment: None, question: "What do you think necessity means?".into() }));
    let body = server.requests()[0].json();
    let user = body["messages"][1]["content"].as_str().unwrap_or_default().to_string();
    assert!(user.contains("<material title=\"Necessity\">"), "{user}");
}

#[test]
fn a_turn_assesses_and_asks_again() {
    let reply = r#"{"assessment": {"level": "understand", "verdict": "solid", "right": ["own words"], "misconceptions": []}, "question": "Where would it fail?"}"#;
    let server = MockServer::start(vec![answer(reply)]);
    let session = session().with_opening("What is it?", Utc::now()).with_answer("What must be.", Utc::now());
    let TutorReply::Turn(turn) = run_tutor_blocking(job(&server.url, TutorRequest::Turn, &session), TutorRequest::Turn, None).unwrap() else { panic!("a turn") };
    assert_eq!(turn.assessment.map(|a| a.verdict), Some(Verdict::Solid));
    let next = session.with_turn(TurnReply { assessment: None, question: turn.question }, Utc::now());
    assert_eq!(next.exchanges.len(), 2);
}

#[test]
fn a_lecture_is_asked_for_once_more_with_a_nudge() {
    let lecture = answer("{\"question\": \"Necessity is the idea that everything follows from causes.\"}");
    let server = MockServer::start(vec![lecture, answer("{\"question\": \"Can you give an example?\"}")]);
    let session = session();
    let reply = run_tutor_blocking(job(&server.url, TutorRequest::Opening, &session), TutorRequest::Opening, None).unwrap();
    assert!(matches!(reply, TutorReply::Turn(TurnReply { ref question, .. }) if question == "Can you give an example?"));
    let requests = server.requests();
    assert_eq!(requests.len(), 2);
    assert!(requests[1].json()["messages"][1]["content"].as_str().unwrap_or_default().contains(RETRY_NUDGE));
}

#[test]
fn two_unusable_replies_fail_and_a_summary_parses() {
    let server = MockServer::start(vec![answer("no"), answer("still no")]);
    let failure = run_tutor_blocking(job(&server.url, TutorRequest::Opening, &session()), TutorRequest::Opening, None).unwrap_err();
    assert_eq!(failure, TutorFailure::Reply(ReplyProblem::NoQuestion));
    let summary = r#"{"shown": ["a definition"], "missing": ["an example"], "review": ["What is an example of necessity?"]}"#;
    let server = MockServer::start(vec![answer(summary)]);
    let reply = run_tutor_blocking(job(&server.url, TutorRequest::Summary, &session()), TutorRequest::Summary, Some(BloomLevel::Remember)).unwrap();
    assert!(matches!(reply, TutorReply::Summary(ref s) if s.review.len() == 1 && s.mastery == Some(BloomLevel::Remember)));
}

#[test]
fn a_rejected_key_is_a_request_failure() {
    let server = MockServer::start(vec![MockReply::raw(401, "application/json", "{}")]);
    let failure = run_tutor_blocking(job(&server.url, TutorRequest::Opening, &session()), TutorRequest::Opening, None).unwrap_err();
    assert!(matches!(failure, TutorFailure::Request(ActionFailure { kind: ActionFailureKind::Rejected, .. })));
}
