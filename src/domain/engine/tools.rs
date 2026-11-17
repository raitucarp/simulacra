//! Tools an agent may call while composing a post.

use super::request::{HeaderPair, HttpMethod};
use serde::{Deserialize, Serialize};

/// Default JSON-schema of a custom tool's arguments: one free-text `query`.
const DEFAULT_PARAMS_SCHEMA: &str =
    r#"{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}"#;

/// One granted tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "tool", rename_all = "snake_case")]
pub enum ToolGrant {
    /// Lexical (BM25) search over the network's Knowledge Vault documents.
    VaultSearch,
    /// Read profiles, recent posts and threads of the agent's network.
    ReadNetwork,
    /// Search the web (provider-native search or a search API).
    WebSearch(WebSearchConfig),
    /// Fetch one web page as text (size and time capped).
    WebFetch,
    /// A user-defined HTTP tool.
    CustomHttp(HttpToolDef),
}

/// Variant keys of [`ToolGrant`], for pickers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolKind {
    VaultSearch,
    ReadNetwork,
    WebSearch,
    WebFetch,
    CustomHttp,
}

impl ToolKind {
    pub const ALL: [ToolKind; 5] = [Self::VaultSearch, Self::ReadNetwork, Self::WebSearch, Self::WebFetch, Self::CustomHttp];

    /// Stable key, also the model-facing tool name of the built-ins.
    pub const fn key(self) -> &'static str {
        match self {
            Self::VaultSearch => "vault_search",
            Self::ReadNetwork => "read_network",
            Self::WebSearch => "web_search",
            Self::WebFetch => "web_fetch",
            Self::CustomHttp => "custom_http",
        }
    }
}

impl ToolGrant {
    /// The variant's kind.
    pub fn kind(&self) -> ToolKind {
        match self {
            Self::VaultSearch => ToolKind::VaultSearch,
            Self::ReadNetwork => ToolKind::ReadNetwork,
            Self::WebSearch(_) => ToolKind::WebSearch,
            Self::WebFetch => ToolKind::WebFetch,
            Self::CustomHttp(_) => ToolKind::CustomHttp,
        }
    }

    /// The referenced credential id, if any.
    pub fn credential(&self) -> Option<&String> {
        match self {
            Self::WebSearch(search) => search.credential.as_ref(),
            Self::CustomHttp(def) => def.credential.as_ref(),
            _ => None,
        }
    }
}

/// Where web searches go.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SearchBackend {
    /// The text provider's own search (Gemini grounding, Anthropic web search, OpenAI
    /// search models); unavailable on other providers.
    #[default]
    ProviderNative,
    /// Brave Search API (credential kind `brave`).
    Brave,
    /// Tavily Search API (credential kind `tavily`).
    Tavily,
}

impl SearchBackend {
    pub const ALL: [SearchBackend; 3] = [Self::ProviderNative, Self::Brave, Self::Tavily];

    /// Stable key (`provider_native`, `brave`, `tavily`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::ProviderNative => "provider_native",
            Self::Brave => "brave",
            Self::Tavily => "tavily",
        }
    }
}

/// Web search settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct WebSearchConfig {
    pub backend: SearchBackend,
    /// Credential of the search API (`None` = the first of its kind).
    pub credential: Option<String>,
}

/// A user-defined HTTP tool. `url_template` and header values may use `{{arg}}` for any
/// argument (URL-encoded) and `{{api_key}}`; GET/DELETE send unused arguments as query
/// parameters, other verbs send all arguments as a JSON body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct HttpToolDef {
    /// Model-facing name (`[A-Za-z0-9_-]`, sanitized at runtime).
    pub name: String,
    pub description: String,
    pub method: HttpMethod,
    pub url_template: String,
    pub headers: Vec<HeaderPair>,
    /// JSON schema of the arguments.
    pub params_schema: String,
    pub credential: Option<String>,
}

impl Default for HttpToolDef {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            method: HttpMethod::Get,
            url_template: String::new(),
            headers: Vec::new(),
            params_schema: DEFAULT_PARAMS_SCHEMA.to_string(),
            credential: None,
        }
    }
}
