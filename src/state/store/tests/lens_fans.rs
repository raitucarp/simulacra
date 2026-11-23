//! N16: fanning a post out through lenses: only to agents that may read it, one lens each,
//! never a locked post; each line finds the reply written through its lens, and an agent asked
//! through a lens reads the lens in its prompt (against the local mock server only).

use super::locked::locked_post;
use crate::domain::post::lens::{assign, FanEntry, Lens};
use crate::domain::{PostId, ProfileId};
use crate::state::store::AppState;
use jiff::Timestamp;

fn with_agents() -> AppState {
    let mut state = AppState::new_in_memory();
    for (id, name, role) in [("ada", "Ada", "historian"), ("bo", "Bo", "ethicist")] {
        let mut agent = crate::domain::Profile::new_agent(id, name, &format!("@{id}"), role);
        agent.network_id = Some(state.active_network_id.clone());
        state.user_profiles.push(agent);
    }
    state
}

fn entry(agent: &str, lens: Lens) -> FanEntry {
    FanEntry { agent: ProfileId::new(agent), lens }
}

fn reply(state: &mut AppState, agent: &str, parent: &PostId, lens: Option<Lens>) {
    let mut reply = crate::domain::Post::create_reply(format!("post_{agent}_{}", lens.map_or("plain", Lens::key)), state.active_network_id.clone(), agent, "Seen through a lens.", parent.clone(), None, None).expect("reply");
    reply.extras.lens = lens;
    state.insert_reply_after_parent(reply).expect("stored");
}

#[test]
fn a_post_is_fanned_out_and_each_line_finds_its_lens_reply() {
    let mut state = with_agents();
    let id = state.submit_post("Should cities ban cars from their centres?").expect("posted");
    state.fan_out(&id, vec![entry("ada", Lens::History), entry("bo", Lens::Ethics)], Timestamp::now()).expect("fanned");
    reply(&mut state, "ada", &id, None);
    reply(&mut state, "bo", &id, Some(Lens::Ethics));
    let lines = state.fan_lines(&id);
    assert_eq!(lines.iter().map(|line| (line.lens, line.reply.is_some())).collect::<Vec<_>>(), vec![(Lens::History, false), (Lens::Ethics, true)], "a plain reply is not a lens reply");
}

#[test]
fn a_fan_never_reaches_outside_the_scope_nor_asks_an_agent_twice() {
    let mut state = with_agents();
    let id = state.submit_post("A thought for Ada only.").expect("posted");
    let only_ada = crate::domain::post::audience::Audience::Agents { agents: vec![ProfileId::new("ada")] };
    state.set_audience(&id, Some(only_ada)).expect("scoped");
    assert!(state.fan_out(&id, vec![entry("bo", Lens::Ethics)], Timestamp::now()).is_err(), "N8: Bo may not read it");
    assert!(state.fan_out(&id, vec![entry("ada", Lens::Ethics), entry("ada", Lens::Law)], Timestamp::now()).is_err(), "one lens per agent");
    assert!(state.fan_out(&id, Vec::new(), Timestamp::now()).is_err());
}

#[test]
fn a_locked_post_is_never_fanned_out_and_lists_no_fan() {
    let mut state = with_agents();
    let locked = locked_post(&mut state);
    assert_eq!(state.fan_out(&locked, vec![entry("ada", Lens::History)], Timestamp::now()).err(), Some(crate::error::AppError::PostLocked));
    let at = state.post_position(&locked).expect("locked");
    state.posts[at].extras.fan = Some(Box::new(crate::domain::post::lens::LensFan { entries: vec![entry("ada", Lens::History)], at: Timestamp::now() }));
    assert!(state.fan_lines(&locked).is_empty(), "a forged fan on a locked post lists nothing");
}

#[test]
fn an_agent_asked_through_a_lens_reads_it_and_its_reply_keeps_it() {
    use crate::agent::mock_server::{MockReply, MockServer};
    use crate::agent::runtime::EnginePlan;
    use crate::agent::{DialecticEngine, EngineJob, JobKind, LlmClient, LlmProvider, LlmSettings, PromptContext};
    let mut state = with_agents();
    let id = state.submit_post("Should cities ban cars from their centres?").expect("posted");
    let room: Vec<(ProfileId, String)> = state.user_profiles.iter().filter(|p| p.is_agent()).map(|p| (p.id.clone(), crate::state::store::agent_knowledge(p))).collect();
    let fan = assign(&[Lens::Ethics], &room);
    assert_eq!(fan[0].agent.as_str(), "bo", "the ethicist takes ethics");
    let server = MockServer::start(vec![MockReply::json(serde_json::json!({ "choices": [{ "message": { "role": "assistant", "content": "Whose streets are they?" } }] }))]);
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key-not-real".into(), model: "mock-model".into(), ..LlmSettings::default() };
    let client = LlmClient::new(settings).with_endpoint(&server.url).with_timeout(std::time::Duration::from_secs(5));
    let agent = state.user_profiles.iter().find(|p| p.id == fan[0].agent).expect("bo").clone();
    let parent = state.authored(state.get_post(&id).expect("post"));
    let kind = JobKind::Lens { parent, lens: Lens::Ethics };
    let outcome = DialecticEngine::run_engine_job(EngineJob { kind, agent, network: state.active_network_id.clone(), context: PromptContext::default(), plan: EnginePlan::from_client(client), tools: None, roll: 0 });
    assert!(outcome.llm_error.is_none(), "{:?}", outcome.llm_error);
    assert!(server.requests()[0].body.contains("Reply through the lens of ethics"), "the agent reads its lens");
    assert_eq!(outcome.reply.extras.lens, Some(Lens::Ethics));
}
