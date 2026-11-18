//! sb E1: the thinking tools' AI requests through the runner, against the local mock server
//! only (no live provider, no real key): an argument extraction and a tension batch.

use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::post_actions::RequestPolicy;
use crate::domain::thinking::argument::{analyse, extraction_prompt, parse_extraction, EdgeKind, Standing, ThreadPost};
use crate::domain::thinking::tensions::{classification_prompt, parse_classifications, rank_candidates, CandidateInput, RankOptions, TensionSide, Verdict};
use crate::domain::PostId;
use serde_json::json;
use std::time::Duration;

fn job(url: &str, prompt: crate::domain::post_actions::ActionPrompt) -> ActionJob {
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key-not-real".into(), model: "mock-model".into(), ..LlmSettings::default() };
    let timeout = Duration::from_secs(5);
    ActionJob { client: LlmClient::new(settings).with_endpoint(url).with_timeout(timeout), prompt, policy: RequestPolicy { timeout, max_retries: 0 }, cancel: CancelToken::new() }
}

fn answer(text: &str) -> MockReply {
    MockReply::json(json!({ "choices": [{ "message": { "role": "assistant", "content": text } }] }))
}

#[test]
fn an_extraction_round_trip_yields_a_validated_map() {
    let posts = vec![
        ThreadPost { id: PostId::from("root"), author: "Ada".into(), text: "We should tax carbon.".into() },
        ThreadPost { id: PostId::from("reply"), author: "Bo".into(), text: "It hurts the poor.".into() },
    ];
    let reply = "```json\n{\"nodes\":[{\"id\":\"c\",\"role\":\"claim\",\"text\":\"Tax carbon\",\"post\":\"P1\"},\
        {\"id\":\"o\",\"role\":\"rebuttal\",\"text\":\"Hurts the poor\",\"post\":\"P2\"},\
        {\"id\":\"x\",\"role\":\"ground\",\"text\":\"Invented\",\"post\":\"P7\"}],\
        \"edges\":[{\"from\":\"o\",\"to\":\"c\",\"kind\":\"attack\"},{\"from\":\"x\",\"to\":\"c\",\"kind\":\"support\"}]}\n```";
    let server = MockServer::start(vec![answer(reply)]);
    let text = run_action_blocking(&job(&server.url, extraction_prompt(&posts, "English"))).expect("answer");
    let sent = server.requests()[0].json();
    assert!(sent["messages"][1]["content"].as_str().unwrap_or_default().contains("[P2] Bo: It hurts the poor."));
    let extraction = parse_extraction(&text, &posts).expect("a map");
    assert_eq!((extraction.map.nodes.len(), extraction.map.edges.len()), (2, 1));
    assert_eq!((extraction.dropped_nodes, extraction.dropped_edges), (1, 1));
    assert_eq!(extraction.map.edges[0].kind, EdgeKind::Attack);
    assert_eq!(analyse(&extraction.map).standing, vec![Standing::Rejected, Standing::Accepted]);
}

#[test]
fn a_tension_batch_round_trip_yields_verdicts_by_pair_key() {
    let inputs: Vec<CandidateInput> = [("a", "Remote work always improves focus."), ("b", "Remote work never improves focus.")]
        .into_iter()
        .map(|(id, text)| CandidateInput { side: TensionSide::post(id, "Ada", text), vector: None })
        .collect();
    let batch = rank_candidates(&inputs, &RankOptions::default());
    assert_eq!(batch.len(), 1, "the opposite quantifiers make a candidate");
    let server = MockServer::start(vec![answer("[{\"pair\": 1, \"verdict\": \"Contradiction\", \"reason\": \"Always and never cannot both hold.\"}]")]);
    let text = run_action_blocking(&job(&server.url, classification_prompt(&batch, "English"))).expect("answer");
    let verdicts = parse_classifications(&text, &batch);
    assert_eq!(verdicts.len(), 1);
    assert_eq!((verdicts[0].pair.as_str(), verdicts[0].verdict), (batch[0].key().as_str(), Verdict::Contradicts));
    assert!(server.requests()[0].json()["messages"][1]["content"].as_str().unwrap_or_default().contains("<pairs>"));
}
