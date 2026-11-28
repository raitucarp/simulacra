//! Tier A, text: plain chat with the app defaults, native structured output for every
//! schema preset, and a full engine job of every kind.

use super::fixtures::{job, parent, plan, seed, PARENT_ID};
use super::harness::{block_on, ensure, pass, quote, run, Key, Live, Model, Outcome, Tier};
use simulacra::agent::engine::{DialecticEngine, JobKind};
use simulacra::agent::runtime::run_engine;
use simulacra::domain::engine::{EngineConfig, OutputMode, SchemaPreset, StructuredOutput};
use simulacra::domain::post::lens::Lens;

const PRESETS: [SchemaPreset; 4] = [SchemaPreset::Post, SchemaPreset::Illustrated, SchemaPreset::Thread, SchemaPreset::Expressive];

fn structured(preset: SchemaPreset) -> EngineConfig {
    EngineConfig { output: OutputMode::JsonSchema(StructuredOutput::from_preset(preset)), ..EngineConfig::default() }
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a01_plain_chat_with_default_budget() {
    run(Tier::A, "plain chat (default max tokens)", Model::Text, Key::Needed, |live| {
        let text = block_on(live.client().generate("You answer with one word.", "Reply with exactly the word: pong"))?;
        ensure(text.to_lowercase().contains("pong"), || format!("unexpected reply {}", quote(&text)))?;
        pass(format!("max_tokens {} answered {}", live.settings().max_tokens, quote(&text)))
    });
}

/// One `run_engine` per preset; every preset must map onto a non-empty post.
fn every_preset(live: &Live) -> Outcome {
    let mut notes = Vec::new();
    for preset in PRESETS {
        let plan = plan(live, &structured(preset));
        let result = block_on(run_engine(&plan, "You are a terse philosopher who posts on a social network.", "Post one thought about lenses.", None, 0))
            .map_err(|e| format!("{}: {e}", preset.key()))?;
        ensure(!result.draft.content.trim().is_empty(), || format!("{}: empty post", preset.key()))?;
        let extra = match preset {
            SchemaPreset::Thread => format!("{} parts", result.draft.thread_parts.len()),
            SchemaPreset::Illustrated => format!("image prompt {}", result.draft.image_prompt.is_some()),
            SchemaPreset::Expressive => format!("mood {:?}", result.draft.mood.unwrap_or_default()),
            _ => format!("{} chars", result.draft.content.chars().count()),
        };
        notes.push(format!("{}: {extra}", preset.key()));
    }
    pass(notes.join("; "))
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a02_native_structured_output_every_preset() {
    run(Tier::A, "structured output (responseSchema, 4 presets)", Model::Text, Key::Needed, every_preset);
}

/// Runs a job of `kind` and checks it was not the heuristic fallback.
fn job_case(live: &Live, kind: JobKind, engine: &EngineConfig) -> Outcome {
    let is_reply = kind.parent().is_some();
    let outcome = DialecticEngine::run_engine_job(job(live, kind, engine));
    if let Some(failure) = outcome.llm_error {
        return Err(format!("fell back to the heuristic post: {}", failure.message));
    }
    ensure(!outcome.reply.content.trim().is_empty(), || "empty post".to_string())?;
    ensure(is_reply == (outcome.reply.reply_to_id.as_ref().map(|id| id.as_str()) == Some(PARENT_ID)), || "wrong reply target".to_string())?;
    pass(format!("{} thread parts, post {}", outcome.thread.len(), quote(&outcome.reply.content)))
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a03_engine_job_reply() {
    run(Tier::A, "engine job: reply", Model::Text, Key::Needed, |live| job_case(live, JobKind::Reply(parent()), &EngineConfig::default()));
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a04_engine_job_new_post_thread() {
    run(Tier::A, "engine job: new post (thread schema)", Model::Text, Key::Needed, |live| job_case(live, JobKind::NewPost(seed()), &structured(SchemaPreset::Thread)));
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a05_engine_job_progress() {
    run(Tier::A, "engine job: to-do progress", Model::Text, Key::Needed, |live| {
        job_case(live, JobKind::Progress { parent: parent(), item: "write notes on the Ethics, part two".into() }, &EngineConfig::default())
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a06_engine_job_lens() {
    run(Tier::A, "engine job: lens reply", Model::Text, Key::Needed, |live| job_case(live, JobKind::Lens { parent: parent(), lens: Lens::Ethics }, &EngineConfig::default()));
}
