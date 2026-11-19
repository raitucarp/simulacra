//! Shares of a dimension: how many items each source has (authors by name; stances and
//! schools by their locale key, so the view translates them), most first, with the variety
//! of the whole distribution.

use super::measures::{variety, Variety};
use super::ProfileMap;
use crate::domain::insights::lenses::Analysed;
use crate::domain::trait_option::TraitOption;
use crate::domain::{Post, Profile};
use std::collections::BTreeMap;

/// What the items are grouped by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Dimension {
    #[default]
    Author,
    /// The epistemic stance of agent authors.
    Stance,
    /// The stylistic school of agent authors.
    School,
    /// Vault subjects of the documents being read.
    Subject,
    /// Creators of the documents being read.
    Creator,
}

impl Dimension {
    /// The dimensions of the feed, in display order.
    pub const FEED: [Self; 3] = [Self::Author, Self::Stance, Self::School];

    /// Stable key (locale keys `insight.dimension.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Author => "author",
            Self::Stance => "stance",
            Self::School => "school",
            Self::Subject => "subject",
            Self::Creator => "creator",
        }
    }
}

/// How a source is named.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShareLabel {
    /// A name shown as is.
    Name(String),
    /// A locale key the view translates.
    Key(String),
}

/// One source and its items.
#[derive(Debug, Clone, PartialEq)]
pub struct Share {
    pub key: String,
    pub label: ShareLabel,
    pub count: usize,
}

/// Every source of one dimension, most items first (ties by key).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Shares {
    pub dimension: Dimension,
    pub shares: Vec<Share>,
    pub variety: Variety,
}

impl Shares {
    /// Counts `items` (key and label per item).
    pub fn of(dimension: Dimension, items: impl Iterator<Item = (String, ShareLabel)>) -> Self {
        let mut grouped: BTreeMap<String, (ShareLabel, usize)> = BTreeMap::new();
        items.for_each(|(key, label)| grouped.entry(key).or_insert((label, 0)).1 += 1);
        let mut shares: Vec<Share> = grouped.into_iter().map(|(key, (label, count))| Share { key, label, count }).collect();
        shares.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.key.cmp(&b.key)));
        let counts: Vec<usize> = shares.iter().map(|s| s.count).collect();
        Self { dimension, variety: variety(&counts), shares }
    }

    /// Items per distinct key.
    pub fn counted(keys: Vec<String>) -> Vec<usize> {
        let mut grouped: BTreeMap<String, usize> = BTreeMap::new();
        keys.into_iter().for_each(|key| *grouped.entry(key).or_default() += 1);
        grouped.into_values().collect()
    }

    /// Share (0–1) of the source at `index`.
    pub fn share(&self, index: usize) -> f64 {
        let items = self.variety.items.max(1) as f64;
        self.shares.get(index).map_or(0.0, |s| s.count as f64 / items)
    }

    pub fn is_empty(&self) -> bool {
        self.shares.is_empty()
    }
}

/// The analysed posts by author.
pub fn authors(analysed: &Analysed) -> Shares {
    let items = analysed.author_of.iter().map(|&a| (analysed.authors[a].id.as_str().to_string(), ShareLabel::Name(analysed.authors[a].name.clone())));
    Shares::of(Dimension::Author, items)
}

fn agent_of<'a>(post: &Post, profiles: &ProfileMap<'a>) -> Option<&'a Profile> {
    profiles.get(post.author_id.as_str()).copied().filter(|p| p.is_agent())
}

/// The epistemic stance of an agent's post.
pub fn stance(post: &Post, profiles: &ProfileMap) -> Option<(String, ShareLabel)> {
    agent_of(post, profiles).map(|agent| agent.config.epistemic_stance).map(|s| (s.key().to_string(), ShareLabel::Key(s.label_key())))
}

/// The stylistic school of an agent's post.
pub fn school(post: &Post, profiles: &ProfileMap) -> Option<(String, ShareLabel)> {
    agent_of(post, profiles).map(|agent| agent.config.rhetoric.stylistic_school).map(|s| (s.key().to_string(), ShareLabel::Key(s.label_key())))
}
