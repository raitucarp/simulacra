//! N15: distilling a post's thread into the vault, from its menu, with or without a summary.

use crate::domain::PostId;

/// One request to distill a thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistillEvent {
    /// The thread of this post, as it is.
    Thread(PostId),
    /// The thread of this post with a summary from the AI engine on top.
    Summarized(PostId),
}
