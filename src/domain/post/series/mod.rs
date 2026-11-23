//! N10: series. Numbered posts across days ("Learning Rust, day 12"): each post of a series
//! keeps a [`SeriesEntry`] (`extras.series`) with the series' id and title, its number and,
//! when the series has one, how many parts it is meant to have. The store reads previous,
//! next and progress from the entries; a locked entry shows only its placeholder.

use crate::domain::embedding::content_hash;
use serde::{Deserialize, Serialize};

/// The longest series title, in characters.
pub const MAX_SERIES_TITLE_CHARS: usize = 60;
/// The most parts a series can be meant to have.
pub const MAX_SERIES_GOAL: u32 = 1000;
/// Prefix of a series id.
const SERIES_ID_PREFIX: &str = "series_";

/// A post's place in a series (`extras.series`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SeriesEntry {
    /// The series' id: the same for every post of it.
    pub id: String,
    pub title: String,
    /// Its number in the series, from 1.
    pub number: u32,
    /// How many parts the series is meant to have (`None`: open-ended).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<u32>,
}

/// `title` on one line with spaces collapsed; `None` when empty or too long.
pub fn series_title(title: &str) -> Option<String> {
    let title = title.split_whitespace().collect::<Vec<_>>().join(" ");
    let count = title.chars().count();
    (count > 0 && count <= MAX_SERIES_TITLE_CHARS).then_some(title)
}

/// The id of the series titled `title` (the same title, in any case, is the same series).
pub fn series_id(title: &str) -> String {
    format!("{SERIES_ID_PREFIX}{}", content_hash(&[&title.to_lowercase()]))
}

/// A goal as typed: a whole number from 1 to [`MAX_SERIES_GOAL`] (`None` otherwise).
pub fn parse_goal(text: &str) -> Option<u32> {
    text.trim().parse::<u32>().ok().filter(|goal| (1..=MAX_SERIES_GOAL).contains(goal))
}

/// How far a series has come: the parts posted over the parts meant (or posted, open-ended),
/// in `[0, 1]`.
pub fn progress(posted: usize, goal: Option<u32>) -> f32 {
    let of = goal.map_or(posted, |goal| goal as usize).max(1);
    (posted as f32 / of as f32).min(1.0)
}

#[cfg(test)]
mod tests;
