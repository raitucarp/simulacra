//! N8: audience scopes. A post says who may read it: everyone (no [`Audience`], the default),
//! only the user, or chosen agents (`extras.audience`). The scope decides which agents may read
//! the post: an agent outside it never gets it in a prompt, a tool result or a reply job. The
//! author always reads their own post, and no agent ever reads a locked one.

use crate::domain::{Post, ProfileId};
use serde::{Deserialize, Serialize};

/// Who besides the author may read a post (`extras.audience`; absent: everyone).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "snake_case")]
pub enum Audience {
    /// The user alone: no agent reads it.
    OnlyMe,
    /// These agents alone (none: the user alone).
    Agents {
        #[serde(default)]
        agents: Vec<ProfileId>,
    },
}

impl Audience {
    /// Whether the agent `reader` is in the audience.
    pub fn admits(&self, reader: &ProfileId) -> bool {
        match self {
            Self::OnlyMe => false,
            Self::Agents { agents } => agents.contains(reader),
        }
    }

    /// The audience with `agent` added or taken out; `None` (everyone) never comes back from it.
    #[must_use]
    pub fn toggled(&self, agent: &ProfileId) -> Self {
        let mut agents = match self {
            Self::OnlyMe => Vec::new(),
            Self::Agents { agents } => agents.clone(),
        };
        match agents.iter().position(|id| id == agent) {
            Some(at) => drop(agents.remove(at)),
            None => agents.push(agent.clone()),
        }
        Self::Agents { agents }
    }

    /// Stable key (locale keys `posts.audience.<key>`).
    pub const fn key(&self) -> &'static str {
        match self {
            Self::OnlyMe => "only_me",
            Self::Agents { .. } => "agents",
        }
    }
}

/// Whether the agent `reader` may read `post`: never a locked one; its own always; otherwise
/// when the post has no audience or its audience admits the agent.
pub fn readable_by(post: &Post, reader: &ProfileId) -> bool {
    if post.is_locked() {
        return false;
    }
    &post.author_id == reader || post.extras.audience.as_deref().is_none_or(|audience| audience.admits(reader))
}

#[cfg(test)]
mod tests;
