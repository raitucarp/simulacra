//! N11, N12: posts made of a voice note or an ink drawing, published by the current profile
//! with an optional caption. Both count as attachments: such a post is never locked.

use super::AppState;
use crate::domain::post::voice::{clean_transcript, VoiceNote};
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

    /// N11: keeps `raw` (cleaned) as the transcript of the voice note on the user's post `id`;
    /// never on a locked post.
    pub fn set_transcript(&mut self, id: &PostId, raw: &str) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        super::require_persona(self.author_of(&self.posts[position]))?;
        if self.posts[position].is_locked() {
            return Err(AppError::PostLocked);
        }
        let transcript = clean_transcript(raw).ok_or(AppError::EmptyContent)?;
        let voice = self.posts[position].extras.voice.as_mut().ok_or(AppError::EmptyContent)?;
        voice.transcript = Some(transcript);
        match &self.db {
            Some(db) => db.save_post(&self.posts[position]),
            None => Ok(()),
        }
    }
}
