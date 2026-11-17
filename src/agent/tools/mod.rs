//! Agent tools, executed on the generation thread over a [`ToolContext`] snapshot.
//!
//! - [`context`]: the `Send` snapshot (network posts, profiles, documents) and fetch policy.
//! - [`toolbox`]: the granted tools of one generation, their specs and dispatch.
//! - [`vault_search`]: BM25 over chunked vault documents.
//! - [`network`]: profiles, recent posts, threads and post search.
//! - [`web_fetch`] / [`web_search`] / [`custom_http`]: capped HTTP tools.
//! - [`http`] / [`html_text`]: URL policy, capped reads, HTML → text.

pub mod context;
pub mod custom_http;
pub mod html_text;
pub mod http;
pub mod network;
mod toolbox;
pub mod vault_search;
pub mod web_fetch;
pub mod web_search;

#[cfg(test)]
mod tests;

pub use context::{DocumentSnapshot, FetchPolicy, PostSnapshot, ProfileSnapshot, ToolContext};
pub use toolbox::ToolBox;

use crate::domain::post::SourceRef;

/// Characters of one tool result handed back to the model.
pub const MAX_TOOL_RESULT_CHARS: usize = 8000;

/// What one tool call returned.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ToolOutput {
    /// Text handed back to the model.
    pub content: String,
    /// Documents, posts, profiles or pages the result drew on.
    pub sources: Vec<SourceRef>,
    /// Whether the call failed (the content then explains why).
    pub failed: bool,
}

impl ToolOutput {
    /// A successful result.
    pub fn text(content: impl Into<String>) -> Self {
        Self { content: content.into(), ..Self::default() }
    }

    /// A failed call; the model sees the reason and may recover.
    pub fn error(reason: impl Into<String>) -> Self {
        Self { content: format!("Error: {}", reason.into()), failed: true, ..Self::default() }
    }

    /// Adds `source` unless already listed.
    pub fn add_source(&mut self, source: SourceRef) {
        if !self.sources.contains(&source) {
            self.sources.push(source);
        }
    }
}
