//! Where an edit of a post starts (V4) and switching which version a post shows (V9): the
//! head moves to any kept version, and the version it left is kept in its place, so no
//! version is ever lost and none is copied. A version brings back its symbols and tint (V10).

use super::{require_persona, AppState};
use crate::domain::data::history::{edited_post, EditBase, PostRecord, RevisionId, RevisionSubject, VersionHead};
use crate::domain::PostId;
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    /// What an edit of the post `id` starts from: the version shown, or the older version
    /// `from` (V4); `None` when either is unknown.
    pub fn edit_base(&self, id: &PostId, from: Option<&RevisionId>) -> Option<EditBase> {
        let post = self.get_post(id)?;
        let revisions = self.post_revisions(id);
        let head = VersionHead::of(post.extras.head.as_ref(), &revisions, post.created_at);
        let taken = revisions.iter().filter_map(|r| r.version).chain([head.version]).collect();
        let (text, version, revision, record) = match from {
            None => (post.content.clone(), head.version, None, Some(PostRecord::of(post))),
            Some(from) => revisions.iter().find(|r| &r.id == from).map(|r| (r.content.clone(), r.version.unwrap_or_default(), Some(r.id.clone()), PostRecord::parse(&r.record_json)))?,
        };
        Some(EditBase { text, version, revision, taken, record })
    }

    /// Shows the kept version `to` of the user's own post `id`, keeping the version it showed.
    /// Returns the version now shown.
    pub fn switch_post_head(&mut self, id: &PostId, to: &RevisionId, now: Timestamp) -> Result<VersionHead> {
        let post = self.get_post(id).cloned().ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&post))?;
        let revisions = self.post_revisions(id);
        let target = revisions.iter().find(|r| &r.id == to).ok_or_else(|| AppError::DatabaseError(format!("version '{}' not found", to.as_str())))?;
        let head = VersionHead::of(post.extras.head.as_ref(), &revisions, post.created_at);
        let kept = crate::domain::data::history::Revision { record_json: PostRecord::of(&post).to_json(), ..head.into_revision(RevisionSubject::Post(id.clone()), &post.content, now) };
        let shown = VersionHead::from_revision(target);
        let mut switched = edited_post(&post, &target.content);
        if let Some(record) = PostRecord::parse(&target.record_json) {
            record.apply(&mut switched);
        }
        switched.extras.head = Some(shown.clone());
        switched.extras.todo_state = post.extras.todo_state.carried(&post.content, &switched.content); // D6
        if let Some(ref db) = self.db {
            db.commit_post_version(switched.clone(), kept, Some(target.id.clone()))?;
        }
        self.replace_post_body(switched);
        Ok(shown)
    }

    /// `post` noting the version of the post it replies to (V12): the version that post shows
    /// now, `1.0.0` for one never edited. Unchanged when it is no reply, already notes one, or
    /// its parent is unknown.
    pub(super) fn with_replied_version(&self, mut post: crate::domain::Post) -> crate::domain::Post {
        let parent = post.reply_to_id.as_ref().and_then(|id| self.get_post(id));
        if let (None, Some(parent)) = (post.extras.replied_version, parent) {
            post.extras.replied_version = Some(parent.extras.head.as_ref().map_or_else(Default::default, |head| head.version));
        }
        post
    }
}

impl AppState {
    /// `content` with its embeds written without a version pinned to the version each
    /// embedded post shows now (N1); embeds of unknown posts stay unpinned.
    pub(super) fn pinned_embeds(&self, content: &str) -> String {
        crate::domain::post::pin_embeds(content, |id| self.get_post_by_id(id).map(|post| post.extras.head.as_ref().map_or_else(Default::default, |head| head.version)))
    }

    /// `post` with its embeds pinned (N1). A locked post holds only its placeholder.
    pub(super) fn with_pinned_embeds(&self, mut post: crate::domain::Post) -> crate::domain::Post {
        post.content = self.pinned_embeds(&post.content);
        post
    }
}
