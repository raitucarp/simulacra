//! N14: retractions and corrections in the store. The user marks one of their posts retracted
//! or corrected with a reason (never a locked one), or withdraws the mark.

use super::{require_persona, AppState};
use crate::domain::post::correction::{clean_reason, Correction, CorrectionKind};
use crate::domain::PostId;
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    fn own_position(&self, id: &PostId) -> Result<usize> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        Ok(position)
    }

    fn save_correction(&mut self, position: usize, correction: Option<Correction>) -> Result<()> {
        self.posts[position].extras.correction = correction.map(Box::new);
        match &self.db {
            Some(db) => db.save_post(&self.posts[position]),
            None => Ok(()),
        }
    }

    /// Marks the user's post `id` retracted or corrected because of `reason` (agents already
    /// told stay told).
    pub fn mark_correction(&mut self, id: &PostId, kind: CorrectionKind, reason: &str, now: Timestamp) -> Result<Correction> {
        let position = self.own_position(id)?;
        if self.posts[position].is_locked() {
            return Err(AppError::PostLocked);
        }
        let reason = clean_reason(reason).ok_or(AppError::EmptyContent)?;
        let told = self.posts[position].extras.correction.as_ref().map(|c| c.told.clone()).unwrap_or_default();
        let correction = Correction { kind, reason, at: now, told };
        self.save_correction(position, Some(correction.clone()))?;
        Ok(correction)
    }

    /// Takes the mark off the user's post `id`.
    pub fn withdraw_correction(&mut self, id: &PostId) -> Result<()> {
        let position = self.own_position(id)?;
        self.save_correction(position, None)
    }
}
