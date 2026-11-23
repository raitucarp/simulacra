//! N3: the garden in the store. The user promotes one of their posts (with the thread under
//! it) to a garden note: it leaves the stream, but search, threads, links and its profile
//! still find it. Tending moves it between seed, budding and evergreen; returning it to the
//! stream drops the note and keeps the post.

use super::{require_persona, AppState};
use crate::domain::post::garden::{GardenNote, Maturity};
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

/// How deep a reply chain is followed up to its garden note.
const MAX_THREAD_DEPTH: usize = 256;

impl AppState {
    /// Plants the user's own post `id` in the garden as a seed (`Some`), tends it to `maturity`,
    /// or returns it to the stream (`None`). `Ok(false)` when nothing changed.
    pub fn set_garden(&mut self, id: &PostId, maturity: Option<Maturity>, now: Timestamp) -> Result<bool> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        let current = self.posts[position].extras.garden;
        let next = match (current, maturity) {
            (_, None) => None,
            (None, Some(stage)) => Some(GardenNote { maturity: stage, ..GardenNote::planted(now) }),
            (Some(note), Some(stage)) => Some(note.tended(stage, now)),
        };
        if next == current {
            return Ok(false);
        }
        self.posts[position].extras.garden = next;
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        self.index.listing_changed();
        Ok(true)
    }

    /// The garden note `post` belongs to: its own, or the nearest one above it in its thread.
    pub fn garden_of<'a>(&'a self, post: &'a Post) -> Option<(&'a Post, GardenNote)> {
        let mut at = post;
        for _ in 0..MAX_THREAD_DEPTH {
            if let Some(note) = at.extras.garden {
                return Some((at, note));
            }
            at = self.get_post(at.reply_to_id.as_ref()?)?;
        }
        None
    }

    /// Whether `post` lives in the garden, outside the stream (N3).
    pub fn in_garden(&self, post: &Post) -> bool {
        self.garden_of(post).is_some()
    }
}
