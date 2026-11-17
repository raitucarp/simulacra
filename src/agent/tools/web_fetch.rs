//! `web_fetch`: one web page as text, within the fetch policy's size, time and host limits.

use super::context::FetchPolicy;
use super::html_text::html_to_text;
use super::http::{checked_url, read_capped, truncate_chars};
use super::ToolOutput;
use crate::domain::post::{SourceKind, SourceRef};
use serde_json::{json, Value};

/// Default and maximum characters of page text returned.
const DEFAULT_CHARS: usize = 4000;
const MAX_CHARS: usize = 12000;
/// Content-type fragment of HTML documents.
const HTML_MARKER: &str = "html";

/// The tool's argument schema.
pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "url": { "type": "string", "description": "An http(s) URL to read." },
            "max_chars": { "type": "integer", "description": "Characters of page text to return (up to 12000)." }
        },
        "required": ["url"]
    })
}

/// Fetches the page at `args.url`.
pub async fn run(http: &reqwest::Client, policy: &FetchPolicy, args: &Value) -> ToolOutput {
    let url = match checked_url(args["url"].as_str().unwrap_or_default(), policy) {
        Ok(url) => url,
        Err(e) => return ToolOutput::error(e),
    };
    let resp = match http.get(url.clone()).send().await {
        Ok(resp) => resp,
        Err(e) => return ToolOutput::error(format!("Fetching {url} failed: {e}")),
    };
    if !resp.status().is_success() {
        return ToolOutput::error(format!("Fetching {url} returned {}", resp.status()));
    }
    let is_html = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.contains(HTML_MARKER));
    let (bytes, cut) = match read_capped(resp, policy.max_bytes).await {
        Ok(read) => read,
        Err(e) => return ToolOutput::error(e),
    };
    let raw = String::from_utf8_lossy(&bytes);
    let text = if is_html { html_to_text(&raw) } else { raw.into_owned() };
    let max_chars = args["max_chars"].as_u64().map_or(DEFAULT_CHARS, |n| n as usize).clamp(1, MAX_CHARS);
    let note = if cut { " (page cut at the size limit)" } else { "" };
    let mut output = ToolOutput::text(format!("{url}{note}\n{}", truncate_chars(&text, max_chars)));
    output.add_source(SourceRef::new(SourceKind::Web, url.as_str(), url.as_str()));
    output
}
