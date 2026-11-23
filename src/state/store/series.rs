//! N10: series in the store. The user adds one of their posts to a series (by title): it gets
//! the next number, and the series' goal when it has one. Every post of a series knows its
//! previous and next part and how far the series has come; the series of a network are
//! listed with their parts in order. A locked part keeps its place but shows only its
//! placeholder wherever its text would be.

use super::{require_persona, AppState};
use crate::domain::post::series::{series_id, series_title, SeriesEntry};
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};

/// A post's place in its series.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeriesNav {
    pub entry: SeriesEntry,
    /// Its position among the parts posted (from 1) and how many there are.
    pub position: usize,
    pub posted: usize,
    pub previous: Option<PostId>,
    pub next: Option<PostId>,
}

/// One series and its parts, in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeriesListing {
    pub id: String,
    pub title: String,
    pub goal: Option<u32>,
    pub parts: Vec<PostId>,
}

impl AppState {
    /// Adds the user's own post `id` to the series titled `title` as its next part (`goal`:
    /// how many parts it is meant to have; kept from the series when `None`).
    pub fn add_to_series(&mut self, id: &PostId, title: &str, goal: Option<u32>) -> Result<SeriesEntry> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        let title = series_title(title).ok_or(AppError::EmptyContent)?;
        let series = series_id(&title);
        let others: Vec<&SeriesEntry> = self.posts.iter().filter(|post| &post.id != id).filter_map(|post| post.extras.series.as_deref()).filter(|entry| entry.id == series).collect();
        let number = others.iter().map(|entry| entry.number).max().unwrap_or_default() + 1;
        let goal = goal.or_else(|| others.iter().find_map(|entry| entry.goal));
        let entry = SeriesEntry { id: series, title, number, goal };
        self.set_series(position, Some(entry.clone()))?;
        Ok(entry)
    }

    /// Takes the user's own post `id` out of its series (the others keep their numbers).
    pub fn leave_series(&mut self, id: &PostId) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        self.set_series(position, None)
    }

    fn set_series(&mut self, position: usize, entry: Option<SeriesEntry>) -> Result<()> {
        self.posts[position].extras.series = entry.map(Box::new);
        match &self.db {
            Some(db) => db.save_post(&self.posts[position]),
            None => Ok(()),
        }
    }

    /// The parts of the series `id` in `post`'s network, by number (then age).
    fn parts_of(&self, network: &crate::domain::NetworkId, id: &str) -> Vec<&Post> {
        let mut parts: Vec<&Post> = self.posts.iter().filter(|post| &post.network_id == network && post.extras.series.as_ref().is_some_and(|entry| entry.id == id)).collect();
        parts.sort_by_key(|post| (post.extras.series.as_ref().map_or(0, |entry| entry.number), post.created_at));
        parts
    }

    /// `post`'s place in its series (`None` outside one).
    pub fn series_nav(&self, post: &Post) -> Option<SeriesNav> {
        let entry = post.extras.series.as_deref().cloned()?;
        let parts = self.parts_of(&post.network_id, &entry.id);
        let at = parts.iter().position(|part| part.id == post.id)?;
        let id_at = |at: Option<usize>| at.and_then(|at| parts.get(at)).map(|part| part.id.clone());
        Some(SeriesNav { entry, position: at + 1, posted: parts.len(), previous: id_at(at.checked_sub(1)), next: id_at(Some(at + 1)) })
    }

    /// The series of the active network, most recently added to first.
    pub fn series_listings(&self) -> Vec<SeriesListing> {
        let mut ids: Vec<&SeriesEntry> = Vec::new();
        for entry in self.current_network_posts().filter_map(|post| post.extras.series.as_deref()) {
            if !ids.iter().any(|known| known.id == entry.id) {
                ids.push(entry);
            }
        }
        let network = self.active_network_id.clone();
        ids.into_iter()
            .map(|entry| {
                let parts = self.parts_of(&network, &entry.id);
                let goal = parts.iter().rev().find_map(|part| part.extras.series.as_ref()?.goal);
                SeriesListing { id: entry.id.clone(), title: entry.title.clone(), goal, parts: parts.into_iter().map(|part| part.id.clone()).collect() }
            })
            .collect()
    }
}
