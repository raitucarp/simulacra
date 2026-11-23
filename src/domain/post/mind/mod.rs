//! N19: "Changed my mind". The user marks one of their posts as one that changed their view
//! ([`MindChange`], `extras.mind_change`): it is linked to the earlier post it revises, says
//! what they used to think, or both. The revised post knows it was revised. A locked post is
//! never marked, nor linked as revised: either would speak past its lock.

use crate::domain::PostId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// The longest "what I used to think", in characters.
pub const MAX_WAS_CHARS: usize = 280;

/// A post that changed the user's mind (`extras.mind_change`, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MindChange {
    /// The earlier post it revises.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revises: Option<PostId>,
    /// What the user used to think, in their words.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub was: String,
    pub at: Timestamp,
}

impl MindChange {
    /// How an agent reads the mark with the post (the old view only: the revised post may be
    /// outside the agent's scope).
    pub fn agent_note(&self) -> String {
        match self.was.as_str() {
            "" => "[The author changed their mind with this post]".to_string(),
            was => format!("[The author changed their mind with this post; before, they thought: {was}]"),
        }
    }
}

/// "What I used to think" as kept: lines trimmed, blank runs dropped (empty is allowed);
/// `None` when too long.
pub fn clean_was(raw: &str) -> Option<String> {
    let was = raw.lines().map(str::trim).filter(|line| !line.is_empty()).collect::<Vec<_>>().join("\n");
    (was.chars().count() <= MAX_WAS_CHARS).then_some(was)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_old_view_is_kept_tidy_and_read_by_agents() {
        assert_eq!(clean_was("  Cities need more roads.\n\n "), Some("Cities need more roads.".into()));
        assert_eq!(clean_was(""), Some(String::new()));
        assert_eq!(clean_was(&"x".repeat(MAX_WAS_CHARS + 1)), None);
        let change = MindChange { revises: None, was: "More roads, less traffic.".into(), at: Timestamp::UNIX_EPOCH };
        assert!(change.agent_note().contains("before, they thought: More roads, less traffic."));
        let json = serde_json::to_string(&change).expect("json");
        assert_eq!(serde_json::from_str::<MindChange>(&json).expect("change"), change);
    }
}
