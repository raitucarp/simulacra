//! N9: ask the room in the store. The user asks one of their posts of the room: the agents it
//! goes to are recorded on it (only agents that may read it, N8); their direct replies are its
//! answers, ranked; the user accepts one (or takes it back). A locked post is never asked, and
//! a locked question lists no answers: its card shows only the placeholder.

use super::{require_persona, AppState};
use crate::domain::post::question::{rank_answers, Answer, Asked, Question};
use crate::domain::{Post, PostId, ProfileId};
use crate::error::{AppError, Result};

fn missing(id: &PostId) -> AppError {
    AppError::DatabaseError(format!("post '{id}' not found"))
}

impl AppState {
    /// The agents of the post's network that may read `post` (and so may be asked it).
    pub fn room_for(&self, post: &Post) -> Vec<&crate::domain::Profile> {
        self.user_profiles.iter().filter(|p| p.is_agent() && p.network_id.as_ref().is_none_or(|n| n == &post.network_id) && self.may_read(&p.id, post)).collect()
    }

    /// Records that the user's own post `id` was asked of `asked` (best fit first).
    pub fn ask_room(&mut self, id: &PostId, asked: Vec<Asked>) -> Result<()> {
        let position = self.post_position(id).ok_or_else(|| missing(id))?;
        let post = &self.posts[position];
        require_persona(self.author_of(post))?;
        if post.is_locked() {
            return Err(AppError::PostLocked); // X2: a sealed question is never sent
        }
        let room: Vec<&ProfileId> = self.room_for(post).into_iter().map(|agent| &agent.id).collect();
        if asked.is_empty() || asked.iter().any(|a| !room.contains(&&a.agent)) {
            return Err(AppError::EmptyContent); // N8: nobody outside the post's scope is asked
        }
        self.posts[position].extras.question = Some(Box::new(Question { asked, accepted: None }));
        self.save_at(position)
    }

    /// The answers to the question `id`, best first: the asked agents' direct replies, the
    /// accepted one at the top. None for a locked question or a post never asked.
    pub fn ranked_answers(&self, id: &PostId) -> Vec<Answer> {
        let Some(post) = self.get_post(id).filter(|post| !post.is_locked()) else { return Vec::new() };
        let Some(question) = post.extras.question.as_ref() else { return Vec::new() };
        let mut replies = self.replies_for_post(id);
        replies.sort_by_key(|reply| reply.created_at);
        let answers = replies
            .into_iter()
            .filter(|reply| question.asked.iter().any(|a| a.agent == reply.author_id))
            .map(|reply| Answer { post: reply.id.clone(), fit: question.fit_of(&reply.author_id), likes: reply.likes, reactions: self.reactions.of(&reply.id).map_or(0, |r| r.rows().len()) })
            .collect();
        rank_answers(answers, question.accepted.as_ref())
    }

    /// Accepts `answer` to the user's question `id`, or takes the acceptance back when it was
    /// the accepted one.
    pub fn accept_answer(&mut self, id: &PostId, answer: &PostId) -> Result<()> {
        if !self.ranked_answers(id).iter().any(|a| &a.post == answer) {
            return Err(AppError::EmptyContent);
        }
        let position = self.post_position(id).ok_or_else(|| missing(id))?;
        require_persona(self.author_of(&self.posts[position]))?;
        if let Some(question) = self.posts[position].extras.question.as_mut() {
            question.accepted = if question.accepted.as_ref() == Some(answer) { None } else { Some(answer.clone()) };
        }
        self.save_at(position)
    }

    fn save_at(&self, position: usize) -> Result<()> {
        match &self.db {
            Some(db) => db.save_post(&self.posts[position]),
            None => Ok(()),
        }
    }
}

/// What an agent knows about, in words (its name, role, occupation, bio, description and
/// topics), for routing a question before anything is embedded.
pub fn agent_knowledge(agent: &crate::domain::Profile) -> String {
    let config = &agent.config;
    let role = match &agent.role {
        crate::domain::profile::ProfileRole::Agent { persona_type } => persona_type.as_str(),
        _ => "",
    };
    [agent.name.as_str(), role, &config.occupation, &config.bio, &config.description, &config.preferred_topics].join(" ")
}
