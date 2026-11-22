//! Publishing password-locked threads (X2) in three steps, so the slow key derivation can
//! run off the UI thread: [`AppState::build_locked_thread`] validates the drafts (text only,
//! personas only), [`seal_thread`] seals every post, and [`AppState::publish_locked`] stores
//! them, refusing any post that still holds its text.

use super::{require_persona, AppState, ThreadAnchor, ThreadDraft};
use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::post::PostLifecycle;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

/// Seals the text of every post of a built thread under `password`.
pub fn seal_thread(posts: Vec<Post>, password: &LockPassword, kdf: KdfParams) -> Result<Vec<Post>> {
    posts.into_iter().map(|mut post| post.lock_with(password, kdf).map(|()| post)).collect()
}

impl AppState {
    /// Validates `drafts` as [`AppState::submit_thread`] does, for locking: the current
    /// profile must be a persona and the drafts must carry text only. Nothing is stored.
    pub fn build_locked_thread(&self, drafts: &[ThreadDraft], anchor: &ThreadAnchor) -> Result<Vec<Post>> {
        require_persona(&self.current_profile_or_err()?)?;
        let posts = self.build_thread(drafts, anchor)?;
        if posts.iter().any(Post::has_attachments) {
            return Err(AppError::LockedWithAttachments);
        }
        Ok(posts)
    }

    /// Stores sealed `posts` (from [`seal_thread`]): published at once, or scheduled for
    /// `publish_at`, which must then be after `now`. Returns their ids in thread order.
    pub fn publish_locked(&mut self, posts: Vec<Post>, publish_at: Option<Timestamp>, now: Timestamp) -> Result<Vec<PostId>> {
        if posts.is_empty() || posts.iter().any(|post| !post.is_locked() || !post.is_sealed_or_plain()) {
            return Err(AppError::LockFailed);
        }
        if publish_at.is_some_and(|at| at <= now) {
            return Err(AppError::ScheduleNotInFuture);
        }
        let ids = posts.iter().map(|post| post.id.clone()).collect();
        for post in posts {
            match publish_at {
                Some(at) => self.hold_scheduled(Post { lifecycle: PostLifecycle::scheduled(at), ..post })?,
                None => self.commit_post(post)?,
            }
        }
        self.draft_text.clear();
        Ok(ids)
    }

    /// Builds, seals and publishes a locked thread in one go (tests and small callers; the
    /// composer seals off the UI thread instead).
    pub fn submit_locked_thread(&mut self, drafts: &[ThreadDraft], anchor: &ThreadAnchor, password: &LockPassword, kdf: KdfParams) -> Result<Vec<PostId>> {
        let sealed = seal_thread(self.build_locked_thread(drafts, anchor)?, password, kdf)?;
        self.publish_locked(sealed, None, Timestamp::now())
    }
}
