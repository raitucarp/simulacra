//! The toolbox against a local mock HTTP server (no real network).

use super::context;
use crate::agent::credentials::{Credential, CredentialKind, CredentialStore};
use crate::agent::llm::ToolCall;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::agent::tools::{FetchPolicy, ToolBox, ToolContext};
use crate::domain::engine::{HeaderPair, HttpMethod, HttpToolDef, SearchBackend, ToolGrant, WebSearchConfig};
use crate::domain::post::SourceKind;
use serde_json::{json, Value};

fn call(name: &str, arguments: Value) -> ToolCall {
    ToolCall { id: "c1".into(), name: name.into(), arguments, signature: None }
}

fn open_context() -> ToolContext {
    ToolContext { fetch: FetchPolicy { allow_private_hosts: true, ..FetchPolicy::default() }, ..context() }
}

#[tokio::test]
async fn web_fetch_reads_html_as_text_within_policy() {
    let server = MockServer::start(vec![MockReply::raw(200, "text/html; charset=utf-8", "<h1>Hello</h1><script>x()</script><p>World</p>")]);
    let toolbox = ToolBox::new(&[ToolGrant::WebFetch], open_context(), &CredentialStore::default(), false);
    let url = format!("{}/page", server.url);
    let output = toolbox.execute(&call("web_fetch", json!({ "url": url }))).await;
    assert!(!output.failed, "{}", output.content);
    assert_eq!(output.content, format!("{url}\nHello\nWorld"));
    assert_eq!((output.sources[0].kind, output.sources[0].id.as_str()), (SourceKind::Web, url.as_str()));
    assert_eq!(server.requests()[0].path, "/page");

    let strict = ToolBox::new(&[ToolGrant::WebFetch], context(), &CredentialStore::default(), false);
    let refused = strict.execute(&call("web_fetch", json!({ "url": url }))).await;
    assert!(refused.failed && refused.content.contains("local or private"), "{}", refused.content);
}

#[tokio::test]
async fn web_fetch_caps_the_body() {
    let big = "x".repeat(4096);
    let server = MockServer::start(vec![MockReply::raw(200, "text/plain", big)]);
    let ctx = ToolContext { fetch: FetchPolicy { allow_private_hosts: true, max_bytes: 100, ..FetchPolicy::default() }, ..context() };
    let toolbox = ToolBox::new(&[ToolGrant::WebFetch], ctx, &CredentialStore::default(), false);
    let output = toolbox.execute(&call("web_fetch", json!({ "url": server.url }))).await;
    assert!(output.content.contains("cut at the size limit"));
    assert_eq!(output.content.lines().nth(1).map(str::len), Some(100));
}

#[tokio::test]
async fn search_apis_use_their_credential_and_endpoint() {
    let brave = MockServer::start(vec![MockReply::json(json!({ "web": { "results": [
        { "title": "Spinoza", "url": "https://plato.test/spinoza", "description": "Monist." } ] } }))]);
    let tavily = MockServer::start(vec![MockReply::json(json!({ "results": [
        { "title": "Weil", "url": "https://plato.test/weil", "content": "Attention." } ] }))]);
    let mut store = CredentialStore::default();
    store.upsert(Credential::new(CredentialKind::Brave, "B", "brave-key-123", format!("{}/search", brave.url)));
    store.upsert(Credential::new(CredentialKind::Tavily, "T", "tvly-key-456", tavily.url.clone()));
    for (backend, expected) in [(SearchBackend::Brave, "Spinoza — https://plato.test/spinoza\nMonist."), (SearchBackend::Tavily, "Weil — https://plato.test/weil\nAttention.")] {
        let grant = ToolGrant::WebSearch(WebSearchConfig { backend, credential: None });
        let toolbox = ToolBox::new(&[grant], context(), &store, false);
        assert_eq!(toolbox.specs()[0].name, "web_search");
        let output = toolbox.execute(&call("web_search", json!({ "query": "monism" }))).await;
        assert_eq!(output.content, expected);
        assert_eq!(output.sources[0].kind, SourceKind::Web);
    }
    let brave_request = &brave.requests()[0];
    assert_eq!(brave_request.path, "/search?q=monism&count=5");
    assert_eq!(brave_request.header("x-subscription-token"), Some("brave-key-123"));
    let tavily_request = &tavily.requests()[0];
    assert_eq!(tavily_request.header("authorization"), Some("Bearer tvly-key-456"));
    assert_eq!(tavily_request.json(), json!({ "query": "monism", "max_results": 5 }));
}

#[tokio::test]
async fn custom_http_tools_fill_templates_and_redact_secrets() {
    let server = MockServer::start(vec![
        MockReply::json(json!({ "forecast": "rain", "echo": "token=tok-secret-789" })),
        MockReply::raw(503, "text/plain", "down"),
    ]);
    let mut store = CredentialStore::default();
    let cred = Credential::new(CredentialKind::Http, "Weather", "tok-secret-789", "");
    store.upsert(cred.clone());
    let def = HttpToolDef {
        name: "weather now".into(),
        description: "Current weather".into(),
        method: HttpMethod::Get,
        url_template: format!("{}/weather/{{{{city}}}}", server.url),
        headers: vec![HeaderPair::new("X-Key", "{{api_key}}")],
        credential: Some(cred.id.clone()),
        ..HttpToolDef::default()
    };
    let toolbox = ToolBox::new(&[ToolGrant::CustomHttp(def)], context(), &store, false);
    let spec = &toolbox.specs()[0];
    assert_eq!((spec.name.as_str(), spec.parameters["required"][0].as_str()), ("weather_now", Some("query")));
    let output = toolbox.execute(&call("weather_now", json!({ "city": "São Paulo", "units": "metric" }))).await;
    assert!(output.content.contains("rain") && !output.content.contains("tok-secret-789"), "{}", output.content);
    let request = &server.requests()[0];
    assert_eq!(request.path, "/weather/S%C3%A3o%20Paulo?units=metric");
    assert_eq!(request.header("x-key"), Some("tok-secret-789"));
    let failed = toolbox.execute(&call("weather_now", json!({ "city": "Oslo" }))).await;
    assert!(failed.failed && failed.content.contains("503"), "{}", failed.content);
}

#[tokio::test]
async fn unknown_tools_native_search_and_missing_credentials() {
    let grants = [
        ToolGrant::VaultSearch,
        ToolGrant::ReadNetwork,
        ToolGrant::VaultSearch,
        ToolGrant::WebSearch(WebSearchConfig::default()),
        ToolGrant::WebSearch(WebSearchConfig { backend: SearchBackend::Tavily, credential: None }),
    ];
    let toolbox = ToolBox::new(&grants, context(), &CredentialStore::default(), true);
    let names: Vec<String> = toolbox.specs().into_iter().map(|s| s.name).collect();
    assert_eq!(names, vec!["vault_search", "read_network"], "duplicates and keyless search APIs are dropped");
    assert!(toolbox.native_web_search);
    assert!(!ToolBox::new(&grants, context(), &CredentialStore::default(), false).native_web_search);
    let missing = toolbox.execute(&call("launch_rockets", json!({}))).await;
    assert!(missing.failed && missing.content.contains("launch_rockets"));
    let found = toolbox.execute(&call("vault_search", json!({ "query": "bread dough" }))).await;
    assert_eq!(found.sources[0].id, "doc_bread");
}
