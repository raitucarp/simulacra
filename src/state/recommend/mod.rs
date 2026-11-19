//! The store side of recommendations (posts are the source of truth): a profile's engagement
//! signals and candidate posts ([`signals`]), and the four uses of post embeddings built on
//! them — the For You ranking, a persona's recommendations, the post an agent answers next,
//! and the posts related to a post.

pub mod signals;
#[cfg(test)]
mod tests;

pub use signals::{candidates_for, conversation_root, network_posts, signals_of};

use crate::domain::embedding::{similar_to, EmbeddingOwner, OwnerKind, ScoredOwner, VectorIndex};
use crate::domain::recommend::{freshness, interest_vector, rank_all, recommend, Blend, Candidate, Recommendation};
use crate::domain::{NetworkId, Post, PostId, Profile, ProfileId};
use crate::state::AppState;
use jiff::Timestamp;
use std::collections::HashMap;

/// Recent posts an agent picks its next answer from.
pub const AGENT_WINDOW: usize = 50;

/// Recommendations over one store, vector index and embedding model, as of `now`.
pub struct Recommender<'a> {
    pub state: &'a AppState,
    pub index: &'a VectorIndex,
    pub model: &'a str,
    pub now: Timestamp,
}

impl Recommender<'_> {
    /// `profile`'s interest vector in `network` (`None` until something it engaged with is
    /// embedded).
    pub fn interest(&self, profile: &ProfileId, network: &NetworkId) -> Option<Vec<f32>> {
        interest_vector(self.index, self.model, &signals_of(self.state, profile, network), self.now)
    }

    /// For You scores of every post of the active network for the profile the user is acting
    /// as: other people's posts by meaning and freshness, the user's own by freshness alone.
    /// Empty while the user has no embedded engagement.
    pub fn for_you(&self) -> HashMap<PostId, f32> {
        let (me, network) = (&self.state.active_profile_id, &self.state.active_network_id);
        let Some(interest) = self.interest(me, network) else { return HashMap::new() };
        let others = network_posts(self.state, network).filter(|post| &post.author_id != me && !post.is_repost());
        let candidates: Vec<Candidate> = others.map(|post| Candidate { post: post.id.clone(), created_at: post.created_at }).collect();
        let mut scores: HashMap<PostId, f32> = rank_all(self.index, self.model, &interest, &candidates, self.now, Blend::FEED).into_iter().map(|r| (r.post, r.score)).collect();
        let unscored: Vec<&Post> = network_posts(self.state, network).filter(|post| !scores.contains_key(&post.id)).collect();
        for post in unscored {
            scores.insert(post.id.clone(), Blend::FEED.freshness * freshness(self.now, post.created_at, Blend::FEED.half_life_hours));
        }
        scores
    }

    /// The `k` posts `profile` would find most relevant in its network (the active one for a
    /// profile that belongs to every network).
    pub fn for_profile(&self, profile: &Profile, k: usize) -> Vec<Recommendation> {
        let network = profile.network_id.clone().unwrap_or_else(|| self.state.active_network_id.clone());
        let Some(interest) = self.interest(&profile.id, &network) else { return Vec::new() };
        recommend(self.index, self.model, &interest, &candidates_for(self.state, &profile.id, &network), self.now, Blend::PERSONA, k)
    }

    /// The recent post of `network` that `agent` cares most about and has not answered yet
    /// (`None` while it has no embedded interest or nothing close to answer).
    pub fn agent_answer(&self, agent: &Profile, network: &NetworkId) -> Option<PostId> {
        let interest = self.interest(&agent.id, network)?;
        let mut recent = candidates_for(self.state, &agent.id, network);
        recent.truncate(AGENT_WINDOW);
        recommend(self.index, self.model, &interest, &recent, self.now, Blend::AGENT, 1).pop().map(|r| r.post)
    }

    /// The `k` posts nearest `post` in meaning, in its network and outside its conversation.
    pub fn related(&self, post: &Post, k: usize) -> Vec<ScoredOwner> {
        let root = conversation_root(self.state, post);
        let keep = |owner: &EmbeddingOwner| {
            owner.kind == OwnerKind::Post
                && self.state.get_post_by_id(&owner.id).is_some_and(|other| other.network_id == post.network_id && !other.is_repost() && conversation_root(self.state, other) != root)
        };
        similar_to(self.index, &EmbeddingOwner::post(&post.id), self.model, k, keep)
    }
}
