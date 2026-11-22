//! L11: pointing an attached file at where it is now. A moved or renamed file is relinked in
//! place (saved with the post, never a new version: the post says the same thing).

use super::{require_persona, AppState};
use crate::domain::media::FileRef;
use crate::domain::PostId;
use crate::error::{AppError, Result};

impl AppState {
    /// Points the `index`-th file of the user's own post `id` at `file`.
    pub fn relink_file(&mut self, id: &PostId, index: usize, file: FileRef) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post {id} not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        let post = &mut self.posts[position];
        let slot = post.extras.files.get_mut(index).ok_or_else(|| AppError::DatabaseError(format!("post {id} has no file {index}")))?;
        *slot = file;
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(())
    }
}
