//! What an action knows about the post: the author, the thought, the thread above it and the
//! replies below it, as owned plain data (built by the shell, rendered into the prompt).

use crate::i18n::Language;

/// One post of the thread as the prompt sees it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ThreadLine {
    pub author_name: String,
    pub author_handle: String,
    pub text: String,
    /// Reply depth below the thought (0 for the thought and the posts above it).
    pub depth: usize,
}

impl ThreadLine {
    pub fn new(author_name: &str, author_handle: &str, text: &str, depth: usize) -> Self {
        Self { author_name: author_name.into(), author_handle: author_handle.into(), text: text.into(), depth }
    }
}

/// Who wrote the thought.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthorContext {
    /// A short self-description (an agent's bio; empty for most people).
    pub bio: String,
    /// The author is an autonomous agent rather than the user.
    pub is_agent: bool,
}

/// Everything an action is built from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionContext {
    pub thought: ThreadLine,
    pub author: AuthorContext,
    /// The reply chain above the thought, root first.
    pub ancestors: Vec<ThreadLine>,
    /// Replies below the thought in reading order (depth 1 and deeper).
    pub replies: Vec<ThreadLine>,
    /// The language the user reads the answer in.
    pub reader_language: Language,
}

impl ActionContext {
    /// Whether the thought is part of a conversation (anything above or below it).
    pub fn has_thread(&self) -> bool {
        !self.ancestors.is_empty() || !self.replies.is_empty()
    }
}
