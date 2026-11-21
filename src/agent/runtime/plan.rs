//! Resolving an agent's [`EngineConfig`] against the global settings and the credential
//! store into an [`EnginePlan`] that can move to the generation thread.

use super::transport::{CustomTransport, Transport};
use crate::agent::credentials::{Credential, CredentialKind, CredentialStore};
use crate::agent::image::ImageJob;
use crate::agent::llm::{LlmClient, LlmProvider, LlmSettings};
use crate::domain::image_prompt::ImageSubject;
use crate::domain::engine::{CustomRequest, EngineConfig, ImageEngine, ImageEngineConfig, RequestMode, StructuredOutput, TextModel, ToolGrant};
use std::path::PathBuf;

/// Failure-gate key of agents on the global default model.
pub const GLOBAL_GATE_KEY: &str = "global";
const PROVIDER_GATE_PREFIX: &str = "provider:";
const CUSTOM_GATE_PREFIX: &str = "custom:";

/// Everything one generation needs, resolved and owned.
pub struct EnginePlan {
    pub transport: Transport,
    /// Backoff / notice key: the credential (or provider, or custom URL) behind the model.
    pub gate_key: String,
    pub structured: Option<StructuredOutput>,
    pub tools: Vec<ToolGrant>,
    pub max_steps: u8,
    pub image: Option<ImageJob>,
    /// The agent as an image subject, for posts that come without an image prompt (sb D3).
    pub image_subject: ImageSubject,
    /// The store (tools resolve their credentials; every secret is redacted from output).
    pub credentials: CredentialStore,
}

impl EnginePlan {
    /// The classic plan: `client`, free text, no tools, no image.
    pub fn from_client(client: LlmClient) -> Self {
        Self {
            transport: Transport::Native(client),
            gate_key: GLOBAL_GATE_KEY.to_string(),
            structured: None,
            tools: Vec::new(),
            max_steps: EngineConfig::default().tool_steps(),
            image: None,
            image_subject: ImageSubject::default(),
            credentials: CredentialStore::default(),
        }
    }

    /// Resolves `engine`; images are saved under `media_dir`.
    pub fn resolve(engine: &EngineConfig, global: &LlmSettings, credentials: &CredentialStore, media_dir: PathBuf) -> Self {
        let (settings, endpoint, gate_key) = text_settings(&engine.text, global, credentials);
        let structured = engine.structured().map(|output| output.with_tint_field().with_reactions_field()); // sb C4, R8
        let (transport, gate_key) = match &engine.request {
            RequestMode::Native => (Transport::Native(LlmClient::new(settings).with_endpoint(&endpoint)), gate_key),
            RequestMode::Custom(request) => {
                let cred = request.credential.as_deref().and_then(|id| credentials.get(id));
                let secret = cred.map(|c| c.secret.trim().to_string()).unwrap_or_default();
                let key = custom_gate_key(request, cred);
                let schema = structured.as_ref().and_then(StructuredOutput::schema_value);
                (Transport::Custom(CustomTransport::new(request.clone(), secret, settings, schema)), key)
            }
        };
        Self {
            transport,
            gate_key,
            structured,
            tools: engine.tools.clone(),
            max_steps: engine.tool_steps(),
            image: image_job(engine, credentials, media_dir),
            image_subject: ImageSubject::default(),
            credentials: credentials.clone(),
        }
    }

    /// Whether a request can be sent at all (a key, or a custom URL).
    pub fn is_ready(&self) -> bool {
        self.transport.is_ready()
    }

    /// `(gate key, ready)` of `engine` without building a client (for the scheduler).
    pub fn gate_of(engine: &EngineConfig, global: &LlmSettings, credentials: &CredentialStore) -> (String, bool) {
        let (settings, _, key) = text_settings(&engine.text, global, credentials);
        match &engine.request {
            RequestMode::Native => (key, settings.has_api_key()),
            RequestMode::Custom(request) => {
                let cred = request.credential.as_deref().and_then(|id| credentials.get(id));
                (custom_gate_key(request, cred), !request.url.trim().is_empty())
            }
        }
    }
}

/// The custom request's credential id, else its URL.
fn custom_gate_key(request: &CustomRequest, cred: Option<&Credential>) -> String {
    cred.map_or_else(|| format!("{CUSTOM_GATE_PREFIX}{}", request.url.trim()), |c| c.id.clone())
}

/// `(settings, endpoint override, gate key)` of the text model.
fn text_settings(text: &TextModel, global: &LlmSettings, credentials: &CredentialStore) -> (LlmSettings, String, String) {
    let TextModel::Own(own) = text else {
        return (global.clone(), String::new(), GLOBAL_GATE_KEY.to_string());
    };
    let cred = credentials.resolve(own.credential.as_deref(), CredentialKind::from_provider(own.provider));
    let cred_endpoint = cred.map(|c| c.endpoint.trim().to_string()).unwrap_or_default();
    let endpoint = if own.endpoint.trim().is_empty() { cred_endpoint.clone() } else { own.endpoint.trim().to_string() };
    let settings = LlmSettings {
        provider: own.provider,
        api_key: cred.map(|c| c.secret.trim().to_string()).unwrap_or_default(),
        model: own.model_or_default().to_string(),
        custom_endpoint: endpoint.clone(),
        temperature: own.temperature,
        max_tokens: own.max_tokens,
    };
    let gate_key = cred.map_or_else(|| format!("{PROVIDER_GATE_PREFIX}{}", own.provider.id_str()), |c| c.id.clone());
    let endpoint = if own.provider == LlmProvider::Custom { String::new() } else { endpoint };
    (settings, endpoint, gate_key)
}

/// The image job, when an engine is configured and can fire.
fn image_job(engine: &EngineConfig, credentials: &CredentialStore, media_dir: PathBuf) -> Option<ImageJob> {
    engine.image.is_enabled().then(|| resolve_image_job(&engine.image, credentials, media_dir))
}

/// The job of any image engine config, its credential resolved (sb D3: also the image ideas helper).
pub fn resolve_image_job(config: &ImageEngineConfig, credentials: &CredentialStore, media_dir: PathBuf) -> ImageJob {
    let kind = match config.engine {
        ImageEngine::OpenAi => CredentialKind::OpenAi,
        ImageEngine::Google => CredentialKind::Gemini,
        _ => CredentialKind::Http,
    };
    let cred = match config.engine {
        ImageEngine::OpenAi | ImageEngine::Google => credentials.resolve(config.credential.as_deref(), kind),
        _ => config.credential.as_deref().and_then(|id| credentials.get(id)),
    };
    ImageJob {
        config: config.clone(),
        secret: cred.map(|c| c.secret.trim().to_string()).unwrap_or_default(),
        endpoint: cred.map(|c| c.endpoint.trim().to_string()).unwrap_or_default(),
        media_dir,
    }
}
