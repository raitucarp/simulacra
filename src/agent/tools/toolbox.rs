//! The tools granted to one generation: their model-facing specs and their dispatch.

use super::custom_http::{self, sanitize_tool_name, HttpTool};
use super::http::{client, truncate_chars};
use super::vault_search::{self, Bm25Index};
use super::web_search::{self, SearchApi};
use super::{network, web_fetch, ToolContext, ToolOutput, MAX_TOOL_RESULT_CHARS};
use crate::agent::credentials::{CredentialKind, CredentialStore};
use crate::agent::llm::{redact_secrets, ToolCall, ToolSpec};
use crate::domain::engine::{SearchBackend, ToolGrant, ToolKind};

const VAULT_SEARCH_DESCRIPTION: &str = "Search the network's Knowledge Vault documents (BM25 keyword search). Returns the best passages with their document ids.";
const READ_NETWORK_DESCRIPTION: &str = "Read the social network you post in: recent posts, someone's profile, a thread, or posts containing a phrase.";
const WEB_FETCH_DESCRIPTION: &str = "Fetch a public web page and return its text.";
const WEB_SEARCH_DESCRIPTION: &str = "Search the web. Returns titles, URLs and snippets.";

enum ActiveTool {
    VaultSearch,
    ReadNetwork,
    WebSearch(SearchApi),
    WebFetch,
    CustomHttp(HttpTool),
}

/// The granted tools plus everything they read.
pub struct ToolBox {
    tools: Vec<(String, ActiveTool)>,
    ctx: ToolContext,
    vault: Option<Bm25Index>,
    http: reqwest::Client,
    secrets: Vec<String>,
    /// The provider's own web search is enabled instead of a function tool.
    pub native_web_search: bool,
}

impl ToolBox {
    /// Activates `grants`. Provider-native web search is used when `native_search` says the
    /// provider has it; search APIs and custom tools without a usable credential are skipped.
    pub fn new(grants: &[ToolGrant], ctx: ToolContext, credentials: &CredentialStore, native_search: bool) -> Self {
        let mut tools: Vec<(String, ActiveTool)> = Vec::new();
        let mut native_web_search = false;
        for grant in grants {
            let active = match grant {
                ToolGrant::VaultSearch => Some(ActiveTool::VaultSearch),
                ToolGrant::ReadNetwork => Some(ActiveTool::ReadNetwork),
                ToolGrant::WebFetch => Some(ActiveTool::WebFetch),
                ToolGrant::WebSearch(cfg) if cfg.backend == SearchBackend::ProviderNative => {
                    native_web_search |= native_search;
                    None
                }
                ToolGrant::WebSearch(cfg) => search_api(cfg.backend, cfg.credential.as_deref(), credentials).map(ActiveTool::WebSearch),
                ToolGrant::CustomHttp(def) => Some(ActiveTool::CustomHttp(HttpTool {
                    name: sanitize_tool_name(&def.name),
                    secret: def.credential.as_deref().and_then(|id| credentials.get(id)).map(|c| c.secret.clone()).unwrap_or_default(),
                    def: def.clone(),
                })),
            };
            if let Some(tool) = active {
                let name = tool_name(&tool);
                if !tools.iter().any(|(existing, _)| existing == &name) {
                    tools.push((name, tool));
                }
            }
        }
        let vault = tools.iter().any(|(_, t)| matches!(t, ActiveTool::VaultSearch)).then(|| vault_search::index_documents(&ctx.documents));
        let http = client(&ctx.fetch);
        Self { tools, ctx, vault, http, secrets: credentials.secrets(), native_web_search }
    }

    /// Whether no function tool is active.
    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    /// Model-facing specs of every function tool.
    pub fn specs(&self) -> Vec<ToolSpec> {
        self.tools.iter().map(|(name, tool)| spec(name, tool)).collect()
    }

    /// Runs `call`; unknown tools and failures become error results. Every result is
    /// capped and has all stored secrets redacted before it reaches the model.
    pub async fn execute(&self, call: &ToolCall) -> ToolOutput {
        let Some((_, tool)) = self.tools.iter().find(|(name, _)| name == &call.name) else {
            return ToolOutput::error(format!("No tool named \"{}\" is available.", call.name));
        };
        let mut output = match tool {
            ActiveTool::VaultSearch => self.vault.as_ref().map_or_else(|| ToolOutput::error("The vault is empty."), |index| vault_search::run(index, &call.arguments)),
            ActiveTool::ReadNetwork => network::run(&self.ctx, &call.arguments),
            ActiveTool::WebFetch => web_fetch::run(&self.http, &self.ctx.fetch, &call.arguments).await,
            ActiveTool::WebSearch(api) => web_search::run(&self.http, api, &call.arguments).await,
            ActiveTool::CustomHttp(tool) => custom_http::run(&self.http, tool, &call.arguments).await,
        };
        output.content = truncate_chars(&redact_secrets(&output.content, &self.secrets), MAX_TOOL_RESULT_CHARS);
        output
    }
}

fn search_api(backend: SearchBackend, credential: Option<&str>, credentials: &CredentialStore) -> Option<SearchApi> {
    let kind = if backend == SearchBackend::Tavily { CredentialKind::Tavily } else { CredentialKind::Brave };
    let cred = credentials.resolve(credential, kind).filter(|c| c.has_secret())?;
    Some(SearchApi { backend, secret: cred.secret.trim().to_string(), endpoint: cred.endpoint.clone() })
}

fn tool_name(tool: &ActiveTool) -> String {
    match tool {
        ActiveTool::VaultSearch => ToolKind::VaultSearch.key().to_string(),
        ActiveTool::ReadNetwork => ToolKind::ReadNetwork.key().to_string(),
        ActiveTool::WebSearch(_) => ToolKind::WebSearch.key().to_string(),
        ActiveTool::WebFetch => ToolKind::WebFetch.key().to_string(),
        ActiveTool::CustomHttp(tool) => tool.name.clone(),
    }
}

fn spec(name: &str, tool: &ActiveTool) -> ToolSpec {
    let (description, parameters) = match tool {
        ActiveTool::VaultSearch => (VAULT_SEARCH_DESCRIPTION.to_string(), vault_search::parameters()),
        ActiveTool::ReadNetwork => (READ_NETWORK_DESCRIPTION.to_string(), network::parameters()),
        ActiveTool::WebSearch(_) => (WEB_SEARCH_DESCRIPTION.to_string(), web_search::parameters()),
        ActiveTool::WebFetch => (WEB_FETCH_DESCRIPTION.to_string(), web_fetch::parameters()),
        ActiveTool::CustomHttp(tool) => (tool.def.description.clone(), custom_http::parameters(&tool.def)),
    };
    ToolSpec { name: name.to_string(), description, parameters }
}
