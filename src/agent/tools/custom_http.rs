//! User-defined HTTP tools: a URL template filled from the model's JSON arguments.

use super::http::{percent_encode, read_capped, truncate_chars};
use super::ToolOutput;
use crate::agent::template::{fill, plain};
use crate::domain::engine::{HttpMethod, HttpToolDef};
use serde_json::{json, Map, Value};

/// Largest custom-tool response read.
const MAX_RESPONSE_BYTES: usize = 128 * 1024;
/// Characters of the response returned to the model.
const MAX_RESPONSE_CHARS: usize = 6000;
/// Longest model-facing tool name.
const MAX_TOOL_NAME_CHARS: usize = 64;
/// Name used when the configured one sanitizes to nothing.
const FALLBACK_TOOL_NAME: &str = "custom_tool";
/// Placeholder of the credential secret.
const API_KEY_NAME: &str = "api_key";

/// A custom tool ready to run: its definition, model-facing name and secret.
#[derive(Debug, Clone)]
pub struct HttpTool {
    pub def: HttpToolDef,
    pub name: String,
    pub secret: String,
}

/// `[A-Za-z0-9_-]`, at most [`MAX_TOOL_NAME_CHARS`] characters.
pub fn sanitize_tool_name(raw: &str) -> String {
    let clean: String = raw
        .trim()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '_' })
        .take(MAX_TOOL_NAME_CHARS)
        .collect();
    if clean.trim_matches('_').is_empty() { FALLBACK_TOOL_NAME.to_string() } else { clean }
}

/// The argument schema (an empty object schema when the configured one is invalid).
pub fn parameters(def: &HttpToolDef) -> Value {
    serde_json::from_str::<Value>(&def.params_schema)
        .ok()
        .filter(Value::is_object)
        .unwrap_or_else(|| json!({ "type": "object", "properties": {} }))
}

/// Calls the tool with `args`.
pub async fn run(http: &reqwest::Client, tool: &HttpTool, args: &Value) -> ToolOutput {
    let empty = Map::new();
    let map = args.as_object().unwrap_or(&empty);
    let lookup = |encode: bool| {
        move |name: &str| {
            let value = if name == API_KEY_NAME { Some(tool.secret.clone()) } else { map.get(name).map(plain) };
            value.map(|v| if encode { percent_encode(&v) } else { v })
        }
    };
    let (url, used) = fill(&tool.def.url_template, lookup(true));
    let method = reqwest::Method::from_bytes(tool.def.method.as_str().as_bytes()).unwrap_or(reqwest::Method::GET);
    let mut request = http.request(method, url.trim());
    for header in tool.def.headers.iter().filter(|h| !h.name.trim().is_empty()) {
        request = request.header(header.name.trim(), fill(&header.value, lookup(false)).0);
    }
    request = if tool.def.method.has_body() {
        request.json(args)
    } else {
        let query: Vec<(String, String)> =
            map.iter().filter(|(k, _)| !used.contains(k)).map(|(k, v)| (k.clone(), plain(v))).collect();
        request.query(&query)
    };
    send(request, tool.def.method).await
}

async fn send(request: reqwest::RequestBuilder, method: HttpMethod) -> ToolOutput {
    let resp = match request.send().await {
        Ok(resp) => resp,
        Err(e) => return ToolOutput::error(format!("{} request failed: {e}", method.as_str())),
    };
    let status = resp.status();
    match read_capped(resp, MAX_RESPONSE_BYTES).await {
        Ok((bytes, _)) if status.is_success() => {
            ToolOutput::text(truncate_chars(&String::from_utf8_lossy(&bytes), MAX_RESPONSE_CHARS))
        }
        Ok((bytes, _)) => ToolOutput::error(format!("HTTP {status}: {}", truncate_chars(&String::from_utf8_lossy(&bytes), MAX_RESPONSE_CHARS))),
        Err(e) => ToolOutput::error(e),
    }
}
