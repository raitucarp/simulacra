//! N20: bilingual posts in the store. A post (anyone's, never a locked one) keeps its text in
//! the language the user is learning, or lets it go.

use super::AppState;
use crate::domain::post::translation::prompt::clean_translation;
use crate::domain::post::translation::Translation;
use crate::domain::PostId;
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    fn save_translation(&mut self, id: &PostId, translation: Option<Translation>) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        if self.posts[position].is_locked() {
            return Err(AppError::PostLocked); // X2: a translation would speak past the lock
        }
        self.posts[position].extras.translation = translation.map(Box::new);
        match &self.db {
            Some(db) => db.save_post(&self.posts[position]),
            None => Ok(()),
        }
    }

    /// Keeps `text` as post `id` in `language` (a locale code).
    pub fn keep_translation(&mut self, id: &PostId, language: &str, text: &str, now: Timestamp) -> Result<Translation> {
        let text = clean_translation(text).ok_or(AppError::EmptyContent)?;
        let translation = Translation { language: language.to_string(), text, at: now };
        self.save_translation(id, Some(translation.clone()))?;
        Ok(translation)
    }

    /// Lets post `id`'s translation go.
    pub fn drop_translation(&mut self, id: &PostId) -> Result<()> {
        self.save_translation(id, None)
    }
}
