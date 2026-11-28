//! Configuration from the environment and the runner every live case goes through: tier
//! selection, the key gate, timing, panic capture, redaction and the report row.

use super::report::{self, Row, Status};
use simulacra::agent::llm::{redact_secrets, LlmClient, LlmProvider, LlmSettings};
use std::future::Future;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Once;
use std::time::Instant;

/// The Gemini API key (required by tiers A–C; read here only, never printed).
pub const KEY_VAR: &str = "GEMINI_API_KEY";
/// Text model id (default: the app's Gemini default).
pub const MODEL_VAR: &str = "SIMULACRA_LIVE_MODEL";
/// Image model id for tier C.
pub const IMAGE_MODEL_VAR: &str = "SIMULACRA_LIVE_IMAGE_MODEL";
/// Embedding model id.
pub const EMBED_MODEL_VAR: &str = "SIMULACRA_LIVE_EMBED_MODEL";
/// Comma-separated tiers to run (`a,b,c,d` or `all`).
pub const TIERS_VAR: &str = "SIMULACRA_LIVE_TIERS";
/// Path of an audio file for the optional transcription case.
pub const AUDIO_VAR: &str = "SIMULACRA_LIVE_AUDIO";
/// Tier C (media) costs more, so it runs only when asked for.
const DEFAULT_TIERS: &str = "a,b,d";
const ALL_TIERS: &str = "all";
pub const DEFAULT_IMAGE_MODEL: &str = "gemini-nano-banana-2.1";
pub const DEFAULT_EMBED_MODEL: &str = "gemini-embedding-001";
/// The Gemini API `models` collection and its OpenAI-compatible chat endpoint.
pub const GEMINI_MODELS_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
pub const GEMINI_OPENAI_URL: &str = "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions";
/// Characters of a model reply quoted in a note.
const QUOTE_CHARS: usize = 70;

/// A tier of the suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    A,
    B,
    C,
    D,
}

impl Tier {
    pub const fn letter(self) -> char {
        match self {
            Self::A => 'a',
            Self::B => 'b',
            Self::C => 'c',
            Self::D => 'd',
        }
    }
}

/// Which model a case reports in its row.
#[derive(Debug, Clone, Copy)]
pub enum Model {
    Text,
    Image,
    Embed,
    Fixed(&'static str),
}

/// Whether a case needs the real key (tier D cases with a fake key do not).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Needed,
    NotNeeded,
}

/// How a case ended without failing.
pub enum Done {
    Pass(String),
    Skip(String),
}

/// A case's result: `Err` is the (unredacted) reason it failed; the runner redacts it.
pub type Outcome = Result<Done, String>;

pub fn pass(note: impl Into<String>) -> Outcome {
    Ok(Done::Pass(note.into()))
}

pub fn skip(note: impl Into<String>) -> Outcome {
    Ok(Done::Skip(note.into()))
}

/// `Err(why())` unless `condition` holds.
pub fn ensure(condition: bool, why: impl FnOnce() -> String) -> Result<(), String> {
    if condition { Ok(()) } else { Err(why()) }
}

/// The first characters of `text` on one line, for notes.
pub fn quote(text: &str) -> String {
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut short: String = line.chars().take(QUOTE_CHARS).collect();
    if line.chars().count() > QUOTE_CHARS {
        short.push('…');
    }
    format!("\"{short}\"")
}

/// Runs `future` on a private current-thread runtime (the app's blocking entry points build
/// their own, so cases never run inside one).
pub fn block_on<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread().enable_all().build().expect("tokio runtime").block_on(future)
}

/// The suite's configuration. The key is private and has no `Debug`.
pub struct Live {
    key: Option<String>,
    pub model: String,
    pub image_model: String,
    pub embed_model: String,
    tiers: String,
}

fn env_or(name: &str, default: &str) -> String {
    std::env::var(name).ok().map(|value| value.trim().to_string()).filter(|value| !value.is_empty()).unwrap_or_else(|| default.to_string())
}

