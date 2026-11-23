//! N11, N12: posts made of a voice note or an ink drawing, published by the current profile
//! with an optional caption. Both count as attachments: such a post is never locked.

use super::AppState;
use crate::domain::post::voice::VoiceNote;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};

impl AppState {
    /// Publishes `caption` (may be empty) with the post finished by `finish`.
    fn submit_media_post(&mut self, caption: &str, finish: impl FnOnce(&mut Post)) -> Result<PostId> {
        let author = self.current_profile_or_err()?;
        let id = Self::new_post_id();
        let mut post = Post::create_with_files(id.clone(), self.active_network_id.clone(), author, caption, None, None, None, None, None, true)?;
        finish(&mut post);
        self.commit_post(post)?;
        Ok(id)
    }

    /// N11: publishes the voice note `note` (its file already in the media folder) with `caption`.
    pub fn submit_voice_post(&mut self, caption: &str, note: VoiceNote) -> Result<PostId> {
        if note.path.trim().is_empty() {
            return Err(AppError::EmptyContent);
        }
        self.submit_media_post(caption, |post| post.extras.voice = Some(Box::new(note)))
    }
}
