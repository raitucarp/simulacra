//! N9: asking the room: only agents that may read the post are asked, their replies (through
//! the agent engine, against the local mock server only) are its ranked answers, one can be
//! accepted, and a locked post is never asked nor shows its answers.

use super::locked::locked_post;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::runtime::EnginePlan;
use crate::agent::{DialecticEngine, EngineJob, JobKind, LlmClient, LlmProvider, LlmSettings, PromptContext};
use crate::domain::post::audience::Audience;
use crate::domain::post::question::Asked;
use crate::domain::{Profile, ProfileId};
use crate::error::AppError;
use crate::state::store::AppState;
use serde_json::json;
use std::time::Duration;

fn with_agents() -> AppState {
    let mut state = AppState::new_in_memory();
    for (id, name) in [("ada", "Ada"), ("bo", "Bo")] {
        let mut agent = Profile::new_agent(id, name, &format!("@{id}"), "optician");
        agent.network_id = Some(state.active_network_id.clone());
        state.user_profiles.push(agent);
    }
    state
}

fn asked(agent: &str, fit: u16) -> Asked {
    Asked { agent: ProfileId::new(agent), fit }
}

/// Ada's answer, written by the engine against the mock server.
fn engine_answer(state: &AppState, question: &crate::domain::PostId) -> crate::domain::Post {
    let server = MockServer::start(vec![MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": "Light slows in glass, so it bends." } }] }))]);
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key-not-real".into(), model: "mock-model".into(), ..LlmSettings::default() };
    let client = LlmClient::new(settings).with_endpoint(&server.url).with_timeout(Duration::from_secs(5));
    let agent = state.user_profiles.iter().find(|p| p.id.as_str() == "ada").expect("ada").clone();
    let parent = state.authored(state.get_post(question).expect("question"));
    let job = EngineJob { kind: JobKind::Reply(parent), agent, network: state.active_network_id.clone(), context: PromptContext::default(), plan: EnginePlan::from_client(client), tools: None, roll: 0 };
    let outcome = DialecticEngine::run_engine_job(job);
    assert!(outcome.llm_error.is_none(), "{:?}", outcome.llm_error);
    outcome.reply
}

#[test]
fn the_asked_agents_answers_are_ranked_and_one_is_accepted() {
    let mut state = with_agents();
    let question = state.submit_post("Why does light bend in water?").expect("question");
    assert!(state.ranked_answers(&question).is_empty(), "never asked");
    state.ask_room(&question, vec![asked("ada", 900), asked("bo", 750)]).expect("asked");
    let answer = engine_answer(&state, &question);
    let ada = answer.id.clone();
    state.insert_reply_after_parent(answer).expect("answer stored");
    let mut late = crate::domain::Post::create_reply("post_bo", state.active_network_id.clone(), "bo", "Refraction.", question.clone(), None, None).expect("reply");
    late.likes = 4;
    state.insert_reply_after_parent(late).expect("stored");
    let me = state.submit_reply(&question, "Thanks both.", None, None).expect("mine");
    let order: Vec<_> = state.ranked_answers(&question).into_iter().map(|a| a.post).collect();
    assert_eq!(order, vec!["post_bo".into(), ada.clone()], "likes lift Bo's answer; the user's own reply is no answer");
    state.accept_answer(&question, &ada).expect("accepted");
    assert_eq!(state.ranked_answers(&question)[0].post, ada);
    assert_eq!(state.get_post(&question).and_then(|p| p.extras.question.as_ref()?.accepted.clone()), Some(ada.clone()));
    assert!(state.accept_answer(&question, &me).is_err(), "only an answer can be accepted");
    state.accept_answer(&question, &ada).expect("taken back");
    assert_eq!(state.get_post(&question).and_then(|p| p.extras.question.as_ref()?.accepted.clone()), None);
}

#[test]
fn only_agents_in_the_scope_are_asked() {
    let mut state = with_agents();
    let question = state.submit_post("For Ada only: how do lenses focus?").expect("question");
    state.set_audience(&question, Some(Audience::Agents { agents: vec![ProfileId::new("ada")] })).expect("scoped");
    let room: Vec<_> = state.room_for(state.get_post(&question).expect("question")).into_iter().map(|p| p.id.as_str().to_string()).collect();
    assert_eq!(room, vec!["ada"]);
    assert!(state.ask_room(&question, vec![asked("bo", 900)]).is_err(), "Bo may not read it");
    assert!(state.ask_room(&question, Vec::new()).is_err());
    state.ask_room(&question, vec![asked("ada", 900)]).expect("asked");
}

#[test]
fn a_locked_post_is_never_asked_and_shows_no_answers() {
    let mut state = with_agents();
    let locked = locked_post(&mut state);
    assert!(state.room_for(state.get_post(&locked).expect("locked")).is_empty());
    assert_eq!(state.ask_room(&locked, vec![asked("ada", 900)]), Err(AppError::PostLocked));
    let at = state.post_position(&locked).expect("stored");
    state.posts[at].extras.question = Some(Box::new(crate::domain::post::question::Question { asked: vec![asked("ada", 900)], accepted: None }));
    let reply = crate::domain::Post::create_reply("post_old", state.active_network_id.clone(), "ada", "An old answer.", locked.clone(), None, None).expect("reply");
    state.insert_reply_after_parent(reply).expect("stored");
    assert!(state.ranked_answers(&locked).is_empty(), "a sealed question lists nothing");
}
