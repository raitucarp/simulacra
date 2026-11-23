//! The view chosen per thread and per profile (W8), kept in the settings table as JSON.
//! The most recent choices come first; past [`MAX_REMEMBERED`] the oldest are forgotten, so
//! the setting stays small however many threads were opened.

use super::{PostView, ViewScope};
use crate::domain::{PostId, ProfileId};
use serde::{Deserialize, Serialize};

/// Choices kept at most.
pub const MAX_REMEMBERED: usize = 200;

/// What a choice belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ViewOwner {
    /// The thread rooted at this post.
    Thread(PostId),
    Profile(ProfileId),
}

impl ViewOwner {
    /// The scope the owner's views come from.
    pub fn scope(&self) -> ViewScope {
        match self {
            Self::Thread(_) => ViewScope::Thread,
            Self::Profile(_) => ViewScope::Profile,
        }
    }

    /// The stored key: `thread:<id>` or `profile:<id>`.
    pub fn key(&self) -> String {
        match self {
            Self::Thread(id) => format!("thread:{id}"),
            Self::Profile(id) => format!("profile:{id}"),
        }
    }
}

/// One remembered choice (stored).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewChoice {
    pub owner: String,
    pub view: PostView,
}

/// Every remembered choice, newest first.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ViewMemory {
    pub choices: Vec<ViewChoice>,
}

impl ViewMemory {
    /// The memory stored as `json` (empty when it is missing or unreadable).
    pub fn from_json(json: Option<&str>) -> Self {
        json.and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// The view chosen for `owner`, if its scope still offers it.
    pub fn chosen(&self, owner: &ViewOwner) -> Option<PostView> {
        let key = owner.key();
        self.choices.iter().find(|choice| choice.owner == key).map(|choice| choice.view).filter(|view| view.offered_in(owner.scope()))
    }

    /// Remembers `view` for `owner` (newest first, the oldest forgotten past the cap).
    pub fn remember(&mut self, owner: &ViewOwner, view: PostView) {
        let key = owner.key();
        self.choices.retain(|choice| choice.owner != key);
        self.choices.insert(0, ViewChoice { owner: key, view });
        self.choices.truncate(MAX_REMEMBERED);
    }
}
