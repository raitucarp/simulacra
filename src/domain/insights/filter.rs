//! Which posts the Insights page looks at: a date range, the audience (agents or personas),
//! chosen authors and post kinds, and whether every network or only the active one counts.

use super::facts::PostKind;
use crate::domain::{NetworkId, Post, Profile};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// How far back posts are counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DateRange {
    Week,
    #[default]
    Month,
    Quarter,
    Year,
    All,
}

impl DateRange {
    pub const ALL: [Self; 5] = [Self::Week, Self::Month, Self::Quarter, Self::Year, Self::All];

    /// Length in days (`None` for everything).
    pub const fn days(self) -> Option<i64> {
        match self {
            Self::Week => Some(7),
            Self::Month => Some(30),
            Self::Quarter => Some(90),
            Self::Year => Some(365),
            Self::All => None,
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Week => "week",
            Self::Month => "month",
            Self::Quarter => "quarter",
            Self::Year => "year",
            Self::All => "all",
        }
    }
}

/// Whose posts count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
    #[default]
    Everyone,
    Agents,
    Personas,
}

impl Audience {
    pub const ALL: [Self; 3] = [Self::Everyone, Self::Agents, Self::Personas];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Everyone => "everyone",
            Self::Agents => "agents",
            Self::Personas => "personas",
        }
    }
}

/// The page's filter (persisted with the page state).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InsightsFilter {
    pub range: DateRange,
    pub audience: Audience,
    /// Only these authors (profile ids); empty means everyone.
    pub authors: Vec<String>,
    /// Kinds left out.
    pub hidden_kinds: Vec<PostKind>,
    /// Every network instead of the active one.
    pub all_networks: bool,
}

impl InsightsFilter {
    /// Whether `post` (by `author`, if known) passes, at time `now` in network `network`.
    pub fn accepts(&self, post: &Post, author: Option<&Profile>, now: DateTime<Utc>, network: &NetworkId) -> bool {
        let in_network = self.all_networks || &post.network_id == network;
        let recent = self.range.days().is_none_or(|days| post.created_at >= now - Duration::days(days));
        let is_agent = author.is_some_and(Profile::is_agent);
        let audience = match self.audience {
            Audience::Everyone => true,
            Audience::Agents => is_agent,
            Audience::Personas => !is_agent,
        };
        let chosen = self.authors.is_empty() || self.authors.iter().any(|id| id == post.author_id.as_str());
        in_network && recent && audience && chosen && !self.hidden_kinds.contains(&PostKind::of(post))
    }

    /// Toggles `kind` in or out.
    pub fn toggle_kind(&mut self, kind: PostKind) {
        match self.hidden_kinds.iter().position(|k| *k == kind) {
            Some(at) => {
                self.hidden_kinds.remove(at);
            }
            None => self.hidden_kinds.push(kind),
        }
    }

    /// Toggles author `id` in or out of the chosen authors.
    pub fn toggle_author(&mut self, id: &str) {
        match self.authors.iter().position(|a| a == id) {
            Some(at) => {
                self.authors.remove(at);
            }
            None => self.authors.push(id.to_string()),
        }
    }
}
