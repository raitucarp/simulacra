//! Delegated to-dos in the store (D12): which items of the user's posts are handed to one of
//! the post's network agents and still wait for a progress reply, and remembering that one
//! was asked for (kept in the item's tick, without a new version).

use super::{require_persona, AppState};
use crate::domain::profile::handle_key;
use crate::domain::todo::delegate::awaiting_progress;
use crate::domain::todo::entries::todo_entries;
use crate::domain::{PostId, Profile};
use crate::error::{AppError, Result};

/// A progress reply to ask `agent` for.
#[derive(Debug, Clone, PartialEq)]
pub struct DelegationJob {
    pub post_id: PostId,
    /// The item's place among the post's to-dos.
    pub index: usize,
    pub item: String,
    pub agent: Profile,
}

impl AppState {
    /// Every delegated item of the user's posts whose `@name` is an agent of the post's
    /// network and whose progress reply was not asked for yet, newest post first.
    pub fn pending_delegations(&self) -> Vec<DelegationJob> {
        let mut jobs = Vec::new();
        for post in self.posts.iter().filter(|post| self.author_of(post).is_human_persona()) {
            let agents = self.agents_for_network(&post.network_id);
            for delegation in awaiting_progress(&todo_entries(&post.content, &post.extras.todo_state)) {
                let key = handle_key(&delegation.delegate);
                if let Some(agent) = agents.iter().find(|agent| handle_key(&agent.handle) == key) {
                    jobs.push(DelegationJob { post_id: post.id.clone(), index: delegation.index, item: delegation.text, agent: (*agent).clone() });
                }
            }
        }
        jobs
    }

    /// Remembers that the progress reply on the `index`-th to-do of `id` was asked for.
    pub fn mark_progress_asked(&mut self, id: &PostId, index: usize) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        require_persona(self.author_of(&self.posts[position]))?;
        let post = &mut self.posts[position];
        let Some(entry) = todo_entries(&post.content, &post.extras.todo_state).into_iter().nth(index) else { return Ok(()) };
        post.extras.todo_state.tick_mut(index, &entry.item).progress_asked = true;
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(())
    }
}
