//! Learning goals: a title, why it matters, a target date, the material it is about (posts,
//! threads, vault documents, review items, concepts) and tasks (often chosen in the weekly
//! review). Progress and the forecast are pure functions of the deck, the learner's posts
//! and the tutor sessions.

pub mod edit;
pub mod forecast;
pub mod progress;

pub use forecast::{expected_share, forecast, Forecast};
pub use progress::{goal_progress, GoalInputs, GoalProgress, OwnPost};

use crate::domain::learning::concept::concept_key;
use crate::domain::review::ItemId;
use crate::domain::NetworkId;
use jiff::Timestamp;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

/// Prefix of goal ids.
pub const GOAL_ID_PREFIX: &str = "goal";
/// Prefix of task ids.
pub const TASK_ID_PREFIX: &str = "task";

/// A goal's identifier (the `learning_goals.id` column).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GoalId(pub String);

impl GoalId {
    pub fn generate() -> Self {
        Self(super::fresh_id(GOAL_ID_PREFIX))
    }
}

/// Material a goal is about.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GoalLink {
    Post { id: String },
    Thread { root_id: String },
    Document { id: String },
    Item { id: ItemId },
    /// A concept, by its key (see [`concept_key`]).
    Concept { name: String },
}

impl GoalLink {
    /// A concept link, its name normalised.
    pub fn concept(name: &str) -> Self {
        Self::Concept { name: concept_key(name) }
    }

    /// Stable key of the variant (locale keys).
    pub const fn key(&self) -> &'static str {
        match self {
            Self::Post { .. } => "post",
            Self::Thread { .. } => "thread",
            Self::Document { .. } => "document",
            Self::Item { .. } => "item",
            Self::Concept { .. } => "concept",
        }
    }
}

/// One step towards a goal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalTask {
    pub id: String,
    pub text: String,
    pub done: bool,
    pub added_on: Date,
    #[serde(default)]
    pub done_on: Option<Date>,
}

/// A learning goal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
    pub network_id: NetworkId,
    pub title: String,
    /// Why it matters to the learner.
    pub why: String,
    pub target: Date,
    pub links: Vec<GoalLink>,
    pub tasks: Vec<GoalTask>,
    /// The learner marked it reached.
    #[serde(default)]
    pub completed: bool,
    pub created_on: Date,
    pub created_at: Timestamp,
}

impl Goal {
    /// The keys of the goal's concepts.
    pub fn concepts(&self) -> Vec<String> {
        self.links.iter().filter_map(|link| match link {
            GoalLink::Concept { name } => Some(name.clone()),
            _ => None,
        }).collect()
    }

    /// Whether `link` is already part of the goal.
    pub fn has_link(&self, link: &GoalLink) -> bool {
        self.links.contains(link)
    }
}
