//! N14: retracting and correcting the user's posts: a reason is needed, a locked post and
//! anyone else's are refused, and the mark can be taken back; the agents that cited a post
//! are told once (only those that may read it), and an agent reads the mark with the post.

use super::locked::locked_post;
use crate::domain::post::correction::CorrectionKind;
use crate::state::store::AppState;
use jiff::Timestamp;

#[test]
fn a_post_is_marked_with_a_reason_and_the_mark_can_be_taken_back() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("Honey never spoils.").expect("posted");
    assert!(state.mark_correction(&id, CorrectionKind::Retracted, "   ", Timestamp::now()).is_err(), "a reason is needed");
    let mark = state.mark_correction(&id, CorrectionKind::Corrected, "It can ferment if wet.", Timestamp::now()).expect("marked");
    assert_eq!((mark.kind, mark.reason.as_str()), (CorrectionKind::Corrected, "It can ferment if wet."));
    assert_eq!(state.get_post(&id).and_then(|p| p.extras.correction.as_deref().map(|c| c.kind)), Some(CorrectionKind::Corrected));
    state.mark_correction(&id, CorrectionKind::Retracted, "The study was a hoax.", Timestamp::now()).expect("remarked");
    assert_eq!(state.get_post(&id).and_then(|p| p.extras.correction.as_deref().map(|c| c.kind)), Some(CorrectionKind::Retracted));
    state.withdraw_correction(&id).expect("withdrawn");
    assert!(state.get_post(&id).is_some_and(|p| p.extras.correction.is_none()));
}

#[test]
fn a_locked_post_and_an_agents_post_are_never_marked() {
    let mut state = AppState::new_in_memory();
    let locked = locked_post(&mut state);
    assert_eq!(state.mark_correction(&locked, CorrectionKind::Retracted, "Leaky reason.", Timestamp::now()).err(), Some(crate::error::AppError::PostLocked));
    assert!(state.get_post(&locked).is_some_and(|p| p.extras.correction.is_none()));
    let agent_post = state.posts.iter().find(|post| state.author_of(post).is_agent()).map(|post| post.id.clone());
    if let Some(agent_post) = agent_post {
        assert!(state.mark_correction(&agent_post, CorrectionKind::Retracted, "Not mine to retract.", Timestamp::now()).is_err());
    }
}

fn with_agents() -> AppState {
    let mut state = AppState::new_in_memory();
    for (id, name) in [("ada", "Ada"), ("bo", "Bo")] {
        let mut agent = crate::domain::Profile::new_agent(id, name, &format!("@{id}"), "historian");
        agent.network_id = Some(state.active_network_id.clone());
        state.user_profiles.push(agent);
    }
    state
}

fn reply_by(state: &mut AppState, agent: &str, parent: &crate::domain::PostId) {
    let reply = crate::domain::Post::create_reply(format!("post_{agent}"), state.active_network_id.clone(), agent, "Citing this.", parent.clone(), None, None).expect("reply");
    state.insert_reply_after_parent(reply).expect("stored");
}

#[test]
fn the_agents_that_cited_a_post_are_told_once_and_only_within_its_scope() {
    let mut state = with_agents();
    let claim = state.submit_post("The battle was in 1415.").expect("posted");
    reply_by(&mut state, "ada", &claim);
    assert!(state.citers_to_tell(&claim).iter().any(|agent| agent.id.as_str() == "ada"));
    assert!(state.citers_to_tell(&claim).iter().all(|agent| agent.id.as_str() != "bo"), "Bo never cited it");
    state.mark_correction(&claim, CorrectionKind::Corrected, "It was 1416.", Timestamp::now()).expect("marked");
    state.record_told(&claim, &[crate::domain::ProfileId::new("ada")]).expect("told");
    assert!(state.citers_to_tell(&claim).is_empty(), "Ada is told once");
    state.mark_correction(&claim, CorrectionKind::Retracted, "No source at all.", Timestamp::now()).expect("remarked");
    assert!(state.citers_to_tell(&claim).is_empty(), "and stays told");
    let scoped = state.submit_post("Only for me: a guess.").expect("posted");
    reply_by(&mut state, "bo", &scoped);
    state.set_audience(&scoped, Some(crate::domain::post::audience::Audience::OnlyMe)).expect("scoped");
    assert!(state.citers_to_tell(&scoped).is_empty(), "N8: nobody outside the scope is told");
}

#[test]
fn an_agent_told_of_a_retraction_reads_it_with_the_post() {
    use crate::agent::mock_server::{MockReply, MockServer};
    use crate::agent::runtime::EnginePlan;
    use crate::agent::{DialecticEngine, EngineJob, JobKind, LlmClient, LlmProvider, LlmSettings, PromptContext};
    let mut state = with_agents();
    let claim = state.submit_post("Eating carrots sharpens night vision.").expect("posted");
    state.mark_correction(&claim, CorrectionKind::Retracted, "That was wartime propaganda.", Timestamp::now()).expect("marked");
    let server = MockServer::start(vec![MockReply::json(serde_json::json!({ "choices": [{ "message": { "role": "assistant", "content": "Good catch: the myth hid radar." } }] }))]);
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key-not-real".into(), model: "mock-model".into(), ..LlmSettings::default() };
    let client = LlmClient::new(settings).with_endpoint(&server.url).with_timeout(std::time::Duration::from_secs(5));
    let agent = state.user_profiles.iter().find(|p| p.id.as_str() == "ada").expect("ada").clone();
    let parent = state.authored(state.get_post(&claim).expect("claim"));
    let job = EngineJob { kind: JobKind::Reply(parent), agent, network: state.active_network_id.clone(), context: PromptContext::default(), plan: EnginePlan::from_client(client), tools: None, roll: 0 };
    let outcome = DialecticEngine::run_engine_job(job);
    assert!(outcome.llm_error.is_none(), "{:?}", outcome.llm_error);
    let sent = server.requests()[0].body.clone();
    assert!(sent.contains("[Retracted by its author: That was wartime propaganda.]"), "the agent reads the mark: {sent}");
    let locked = locked_post(&mut state);
    let at = state.posts.iter().position(|p| p.id == locked).expect("locked");
    state.posts[at].extras.correction = state.get_post(&claim).and_then(|p| p.extras.correction.clone());
    assert!(!state.get_post(&locked).expect("post").agent_text().contains("Retracted"), "a locked post is its placeholder alone");
}
