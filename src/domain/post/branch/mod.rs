//! N6: what-if branches. A thread forks at any post into alternate lines of thought: the first
//! reply of a branch carries a [`BranchPoint`] (`extras.branch`) naming what it supposes, and
//! every reply under it follows that branch. Replies without one carry on the main line. A
//! locked post is never forked, and a branch whose first post is locked shows no label.

use serde::{Deserialize, Serialize};

/// The longest label a branch has, in characters.
pub const MAX_BRANCH_LABEL_CHARS: usize = 80;

/// The pointer a branch's first reply carries (`extras.branch`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BranchPoint {
    /// What the branch supposes ("What if we used a queue?").
    pub label: String,
}

impl BranchPoint {
    /// A branch named `label` (on one line, spaces collapsed); `None` when it is empty or
    /// longer than [`MAX_BRANCH_LABEL_CHARS`].
    pub fn named(label: &str) -> Option<Self> {
        let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
        let count = label.chars().count();
        (count > 0 && count <= MAX_BRANCH_LABEL_CHARS).then_some(Self { label })
    }
}

#[cfg(test)]
mod tests;
