//! N20: a post keeps its translation (asked of the AI through the post actions runner,
//! against the local mock server only) and lets it go; a locked post is never translated.

use super::locked::locked_post;
use crate::error::AppError;
use crate::state::store::AppState;
use jiff::Timestamp;

#[test]
fn a_translation_comes_back_through_the_runner_and_is_kept() {
    use crate::agent::mock_server::{MockReply, MockServer};
    use crate::agent::post_actions::{run_action_blocking, ActionJob, CancelToken};
    use crate::agent::{LlmClient, LlmProvider, LlmSettings};
    use crate::domain::post::translation::prompt::translation_prompt;
    use crate::domain::post_actions::RequestPolicy;
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("The cat sleeps on the warm roof.").expect("posted");
    let server = MockServer::start(vec![MockReply::json(serde_json::json!({ "choices": [{ "message": { "role": "assistant", "content": "El gato duerme en el tejado caliente." } }] }))]);
    let settings = LlmSettings { provider: LlmProvider::OpenAi, api_key: "test-key-not-real".into(), model: "mock-model".into(), ..LlmSettings::default() };
    let timeout = std::time::Duration::from_secs(5);
    let prompt = translation_prompt(state.get_post(&id).expect("post"), "Spanish");
    let job = ActionJob { client: LlmClient::new(settings).with_endpoint(&server.url).with_timeout(timeout), prompt, policy: RequestPolicy { timeout, max_retries: 0 }, cancel: CancelToken::new() };
    let answer = run_action_blocking(&job).expect("answer");
    assert!(server.requests()[0].body.contains("warm roof"), "the post is sent as data");
    let kept = state.keep_translation(&id, "es", &answer, Timestamp::now()).expect("kept");
    assert_eq!((kept.language.as_str(), kept.text.as_str()), ("es", "El gato duerme en el tejado caliente."));
    assert_eq!(state.get_post(&id).expect("post").content, "The cat sleeps on the warm roof.", "the original stays");
    state.drop_translation(&id).expect("dropped");
    assert!(state.get_post(&id).is_some_and(|post| post.extras.translation.is_none()));
}

#[test]
fn a_locked_post_is_never_translated() {
    let mut state = AppState::new_in_memory();
    let locked = locked_post(&mut state);
    assert_eq!(state.keep_translation(&locked, "es", "Texto secreto.", Timestamp::now()).err(), Some(AppError::PostLocked));
    assert!(state.get_post(&locked).is_some_and(|post| post.extras.translation.is_none()));
}
