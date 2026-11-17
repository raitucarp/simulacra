//! Executes a user-built [`CustomRequest`]: placeholders filled (JSON values in the body,
//! raw text in the URL and headers), the reply decoded as JSON.

use crate::agent::template::fill;
use crate::domain::engine::CustomRequest;
use serde_json::Value;

/// Placeholder names expanded in URLs and headers.
const API_KEY_NAME: &str = "api_key";
const MODEL_NAME: &str = "model";
/// Largest custom reply read.
const MAX_REPLY_BYTES: usize = 32 * 1024 * 1024;

/// Expands `{{name}}` placeholders of `template` with the JSON encoding of `vars`, then parses
/// the result as JSON. Unknown placeholders are left in place (and usually make it invalid).
pub fn render_body(template: &str, vars: &[(&str, Value)]) -> Result<Value, String> {
    let (text, _) = fill(template, |name| vars.iter().find(|(key, _)| *key == name).map(|(_, value)| value.to_string()));
    serde_json::from_str(&text).map_err(|e| format!("The custom body template is not valid JSON after filling it in: {e}"))
}

/// Expands `{{api_key}}` and `{{model}}` as raw text (URLs and header values).
pub fn render_text(template: &str, secret: &str, model: &str) -> String {
    fill(template, |name| match name {
        API_KEY_NAME => Some(secret.to_string()),
        MODEL_NAME => Some(model.to_string()),
        _ => None,
    })
    .0
}

/// Sends `request` with `vars` and returns the JSON reply.
pub async fn send(
    http: &reqwest::Client,
    request: &CustomRequest,
    secret: &str,
    model: &str,
    vars: &[(&str, Value)],
) -> Result<Value, String> {
    let url = render_text(&request.url, secret, model);
    if url.trim().is_empty() {
        return Err("The custom request has no URL".to_string());
    }
    let method = reqwest::Method::from_bytes(request.method.as_str().as_bytes()).unwrap_or(reqwest::Method::POST);
    let mut builder = http.request(method, url.trim());
    for header in request.headers.iter().filter(|h| !h.name.trim().is_empty()) {
        builder = builder.header(header.name.trim(), render_text(&header.value, secret, model));
    }
    if request.method.has_body() {
        builder = builder.json(&render_body(&request.body_template, vars)?);
    }
    let resp = builder.send().await.map_err(|e| format!("Custom request error: {e}"))?;
    let status = resp.status();
    let (bytes, _) = crate::agent::tools::http::read_capped(resp, MAX_REPLY_BYTES).await?;
    if !status.is_success() {
        return Err(format!("Custom request failed ({status}): {}", String::from_utf8_lossy(&bytes)));
    }
    serde_json::from_slice(&bytes).map_err(|e| format!("Custom reply is not JSON: {e}"))
}
