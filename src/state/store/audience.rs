//! N8: audience scopes in the store. The user sets who may read one of their posts; every
//! path that hands posts to an agent asks [`AppState::may_read`] first. People (the user's
//! personas) are never kept out; agents read only what is in their scope and never a locked post.

use super::{require_persona, AppState};
use crate::domain::post::audience::{readable_by, Audience};
use crate::domain::{Post, PostId, ProfileId};
use crate::error::{AppError, Result};

impl AppState {
    /// Sets who may read the user's own post `id` (`None`: everyone). `Ok(false)` when it
    /// already had that scope.
    pub fn set_audience(&mut self, id: &PostId, audience: Option<Audience>) -> Result<bool> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        if self.posts[position].extras.audience.as_deref() == audience.as_ref() {
            return Ok(false);
        }
        self.posts[position].extras.audience = audience.map(Box::new);
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(true)
    }

    /// Whether the profile `reader` may read `post`: one of the user's personas always; an
    /// agent (or a profile the store does not know) only within the post's scope, never a
    /// locked post.
    pub fn may_read(&self, reader: &ProfileId, post: &Post) -> bool {
        match self.user_profiles.iter().find(|profile| &profile.id == reader) {
            Some(profile) if !profile.is_agent() => true,
            _ => readable_by(post, reader),
        }
    }

    /// The agents of the active network, by name (the ones a post can be shared with).
    pub fn audience_agents(&self) -> Vec<(ProfileId, String)> {
        let mut agents: Vec<_> = self.current_network_agents().into_iter().map(|agent| (agent.id.clone(), agent.name.clone())).collect();
        agents.sort_by_key(|(_, name)| name.to_lowercase());
        agents
    }
}
