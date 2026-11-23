//! N5: renderings in the store. A rendering of a post is kept as a version edited from the
//! version the post shows (a patch numbered among the post's versions), its record naming
//! the depth; the post keeps showing what it showed. A locked post is never rendered.

use super::AppState;
use crate::domain::data::history::{free_version, Bump, PostRecord, Revision, RevisionId, RevisionSubject, SemVer, VersionHead};
use crate::domain::post::depth::Depth;
use crate::domain::PostId;
use crate::error::{AppError, Result};
use jiff::Timestamp;

fn missing(id: &PostId) -> AppError {
    AppError::DatabaseError(format!("post '{id}' not found"))
}

impl AppState {
    /// Keeps `text` as the `depth` rendering of the version the post `id` shows. The post
    /// gets a stored head first when it had none, so the rendering has a version to hang from.
    pub fn keep_rendering(&mut self, id: &PostId, depth: Depth, text: &str, now: Timestamp) -> Result<Revision> {
        let post = self.get_post(id).cloned().ok_or_else(|| missing(id))?;
        if post.is_locked() {
            return Err(AppError::PostLocked); // X2: a sealed text is never rendered
        }
        let text = text.trim();
        if text.is_empty() {
            return Err(AppError::EmptyContent);
        }
        let revisions = self.post_revisions(id);
        let head = VersionHead::of(post.extras.head.as_ref(), &revisions, post.created_at);
        if post.extras.head.is_none() {
            self.store_head(id, head.clone())?;
        }
        let taken: Vec<SemVer> = revisions.iter().filter_map(|r| r.version).chain([head.version]).collect();
        let record = PostRecord { rendering: Some(depth), ..PostRecord::of(&post) };
        let rendering = Revision { id: RevisionId::generate(), parent_id: Some(head.id), version: Some(free_version(head.version, Bump::Patch, &taken)), bump: Bump::Patch, edited_at: Some(now), record_json: record.to_json(), ..Revision::of(RevisionSubject::Post(id.clone()), text, now) };
        if let Some(ref db) = self.db {
            db.save_revision(rendering.clone())?;
        }
        self.index.versions.forget(id);
        Ok(rendering)
    }

    /// The newest `depth` rendering of the version the post `id` shows, if any.
    pub fn rendering_of(&self, id: &PostId, depth: Depth) -> Option<Revision> {
        let head = self.get_post(id)?.extras.head.clone()?;
        let is_rendering = |r: &Revision| r.parent_id.as_ref() == Some(&head.id) && PostRecord::parse(&r.record_json).and_then(|record| record.rendering) == Some(depth);
        self.post_revisions(id).into_iter().filter(is_rendering).max_by_key(|r| r.edited_at)
    }

    /// Stores `head` as the version the post `id` shows (its first stored head).
    fn store_head(&mut self, id: &PostId, head: VersionHead) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| missing(id))?;
        self.posts[position].extras.head = Some(head);
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(())
    }
}
