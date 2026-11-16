//! Multi-post threads: splitting long drafts and validating every draft before publishing.

mod split;

pub use split::split_into_thread;

use super::post::MAX_POST_CHARS;

/// Why a single thread draft cannot be published.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftIssue {
    /// No text (and no attachment that could carry the post).
    Empty,
    /// More characters than the per-post limit.
    TooLong { actual: usize, max: usize },
}

impl DraftIssue {
    /// Characters over the limit (zero for non-length issues).
    pub fn overflow(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::TooLong { actual, max } => actual.saturating_sub(*max),
        }
    }
}

/// The first draft of a thread that blocks publishing, with its zero-based position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreadProblem {
    pub index: usize,
    pub issue: DraftIssue,
}

/// Checks one draft: blank text is only allowed when an attachment carries the post.
pub fn draft_issue(text: &str, has_media: bool) -> Option<DraftIssue> {
    let trimmed = text.trim();
    if trimmed.is_empty() && !has_media {
        return Some(DraftIssue::Empty);
    }
    let actual = trimmed.chars().count();
    (actual > MAX_POST_CHARS).then_some(DraftIssue::TooLong { actual, max: MAX_POST_CHARS })
}

/// Validates every draft of a thread in order. Attachments belong to the first post, so only
/// it may be text-less when `first_has_media` is set. An empty thread is reported as an empty
/// first draft.
pub fn first_thread_problem<S: AsRef<str>>(drafts: &[S], first_has_media: bool) -> Option<ThreadProblem> {
    if drafts.is_empty() {
        return Some(ThreadProblem { index: 0, issue: DraftIssue::Empty });
    }
    drafts.iter().enumerate().find_map(|(index, text)| {
        let has_media = index == 0 && first_has_media;
        draft_issue(text.as_ref(), has_media).map(|issue| ThreadProblem { index, issue })
    })
}

#[cfg(test)]
mod tests;
