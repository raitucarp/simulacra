//! N15: distilling a post's thread into the vault, from its menu.

use crate::domain::PostId;

/// One request to distill a thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistillEvent {
    /// The thread of this post, as it is.
    Thread(PostId),
}
