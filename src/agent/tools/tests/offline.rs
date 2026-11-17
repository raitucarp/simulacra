//! Pure tool pieces: chunking, BM25 ranking, HTML → text, URL policy and templates.

use super::documents;
use crate::agent::template::fill;
use crate::agent::tools::context::FetchPolicy;
use crate::agent::tools::custom_http::sanitize_tool_name;
use crate::agent::tools::html_text::html_to_text;
use crate::agent::tools::http::{checked_url, is_private_host, percent_encode, truncate_chars};
use crate::agent::tools::vault_search::{self, chunk_document, tokenize, Bm25Index, CHUNK_OVERLAP_WORDS, CHUNK_WORDS};
use crate::domain::post::SourceKind;
use serde_json::json;

#[test]
fn documents_chunk_into_overlapping_windows() {
    let words: Vec<String> = (0..250).map(|i| format!("w{i}")).collect();
    let chunks = chunk_document("d", "T", &words.join(" "));
    let stride = CHUNK_WORDS - CHUNK_OVERLAP_WORDS;
    assert_eq!(chunks.len(), 3);
    assert!(chunks[0].text.starts_with("w0 ") && chunks[0].text.ends_with(&format!("w{}", CHUNK_WORDS - 1)));
    assert!(chunks[1].text.starts_with(&format!("w{stride} ")));
    assert!(chunks[2].text.ends_with("w249"));
    assert_eq!(chunks.iter().map(|c| c.ordinal).collect::<Vec<_>>(), vec![0, 1, 2]);
    assert!(chunk_document("d", "T", "   ").is_empty());
    assert_eq!(chunk_document("d", "T", "one two").len(), 1);
}

#[test]
fn tokenizer_lowercases_and_drops_noise() {
    assert_eq!(tokenize("The Mind, and the BODY: a monism!"), vec!["mind", "body", "monism"]);
}

#[test]
fn bm25_ranks_the_matching_document_first() {
    let index = vault_search::index_documents(&documents());
    assert_eq!(index.len(), 3);
    let hits = index.search("monism of mind and body", 3);
    assert_eq!(hits[0].chunk.doc_id, "doc_ethics");
    assert!(hits.iter().all(|h| h.chunk.doc_id != "doc_bread"), "no shared terms, no hit");
    let attention = index.search("attention generosity", 3);
    assert_eq!(attention[0].chunk.doc_id, "doc_attention");
    assert!(index.search("quantum", 3).is_empty());
    assert!(Bm25Index::new(Vec::new()).search("anything", 3).is_empty());
}

#[test]
fn bm25_prefers_rarer_terms_and_respects_the_limit() {
    let docs = vec![
        chunk_document("common", "A", "idea idea idea idea"),
        chunk_document("rare", "B", "idea zeugma"),
        chunk_document("other", "C", "idea plain text here"),
    ];
    let index = Bm25Index::new(docs.into_iter().flatten().collect());
    let hits = index.search("zeugma idea", 2);
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].chunk.doc_id, "rare", "the rare term outweighs repetition of a common one");
    assert!(hits[0].score > hits[1].score);
}

#[test]
fn vault_search_tool_returns_passages_and_sources() {
    let index = vault_search::index_documents(&documents());
    let output = vault_search::run(&index, &json!({ "query": "substance monism", "limit": 1 }));
    assert!(!output.failed);
    assert!(output.content.starts_with("[doc_ethics] Ethics (passage 1"), "{}", output.content);
    assert_eq!(output.sources.len(), 1);
    assert_eq!((output.sources[0].kind, output.sources[0].id.as_str()), (SourceKind::VaultDoc, "doc_ethics"));
    assert!(vault_search::run(&index, &json!({})).failed);
    assert!(vault_search::run(&index, &json!({ "query": "zzz" })).content.contains("No vault passage"));
}

#[test]
fn html_becomes_readable_text() {
    let html = "<html><head><title>T</title><style>p{color:red}</style></head><body><h1>Title</h1>\
                <script>alert('x')</script><p>One &amp; two&nbsp;three</p><div>Next<br>line</div>\
                <SCRIPT type=x>var a = '<p>';</SCRIPT><p>caf\u{e9} &lt;ok&gt;</p></body></html>";
    assert_eq!(html_to_text(html), "Title\nOne & two three\nNext\nline\ncaf\u{e9} <ok>");
    assert_eq!(html_to_text("plain text"), "plain text");
}

#[test]
fn private_hosts_are_refused_unless_allowed() {
    let policy = FetchPolicy::default();
    for url in ["http://localhost:8080", "http://127.0.0.1", "http://10.0.0.5/x", "http://192.168.1.1", "http://[::1]/", "http://printer.local", "http://169.254.169.254/meta"] {
        assert!(checked_url(url, &policy).is_err(), "{url}");
    }
    assert!(checked_url("https://example.com/page", &policy).is_ok());
    assert!(checked_url("ftp://example.com", &policy).is_err());
    assert!(checked_url("not a url", &policy).is_err());
    let open = FetchPolicy { allow_private_hosts: true, ..policy };
    assert!(checked_url("http://127.0.0.1:9/x", &open).is_ok());
    assert!(!is_private_host(&reqwest::Url::parse("https://8.8.8.8").expect("url")));
}

#[test]
fn templates_encoding_and_names() {
    let (text, used) = fill("{{a}}-{{ b }}-{{missing}}-{{a}}", |name| (name != "missing").then(|| name.to_uppercase()));
    assert_eq!(text, "A-B-{{missing}}-A");
    assert_eq!(used, vec!["a", "b", "a"]);
    assert_eq!(fill("open {{never", |_| Some("x".into())).0, "open {{never");
    assert_eq!(percent_encode("a b/é?&"), "a%20b%2F%C3%A9%3F%26");
    assert_eq!(sanitize_tool_name(" Get weather! "), "Get_weather_");
    assert_eq!(sanitize_tool_name("***"), "custom_tool");
    assert_eq!(truncate_chars("abcdef", 3), "abc …[truncated]");
    assert_eq!(truncate_chars("abc", 3), "abc");
}