impl Live {
    pub fn from_env() -> Self {
        Self {
            key: std::env::var(KEY_VAR).ok().map(|key| key.trim().to_string()).filter(|key| !key.is_empty()),
            model: env_or(MODEL_VAR, LlmProvider::Gemini.default_model()),
            image_model: env_or(IMAGE_MODEL_VAR, DEFAULT_IMAGE_MODEL),
            embed_model: env_or(EMBED_MODEL_VAR, DEFAULT_EMBED_MODEL),
            tiers: env_or(TIERS_VAR, DEFAULT_TIERS).to_lowercase(),
        }
    }

    /// The key for building in-memory settings and stores (empty when unset).
    pub fn key(&self) -> &str {
        self.key.as_deref().unwrap_or_default()
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    pub fn tiers(&self) -> &str {
        &self.tiers
    }

    fn selected(&self, tier: Tier) -> bool {
        let letter = tier.letter().to_string();
        self.tiers.split(',').map(str::trim).any(|part| part == ALL_TIERS || part == letter)
    }

    fn model_name(&self, model: Model) -> String {
        match model {
            Model::Text => self.model.clone(),
            Model::Image => self.image_model.clone(),
            Model::Embed => self.embed_model.clone(),
            Model::Fixed(name) => name.to_string(),
        }
    }

    /// Gemini settings for the text model with the app defaults (temperature, max tokens).
    pub fn settings(&self) -> LlmSettings {
        LlmSettings { provider: LlmProvider::Gemini, api_key: self.key().to_string(), model: self.model.clone(), ..LlmSettings::default() }
    }

    pub fn client(&self) -> LlmClient {
        LlmClient::new(self.settings())
    }

    /// `text` with the key (and any `key=` query value) replaced.
    pub fn redact(&self, text: &str) -> String {
        redact_secrets(text, &self.key.iter().cloned().collect::<Vec<_>>())
    }
}

/// Prints panics with every secret redacted (the default hook would print them raw).
fn install_redacting_panic_hook() {
    static HOOK: Once = Once::new();
    HOOK.call_once(|| {
        let live = Live::from_env();
        panic::set_hook(Box::new(move |info| {
            let message = info.payload().downcast_ref::<String>().cloned().or_else(|| info.payload().downcast_ref::<&str>().map(|s| s.to_string()));
            let place = info.location().map(|l| format!("{}:{}", l.file(), l.line())).unwrap_or_default();
            eprintln!("[live] panic at {place}: {}", live.redact(&message.unwrap_or_default()));
        }));
    });
}

/// Runs one case and records its row; a failed case panics (with a redacted message) after
/// its row is written, so `cargo test` reports it too.
pub fn run(tier: Tier, name: &str, model: Model, key: Key, case: impl FnOnce(&Live) -> Outcome) {
    install_redacting_panic_hook();
    let live = Live::from_env();
    let model = live.model_name(model);
    let row = |status, latency_ms, note: String| Row { tier: tier.letter(), name: name.to_string(), status, latency_ms, model: model.clone(), note: live.redact(&note) };
    if !live.selected(tier) {
        return report::record(row(Status::Skip, 0, format!("tier {} not in {TIERS_VAR}={}", tier.letter(), live.tiers())));
    }
    if key == Key::Needed && !live.has_key() {
        return report::record(row(Status::Skip, 0, format!("skipped: {KEY_VAR} not set")));
    }
    let started = Instant::now();
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| case(&live)));
    let latency_ms = started.elapsed().as_millis();
    let (status, note) = match outcome {
        Ok(Ok(Done::Pass(note))) => (Status::Pass, note),
        Ok(Ok(Done::Skip(note))) => (Status::Skip, note),
        Ok(Err(why)) => (Status::Fail, why),
        Err(_) => (Status::Fail, "the case panicked (see the redacted panic line above)".to_string()),
    };
    let row = row(status, latency_ms, note);
    let failure = (status == Status::Fail).then(|| row.note.clone());
    report::record(row);
    if let Some(note) = failure {
        panic!("{name} failed: {note}");
    }
}
