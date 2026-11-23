//! N14: retractions and corrections in the store. The user marks one of their posts retracted
//! or corrected with a reason (never a locked one), or withdraws the mark. The agents that
//! cited the post (replied to, quoted, reposted, linked or used it as a source) and may read
//! it are the ones told; each is remembered once told.

use super::{require_persona, AppState};
use crate::domain::post::correction::{clean_reason, Correction, CorrectionKind};
use crate::domain::post::SourceKind;
use crate::domain::{Post, PostId, Profile, ProfileId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

/// Whether `post` cites the post `id`.
fn cites(post: &Post, id: &PostId) -> bool {
    [&post.reply_to_id, &post.quote_id, &post.repost_id].into_iter().any(|target| target.as_ref() == Some(id))
        || post.linked_posts.contains(id)
        || post.extras.sources.iter().any(|source| source.kind == SourceKind::Post && source.id == id.as_str())
}

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

    /// The agents that cited the post `id`, may read it and were not told of its mark yet.
    pub fn citers_to_tell(&self, id: &PostId) -> Vec<&Profile> {
        let Some(post) = self.get_post(id) else { return Vec::new() };
        let told: &[ProfileId] = post.extras.correction.as_ref().map_or(&[], |c| c.told.as_slice());
        let citing: Vec<&ProfileId> = self.posts.iter().filter(|other| other.id != *id && cites(other, id)).map(|other| &other.author_id).collect();
        self.user_profiles.iter().filter(|p| p.is_agent() && citing.contains(&&p.id) && !told.contains(&p.id) && self.may_read(&p.id, post)).collect()
    }

    /// Remembers that `agents` were told of the mark on post `id`.
    pub fn record_told(&mut self, id: &PostId, agents: &[ProfileId]) -> Result<()> {
        let position = self.own_position(id)?;
        let Some(mut correction) = self.posts[position].extras.correction.as_deref().cloned() else { return Ok(()) };
        for agent in agents {
            if !correction.told.contains(agent) {
                correction.told.push(agent.clone());
            }
        }
        self.save_correction(position, Some(correction))
    }
}
