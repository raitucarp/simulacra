//! `web_search` through a search API (Brave or Tavily) when the provider has no native search.

use super::http::read_capped;
use super::ToolOutput;
use crate::domain::engine::SearchBackend;
use crate::domain::post::{SourceKind, SourceRef};
use serde_json::{json, Value};

const BRAVE_URL: &str = "https://api.search.brave.com/res/v1/web/search";
const BRAVE_TOKEN_HEADER: &str = "X-Subscription-Token";
const TAVILY_URL: &str = "https://api.tavily.com/search";
/// Results requested per search.
const RESULT_COUNT: u32 = 5;
/// Largest search response read.
const MAX_RESPONSE_BYTES: usize = 256 * 1024;

/// A configured search API.
#[derive(Debug, Clone)]
pub struct SearchApi {
    pub backend: SearchBackend,
    pub secret: String,
    /// Replaces the API URL when non-blank.
    pub endpoint: String,
}

/// The tool's argument schema.
pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": { "query": { "type": "string", "description": "What to search the web for." } },
        "required": ["query"]
    })
}

/// Runs a search; results as `title — url — snippet` lines, each URL a source.
pub async fn run(http: &reqwest::Client, api: &SearchApi, args: &Value) -> ToolOutput {
    let query = args["query"].as_str().unwrap_or_default().trim();
    if query.is_empty() {
        return ToolOutput::error("web_search needs a non-empty \"query\".");
    }
    let url = |default: &str| if api.endpoint.trim().is_empty() { default.to_string() } else { api.endpoint.trim().to_string() };
    let request = match api.backend {
        SearchBackend::Tavily => http
            .post(url(TAVILY_URL))
            .bearer_auth(&api.secret)
            .json(&json!({ "query": query, "max_results": RESULT_COUNT })),
        _ => http
            .get(url(BRAVE_URL))
            .header(BRAVE_TOKEN_HEADER, &api.secret)
            .query(&[("q", query.to_string()), ("count", RESULT_COUNT.to_string())]),
    };
    match fetch_json(request).await {
        Ok(body) => format_results(&results(api.backend, &body)),
        Err(e) => ToolOutput::error(e),
    }
}

async fn fetch_json(request: reqwest::RequestBuilder) -> Result<Value, String> {
    let resp = request.send().await.map_err(|e| format!("Search request failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("Search API returned {}", resp.status()));
    }
    let (bytes, _) = read_capped(resp, MAX_RESPONSE_BYTES).await?;
    serde_json::from_slice(&bytes).map_err(|e| format!("Search API reply is not JSON: {e}"))
}

/// `(title, url, snippet)` of every result.
fn results(backend: SearchBackend, body: &Value) -> Vec<(String, String, String)> {
    let (list, snippet_key) = match backend {
        SearchBackend::Tavily => (&body["results"], "content"),
        _ => (&body["web"]["results"], "description"),
    };
    let field = |item: &Value, key: &str| item[key].as_str().unwrap_or_default().to_string();
    list.as_array()
        .map(|items| items.iter().map(|item| (field(item, "title"), field(item, "url"), field(item, snippet_key))).collect())
        .unwrap_or_default()
}

fn format_results(results: &[(String, String, String)]) -> ToolOutput {
    if results.is_empty() {
        return ToolOutput::text("No web results.");
    }
    let mut output = ToolOutput::text(String::new());
    let lines: Vec<String> = results
        .iter()
        .map(|(title, url, snippet)| {
            output.add_source(SourceRef::new(SourceKind::Web, url, title));
            format!("{title} — {url}\n{snippet}")
        })
        .collect();
    output.content = lines.join("\n\n");
    output
}
