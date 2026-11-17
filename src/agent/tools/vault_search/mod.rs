//! `vault_search`: lexical BM25 search over the network's Knowledge Vault documents.

mod bm25;
mod chunk;

pub use bm25::{Bm25Index, Hit};
pub use chunk::{chunk_document, tokenize, Chunk, CHUNK_OVERLAP_WORDS, CHUNK_WORDS};

use super::context::DocumentSnapshot;
use super::ToolOutput;
use crate::domain::post::{SourceKind, SourceRef};
use serde_json::{json, Value};

/// Default and maximum number of passages returned.
const DEFAULT_RESULTS: usize = 4;
const MAX_RESULTS: usize = 8;

/// The tool's argument schema.
pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "query": { "type": "string", "description": "Keywords to look for in the vault documents." },
            "limit": { "type": "integer", "description": "How many passages to return (1-8)." }
        },
        "required": ["query"]
    })
}

/// Indexes every chunk of `documents`.
pub fn index_documents(documents: &[DocumentSnapshot]) -> Bm25Index {
    let chunks = documents.iter().flat_map(|doc| chunk_document(&doc.id, &doc.title, &doc.content)).collect();
    Bm25Index::new(chunks)
}

/// Runs a search call: the best passages as text, each document as a source.
pub fn run(index: &Bm25Index, args: &Value) -> ToolOutput {
    let query = args["query"].as_str().unwrap_or_default();
    if query.trim().is_empty() {
        return ToolOutput::error("vault_search needs a non-empty \"query\".");
    }
    let limit = args["limit"].as_u64().map_or(DEFAULT_RESULTS, |n| n as usize).clamp(1, MAX_RESULTS);
    let hits = index.search(query, limit);
    if hits.is_empty() {
        return ToolOutput::text(format!("No vault passage matches \"{query}\"."));
    }
    let mut output = ToolOutput::text(String::new());
    let passages: Vec<String> = hits
        .iter()
        .map(|hit| {
            output.add_source(SourceRef::new(SourceKind::VaultDoc, &hit.chunk.doc_id, &hit.chunk.title));
            format!("[{}] {} (passage {}, score {:.2})\n{}", hit.chunk.doc_id, hit.chunk.title, hit.chunk.ordinal + 1, hit.score, hit.chunk.text)
        })
        .collect();
    output.content = passages.join("\n\n");
    output
}
