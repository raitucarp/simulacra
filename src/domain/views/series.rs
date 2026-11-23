//! N10: the series view of a list of posts: each series among them, most recently added to
//! first, with its parts in order, how many there are (of the goal) and how far it has come.
//! A locked part keeps its number but shows only its placeholder.

use super::rapid_log::one_line;
use crate::domain::{Post, PostId};
use std::cmp::Reverse;

/// One part of a series as listed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeriesPart {
    pub post_id: PostId,
    pub number: u32,
    /// Its text on one line (empty when locked).
    pub text: String,
    pub locked: bool,
}

/// One series and its parts in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeriesGroup {
    pub id: String,
    pub title: String,
    pub goal: Option<u32>,
    pub parts: Vec<SeriesPart>,
}

/// The series of `posts`, the one added to most recently first.
pub fn series_groups<'a>(posts: impl IntoIterator<Item = &'a Post>) -> Vec<SeriesGroup> {
    let mut groups: Vec<(jiff::Timestamp, SeriesGroup)> = Vec::new();
    for post in posts {
        let Some(entry) = post.extras.series.as_deref() else { continue };
        let part = SeriesPart { post_id: post.id.clone(), number: entry.number, text: one_line(post), locked: post.is_locked() };
        match groups.iter_mut().find(|(_, group)| group.id == entry.id) {
            Some((latest, group)) => {
                *latest = (*latest).max(post.created_at);
                group.goal = group.goal.or(entry.goal);
                group.parts.push(part);
            }
            None => groups.push((post.created_at, SeriesGroup { id: entry.id.clone(), title: entry.title.clone(), goal: entry.goal, parts: vec![part] })),
        }
    }
    groups.sort_by_key(|(latest, _)| Reverse(*latest));
    groups.into_iter().map(|(_, mut group)| {
        group.parts.sort_by_key(|part| part.number);
        group
    }).collect()
}
