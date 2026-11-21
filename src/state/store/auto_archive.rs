//! Auto-archive (Y6): a post moves to the archive a chosen time after it went out. The
//! user's default (Writing › Archive posts automatically) is stamped on the posts their
//! personas publish; each post can change it. The lifecycle heartbeat archives what is due.

use super::{require_persona, AppState};
use crate::domain::post::PostStatus;
use crate::domain::preferences::ArchiveAfter;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    /// `post` with the default auto-archive moment, when one of the user's personas wrote it
    /// and it has none yet.
    pub(super) fn with_default_archive(&self, mut post: Post) -> Post {
        if post.lifecycle.archive_at.is_none() && self.author_of(&post).is_human_persona() {
            post.lifecycle.archive_at = self.preferences.writing.auto_archive.archive_at(post.created_at);
        }
        post
    }

    /// Archives the user's own post `id` `after` it went out (`Never` stops it).
    pub fn set_auto_archive(&mut self, id: &PostId, after: ArchiveAfter) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        let post = &mut self.posts[position];
        post.lifecycle.archive_at = after.archive_at(post.created_at);
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(())
    }

    /// Archives every published post whose moment has come at `now`, newest first.
    pub fn archive_due(&mut self, now: Timestamp) -> Result<Vec<PostId>> {
        let due = |post: &Post| post.lifecycle.status == PostStatus::Published && post.lifecycle.archive_at.is_some_and(|at| at <= now);
        let positions: Vec<usize> = self.posts.iter().enumerate().filter(|(_, post)| due(post)).map(|(at, _)| at).collect();
        let mut archived = Vec::with_capacity(positions.len());
        for position in positions {
            let post = &mut self.posts[position];
            post.lifecycle.status = PostStatus::Archived;
            post.lifecycle.archive_at = None;
            if let Some(ref db) = self.db {
                db.save_post(&self.posts[position])?;
            }
            archived.push(self.posts[position].id.clone());
        }
        if !archived.is_empty() {
            self.index.listing_changed();
        }
        Ok(archived)
    }
}
