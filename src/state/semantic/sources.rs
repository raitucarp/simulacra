//! What the index covers: every non-empty post (instant reposts carry no text of their own;
//! a locked post, X2, only a placeholder, so it is left out)
//! and every vault document, across all networks.

use super::SourceText;
use crate::domain::embedding::EmbeddingOwner;
use crate::state::AppState;

/// Every embeddable post and document.
pub fn index_sources(state: &AppState) -> Vec<SourceText> {
    let posts = state
        .posts
        .iter()
        .filter(|post| !post.is_repost() && !post.is_locked() && !post.content.trim().is_empty())
        .map(|post| SourceText { owner: EmbeddingOwner::post(&post.id), title: String::new(), text: post.content.clone() });
    let documents = state
        .documents
        .iter()
        .map(|doc| SourceText { owner: EmbeddingOwner::document(doc.id.clone()), title: doc.title.clone(), text: doc.content.clone() });
    posts.chain(documents).collect()
}

/// A cheap fingerprint of the indexed collections: when it is unchanged, so is the plan.
/// (Counts, newest ids and total text length catch additions, deletions and edits.)
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IndexKey {
    posts: usize,
    documents: usize,
    newest_post: Option<String>,
    newest_document: Option<String>,
    text_bytes: usize,
    /// Model, signature, endpoint and key identity of the provider configuration.
    config: String,
}

impl IndexKey {
    pub fn of(state: &AppState, config: &str) -> Self {
        let post_bytes: usize = state.posts.iter().map(|post| post.content.len()).sum();
        let document_bytes: usize = state.documents.iter().map(|doc| doc.content.len() + doc.title.len()).sum();
        Self {
            posts: state.posts.len(),
            documents: state.documents.len(),
            newest_post: state.posts.first().map(|post| post.id.as_str().to_string()),
            newest_document: state.documents.first().map(|doc| doc.id.clone()),
            text_bytes: post_bytes + document_bytes,
            config: config.to_string(),
        }
    }

    /// Whether only the collections changed (the provider configuration is the same).
    pub fn same_config(&self, other: &Self) -> bool {
        self.config == other.config
    }
}
