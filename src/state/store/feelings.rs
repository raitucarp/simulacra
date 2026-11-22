//! Posting feelings (M1): one post of kind [`PostKind::Feeling`] whose text is its emojis and
//! note, so search and the reply context read it as written.

use super::{AppState, ThreadAnchor, ThreadDraft};
use crate::domain::post::PostKind;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};

impl AppState {
    /// Publishes `feeling` (a [`PostKind::Feeling`]) as the current profile, replying to or
    /// quoting `anchor`'s post when given.
    pub fn submit_feeling(&mut self, feeling: PostKind, anchor: &ThreadAnchor) -> Result<PostId> {
        let Some(text) = feeling.feeling_text() else { return Err(AppError::EmptyContent) };
        let mut posts = self.build_thread(&[ThreadDraft::text(text)], anchor)?;
        let post = Post { kind: feeling, ..posts.remove(0) };
        let id = post.id.clone();
        self.commit_post(post)?;
        Ok(id)
    }
}
