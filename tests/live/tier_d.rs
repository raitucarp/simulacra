//! Tier D, real failures: an obviously fake key and an unknown model, classified the way the
//! app classifies them, with the key never surviving into a message. The fake-key cases need
//! no secret but still reach Google, so they stay ignored like the rest.

use super::fixtures::{job, parent};
use super::harness::{block_on, ensure, pass, run, Key, Live, Model, Tier};
use super::tier_a_embed::{config, embed};
use simulacra::agent::embedding::{EmbedInput, TaskType};
use simulacra::agent::engine::{DialecticEngine, JobKind};
use simulacra::agent::image::{self, ImageJob};
use simulacra::agent::llm::{redact_secrets, LlmClient, LlmSettings};
use simulacra::domain::engine::{EngineConfig, ImageEngine, ImageEngineConfig, ImageTrigger};
use simulacra::domain::post_actions::ActionFailureKind;

/// A key no provider issues; it must never appear in a described failure.
const FAKE_KEY: &str = "simulacra-live-fake-key-0000000000";
const UNKNOWN_MODEL: &str = "gemini-model-that-does-not-exist-0";
const REJECTED_STATUSES: [&str; 3] = ["400", "401", "403"];

fn fake_settings(live: &Live) -> LlmSettings {
    LlmSettings { api_key: FAKE_KEY.into(), ..live.settings() }
}

/// `raw` is an HTTP rejection the post-action runner would not retry, and its described
/// form names the status without the key.
fn rejected(client: &LlmClient, raw: &str, statuses: &[&str]) -> Result<String, String> {
    let message = client.describe_failure(raw).message;
    ensure(ActionFailureKind::classify(raw) == ActionFailureKind::Rejected, || format!("classified {:?}: {message}", ActionFailureKind::classify(raw)))?;
    ensure(!message.contains(FAKE_KEY), || "the fake key leaked into the message".to_string())?;
    ensure(statuses.iter().any(|status| message.contains(status)), || format!("no {statuses:?} status in {message}"))?;
    Ok(message)
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn d01_fake_key_is_rejected_and_redacted() {
    run(Tier::D, "fake key: rejected, key redacted", Model::Text, Key::NotNeeded, |live| {
        let client = LlmClient::new(fake_settings(live));
        let raw = block_on(client.generate("S", "Say hi.")).err().ok_or("a fake key was accepted")?;
        pass(rejected(&client, &raw, &REJECTED_STATUSES)?)
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn d02_unknown_model_is_rejected() {
    run(Tier::D, "unknown model: 404, rejected", Model::Fixed(UNKNOWN_MODEL), Key::Needed, |live| {
        let client = LlmClient::new(LlmSettings { model: UNKNOWN_MODEL.into(), ..live.settings() });
        let raw = block_on(client.generate("S", "Say hi.")).err().ok_or("an unknown model answered")?;
        pass(rejected(&client, &raw, &["404"])?)
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn d03_embedding_fake_key_is_a_bad_key() {
    run(Tier::D, "embeddings fake key: BadKey", Model::Embed, Key::NotNeeded, |live| {
        let error = embed(config(&live.embed_model, FAKE_KEY), TaskType::RetrievalQuery, &[EmbedInput::text("hi")]).err().ok_or("a fake key was accepted")?;
        ensure(error.starts_with("BadKey"), || format!("expected BadKey: {error}"))?;
        ensure(!error.contains(FAKE_KEY), || "the fake key leaked into the detail".to_string())?;
        pass(error)
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn d04_embedding_unknown_model_is_model_not_found() {
    run(Tier::D, "embeddings unknown model: ModelNotFound", Model::Fixed(UNKNOWN_MODEL), Key::Needed, |live| {
        let error = embed(config(UNKNOWN_MODEL, live.key()), TaskType::RetrievalQuery, &[EmbedInput::text("hi")]).err().ok_or("an unknown model answered")?;
        ensure(error.starts_with("ModelNotFound"), || format!("expected ModelNotFound: {error}"))?;
        pass(error)
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn d05_engine_job_falls_back_with_a_redacted_failure() {
    run(Tier::D, "engine job fake key: heuristic post, redacted notice", Model::Text, Key::NotNeeded, |live| {
        let mut job = job(live, JobKind::Reply(parent()), &EngineConfig::default());
        job.plan = simulacra::agent::runtime::EnginePlan::from_client(LlmClient::new(fake_settings(live)));
        let outcome = DialecticEngine::run_engine_job(job);
        let failure = outcome.llm_error.ok_or("a fake key produced a model post")?;
        ensure(!failure.message.contains(FAKE_KEY), || "the fake key leaked into the notice".to_string())?;
        ensure(!outcome.reply.content.trim().is_empty(), || "no heuristic post".to_string())?;
        pass(format!("notice: {}", failure.message))
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn d06_image_fake_key_is_rejected_and_redacted() {
    run(Tier::D, "image fake key: rejected, key redacted", Model::Image, Key::NotNeeded, |live| {
        let config = ImageEngineConfig { engine: ImageEngine::Google, model: live.image_model.clone(), trigger: ImageTrigger::Always, ..ImageEngineConfig::default() };
        let job = ImageJob { config, secret: FAKE_KEY.into(), endpoint: String::new(), media_dir: super::report::media_dir() };
        let raw = block_on(image::generate(&job, "a fox")).err().ok_or("a fake key made an image")?;
        let shown = redact_secrets(&raw, &[FAKE_KEY.to_string()]);
        ensure(REJECTED_STATUSES.iter().any(|status| shown.contains(status)), || format!("no rejection status in {shown}"))?;
        ensure(!format!("{job:?}").contains(FAKE_KEY), || "ImageJob Debug printed the key".to_string())?;
        pass(shown)
    });
}
