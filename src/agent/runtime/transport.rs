//! Where a [`ChatRequest`] goes: the provider's native API or a custom raw request.

use crate::agent::custom_request;
use crate::agent::llm::wire::openai;
use crate::agent::llm::{redact_secrets, ChatReply, ChatRequest, LlmClient, LlmFailure, LlmProvider, LlmSettings, Role, SchemaStrategy};
use crate::domain::engine::{value_at, value_text, CustomRequest};
use serde_json::Value;
use std::time::Duration;

/// Timeout of one custom text request.
const CUSTOM_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const EMPTY_CUSTOM_REPLY: &str = "The custom reply has no text at its reply path and no tool calls";

/// A user-built raw request plus what fills its placeholders.
pub struct CustomTransport {
    pub request: CustomRequest,
    pub secret: String,
    pub settings: LlmSettings,
    /// The structured-output schema, available to the template as `{{schema}}`.
    pub schema: Option<Value>,
    http: reqwest::Client,
}

impl CustomTransport {
    pub fn new(request: CustomRequest, secret: String, settings: LlmSettings, schema: Option<Value>) -> Self {
        let http = reqwest::Client::builder().timeout(CUSTOM_REQUEST_TIMEOUT).build().unwrap_or_default();
        Self { request, secret, settings, schema, http }
    }

    /// The template variables for `req`.
    pub fn variables(&self, req: &ChatRequest) -> Vec<(&'static str, Value)> {
        let last_user = req.messages.iter().rev().find(|m| m.role == Role::User).map(|m| m.content.clone()).unwrap_or_default();
        vec![
            ("system", Value::String(req.system_text())),
            ("user", Value::String(last_user)),
            ("messages", openai::messages(&req.messages)),
            ("schema", self.schema.clone().unwrap_or(Value::Null)),
            ("model", Value::String(self.settings.model.clone())),
            ("tools", openai::tools(&req.tools)),
        ]
    }

    /// Reads the reply text and tool calls at the configured paths.
    pub fn parse(&self, reply: &Value) -> Result<ChatReply, String> {
        let text = value_text(reply, &self.request.reply_path).unwrap_or_default().trim().to_string();
        let tool_calls = value_at(reply, &self.request.tool_call_path).map(openai::parse_tool_calls).unwrap_or_default();
        let parsed = ChatReply { text, tool_calls, ..ChatReply::default() };
        if parsed.is_empty() {
            return Err(EMPTY_CUSTOM_REPLY.to_string());
        }
        Ok(parsed)
    }

    async fn send(&self, req: &ChatRequest) -> Result<ChatReply, String> {
        let vars = self.variables(req);
        let reply = custom_request::send(&self.http, &self.request, &self.secret, &self.settings.model, &vars).await?;
        self.parse(&reply)
    }
}

/// The text model endpoint of one generation.
pub enum Transport {
    Native(LlmClient),
    Custom(CustomTransport),
}

impl Transport {
    /// Sends one request.
    pub async fn send(&self, req: &ChatRequest) -> Result<ChatReply, String> {
        match self {
            Self::Native(client) => client.chat(req).await,
            Self::Custom(custom) => custom.send(req).await,
        }
    }

    /// The provider failures are reported under.
    pub fn provider(&self) -> LlmProvider {
        match self {
            Self::Native(client) => client.provider(),
            Self::Custom(_) => LlmProvider::Custom,
        }
    }

    /// Whether the transport can be used (native providers need a key; custom requests
    /// carry their own authentication).
    pub fn is_ready(&self) -> bool {
        match self {
            Self::Native(client) => client.has_api_key(),
            Self::Custom(custom) => !custom.request.url.trim().is_empty(),
        }
    }

    /// Whether the provider's own web search can be switched on.
    pub fn native_web_search(&self) -> bool {
        matches!(self, Self::Native(client) if client.provider().native_web_search())
    }

    /// How structured output is requested.
    pub fn schema_strategy(&self, with_tools: bool) -> SchemaStrategy {
        match self {
            Self::Native(client) => client.provider().schema_strategy(with_tools),
            Self::Custom(_) => SchemaStrategy::Instruct,
        }
    }

    /// A display-safe failure with every secret in `secrets` redacted.
    pub fn describe_failure(&self, raw: &str, secrets: &[String]) -> LlmFailure {
        let redacted = redact_secrets(raw, secrets);
        match self {
            Self::Native(client) => client.describe_failure(&redacted),
            Self::Custom(custom) => LlmFailure::new(LlmProvider::Custom, &redacted, &custom.secret, !self.is_ready()),
        }
    }
}
