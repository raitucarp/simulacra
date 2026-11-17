//! How the text request is sent: the provider's native API or a custom raw request.

use serde::{Deserialize, Serialize};

/// Placeholders a custom body template may use. Each is replaced by a JSON value
/// (strings arrive quoted and escaped), so write `"prompt": {{user}}`, not `"{{user}}"`.
pub const REQUEST_PLACEHOLDERS: [&str; 6] =
    ["{{system}}", "{{user}}", "{{messages}}", "{{schema}}", "{{model}}", "{{tools}}"];
/// Placeholder for the referenced credential's secret; only expanded in URLs and headers.
pub const API_KEY_PLACEHOLDER: &str = "{{api_key}}";

/// Default custom body: an OpenAI-compatible chat completion.
const DEFAULT_BODY_TEMPLATE: &str = r#"{"model": {{model}}, "messages": {{messages}}, "tools": {{tools}}}"#;
/// Default dotted path of the reply text.
const DEFAULT_REPLY_PATH: &str = "choices.0.message.content";
/// Default dotted path of the tool-call array.
const DEFAULT_TOOL_CALL_PATH: &str = "choices.0.message.tool_calls";
/// Default authorization header of a custom request.
const AUTHORIZATION_HEADER: &str = "Authorization";
const BEARER_VALUE: &str = "Bearer {{api_key}}";

/// The request mode of the text model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum RequestMode {
    /// The provider's own wire format (tools, schemas and search where supported).
    #[default]
    Native,
    /// A user-built raw HTTP request.
    Custom(CustomRequest),
}

impl RequestMode {
    /// Stable key of the variant (`native` / `custom`).
    pub fn key(&self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Custom(_) => "custom",
        }
    }
}

/// HTTP verbs available to custom requests and tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    #[default]
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    pub const ALL: [HttpMethod; 5] = [Self::Get, Self::Post, Self::Put, Self::Patch, Self::Delete];

    /// The verb as sent on the wire.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        }
    }

    /// Whether requests with this verb carry a JSON body.
    pub const fn has_body(self) -> bool {
        matches!(self, Self::Post | Self::Put | Self::Patch)
    }
}

/// One request header; `value` may contain [`API_KEY_PLACEHOLDER`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct HeaderPair {
    pub name: String,
    pub value: String,
}

impl HeaderPair {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into() }
    }
}

/// A raw request template. The reply text is read at `reply_path` and tool calls (an
/// OpenAI-style array) at `tool_call_path`; both are dotted paths such as
/// `choices.0.message.content`. Image engines read a base64 string or URL at `reply_path`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct CustomRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<HeaderPair>,
    pub body_template: String,
    pub reply_path: String,
    pub tool_call_path: String,
    /// Credential whose secret fills `{{api_key}}`.
    pub credential: Option<String>,
}

impl Default for CustomRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Post,
            url: String::new(),
            headers: vec![HeaderPair::new(AUTHORIZATION_HEADER, BEARER_VALUE)],
            body_template: DEFAULT_BODY_TEMPLATE.to_string(),
            reply_path: DEFAULT_REPLY_PATH.to_string(),
            tool_call_path: DEFAULT_TOOL_CALL_PATH.to_string(),
            credential: None,
        }
    }
}
