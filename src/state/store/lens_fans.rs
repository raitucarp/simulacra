//! N16: lens fans in the store. A post (anyone's, never a locked one) is fanned out to agents
//! of its room (only agents that may read it, N8), each through one lens; the fan is recorded
//! on the post, and each line of it finds the reply that agent wrote through that lens. A
//! locked post lists no fan.

use super::AppState;
use crate::domain::post::lens::{FanEntry, FanLine, LensFan};
use crate::domain::{PostId, ProfileId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

impl AppState {
    /// Records that post `id` was fanned out to `entries` (one lens per agent, every agent in
    /// the post's room).
    pub fn fan_out(&mut self, id: &PostId, entries: Vec<FanEntry>, now: Timestamp) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        let post = &self.posts[position];
        if post.is_locked() {
            return Err(AppError::PostLocked); // X2: a sealed post is never sent
        }
        let room: Vec<&ProfileId> = self.room_for(post).into_iter().map(|agent| &agent.id).collect();
        let repeated = entries.iter().enumerate().any(|(at, entry)| entries[..at].iter().any(|earlier| earlier.agent == entry.agent));
        if entries.is_empty() || repeated || entries.iter().any(|entry| !room.contains(&&entry.agent)) {
            return Err(AppError::EmptyContent); // N8: nobody outside the post's scope is asked
        }
        self.posts[position].extras.fan = Some(Box::new(LensFan { entries, at: now }));
        match &self.db {
            Some(db) => db.save_post(&self.posts[position]),
            None => Ok(()),
        }
    }

    /// The fan of post `id`, lens by lens, each with the newest reply its agent wrote through
    /// it; empty for a post never fanned out, and for a locked one.
    pub fn fan_lines(&self, id: &PostId) -> Vec<FanLine> {
        let Some(fan) = self.get_post(id).filter(|post| !post.is_locked()).and_then(|post| post.extras.fan.as_deref()) else { return Vec::new() };
        let mut replies = self.replies_for_post(id);
        replies.sort_by_key(|reply| std::cmp::Reverse(reply.created_at));
        let reply_of = |entry: &FanEntry| replies.iter().find(|reply| reply.author_id == entry.agent && reply.extras.lens == Some(entry.lens)).map(|reply| reply.id.clone());
        fan.entries.iter().map(|entry| FanLine { lens: entry.lens, agent: entry.agent.clone(), reply: reply_of(entry) }).collect()
    }
}
