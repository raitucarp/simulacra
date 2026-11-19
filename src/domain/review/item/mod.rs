//! Review items. Nothing is a free-floating flashcard: every item is anchored to a source.
//!
//! - **Anchored:** a post or a vault passage is one face; the learner wrote the other.
//!   "Add question" makes the source the answer ([`AnchorSide::Back`]); "Add answer" makes
//!   it the prompt ([`AnchorSide::Front`]).
//! - **Connections:** a map of points (posts, passages, concepts) whose connections are
//!   recalled as a whole (see [`crate::domain::review::connections`]).

mod anchor;
mod draft;
mod faces;
mod retire;

pub use anchor::Anchor;
pub use draft::{draft_problem, edited_item, new_item, new_map_item, parse_tags, DraftProblem, ItemDraft, NewItemContext};
pub use faces::{Face, ItemFaces};
pub use retire::{retire_orphans, retired_now};

use super::connections::ConnectionMap;
use super::schedule::ItemSchedule;
use crate::domain::NetworkId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Prefix of generated item ids.
pub const ITEM_ID_PREFIX: &str = "item_";

/// An item's identifier (the `review_items.id` column).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ItemId(String);

impl ItemId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// A fresh random id.
    pub fn generate() -> Self {
        Self(format!("{ITEM_ID_PREFIX}{}", uuid::Uuid::new_v4().simple()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Which face the anchored source is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorSide {
    /// The source is the prompt; the learner wrote the answer ("Add answer").
    Front,
    /// The source is the answer; the learner wrote the question ("Add question").
    #[default]
    Back,
}

impl AnchorSide {
    pub const ALL: [Self; 2] = [Self::Back, Self::Front];

    /// Stable key (locale keys, wire). Never rename one.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Front => "front",
            Self::Back => "back",
        }
    }
}

/// What an item asks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Subject {
    Anchored { anchor: Anchor, side: AnchorSide, written: String },
    Connections { map: ConnectionMap },
}

impl Subject {
    /// Stable key of the variant (statistics, the storage column, locale keys).
    pub const fn key(&self) -> &'static str {
        match self {
            Self::Anchored { side: AnchorSide::Back, .. } => "question",
            Self::Anchored { side: AnchorSide::Front, .. } => "answer",
            Self::Connections { .. } => "connections",
        }
    }

    pub fn anchor(&self) -> Option<&Anchor> {
        match self {
            Self::Anchored { anchor, .. } => Some(anchor),
            Self::Connections { .. } => None,
        }
    }

    pub fn map(&self) -> Option<&ConnectionMap> {
        match self {
            Self::Connections { map } => Some(map),
            Self::Anchored { .. } => None,
        }
    }
}

/// One review item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewItem {
    pub id: ItemId,
    pub network_id: NetworkId,
    pub subject: Subject,
    pub tags: Vec<String>,
    pub created_at: Timestamp,
    pub schedule: ItemSchedule,
    /// Left out of reviews until resumed.
    pub suspended: bool,
    /// Its post was deleted: out of reviews for good, its log kept.
    #[serde(default)]
    pub retired: bool,
}

impl ReviewItem {
    /// Whether the item takes part in reviews.
    pub fn is_active(&self) -> bool {
        !self.suspended && !self.retired
    }

    /// Whether the item shares a tag with `other` (quiz distractors prefer these).
    pub fn shares_tag_with(&self, other: &ReviewItem) -> bool {
        self.tags.iter().any(|tag| other.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
    }

    /// The key grouping items of one source (a map groups under its own id).
    pub fn group_key(&self) -> String {
        self.subject.anchor().map_or_else(|| self.id.as_str().to_string(), Anchor::key)
    }

    /// Whether the item is anchored to post `id`.
    pub fn is_on_post(&self, id: &str) -> bool {
        matches!(self.subject.anchor(), Some(Anchor::Post { id: post }) if post == id)
    }
}
