//! What a post action reads from the store (sb C2): the thought with its author and thread
//! as an [`ActionContext`], and the "search concept" extras (vault full-text hits from
//! `documents_fts`, and the items nearest the thought in meaning).

use super::super::view::AppView;
use crate::domain::document::query::CatalogueQuery;
use crate::domain::embedding::{similar_to, EmbeddingOwner, ScoredOwner};
use crate::domain::post_actions::{ActionContext, AuthorContext, ThreadLine};
use crate::domain::{Post, PostId};
use crate::state::semantic::{hits_for, lexical_hits, SemanticHit};
use crate::state::{build_conversation, ReplyLimits};
use crate::ui::features::post_actions::ConceptHits;

/// Hits listed per section.
const HIT_LIMIT: usize = 5;
/// Reply depth of a direct reply and of a reply to it.
const DIRECT_DEPTH: usize = 1;
const NESTED_DEPTH: usize = 2;
/// Score of the best full-text match (later ranks fall off linearly).
const TOP_SCORE: f32 = 1.0;

impl AppView {
    /// The thought `post_id` with its author, the thread above it and every reply below it.
    pub(super) fn action_context(&self, post_id: &PostId) -> Option<ActionContext> {
        let post = self.state.get_post(post_id)?;
        let conversation = build_conversation(&self.state, post, ReplyLimits::UNLIMITED);
        let line = |post: &Post, depth: usize| {
            let author = self.state.author_of(post);
            ThreadLine::new(&author.name, &author.handle, &post.content, depth)
        };
        let replies = conversation.branches.iter().flat_map(|branch| {
            std::iter::once(line(branch.reply, DIRECT_DEPTH)).chain(branch.nested.iter().map(|nested| line(nested, NESTED_DEPTH)))
        });
        let author = self.state.author_of(post);
        Some(ActionContext {
            thought: line(post, 0),
            author: AuthorContext { bio: author.config.bio.clone(), is_agent: !author.is_human_persona() },
            ancestors: conversation.ancestors.iter().map(|ancestor| line(ancestor, 0)).collect(),
            replies: replies.collect(),
            reader_language: self.state.preferences().general.language,
        })
    }

    /// Vault entries mentioning `concept`, and the items nearest the thought.
    pub(super) fn concept_hits(&self, post_id: &PostId, concept: &str) -> ConceptHits {
        ConceptHits { vault: self.vault_concept_hits(concept), nearby: self.nearby_hits(post_id, concept) }
    }

    /// Full-text matches in the active network's vault, best first.
    fn vault_concept_hits(&self, concept: &str) -> Vec<SemanticHit> {
        let Some(vault) = self.state.db.as_ref() else { return Vec::new() };
        let query = CatalogueQuery { text: concept.to_string(), ..CatalogueQuery::new(self.state.active_network_id.clone()) };
        let ids = vault.search_catalogue(&query, &self.state.library.tree()).unwrap_or_default();
        let count = ids.len().min(HIT_LIMIT).max(1) as f32; // at least one, so the division is safe
        let scored: Vec<ScoredOwner> = ids
            .into_iter()
            .take(HIT_LIMIT)
            .enumerate()
            .map(|(rank, id)| ScoredOwner { owner: EmbeddingOwner::document(id), score: TOP_SCORE - rank as f32 / count, best_chunk: 0 })
            .collect();
        hits_for(&self.state, &scored, self.chunk_spec())
    }

    /// The thought's nearest neighbours by meaning; word matches on the concept when meaning
    /// is unavailable (search off, no key, or nothing indexed).
    fn nearby_hits(&self, post_id: &PostId, concept: &str) -> Vec<SemanticHit> {
        let owner = EmbeddingOwner::post(post_id);
        let keep = |candidate: &EmbeddingOwner| candidate != &owner && self.state.is_in_active_network(candidate);
        let model = self.embedding_config().model;
        let ranked = match self.semantic_fallback() {
            None => similar_to(&self.semantic.index, &owner, &model, HIT_LIMIT, keep),
            Some(_) => Vec::new(),
        };
        if !ranked.is_empty() {
            return hits_for(&self.state, &ranked, self.chunk_spec());
        }
        let words = lexical_hits(&self.state, concept, HIT_LIMIT + 1, self.chunk_spec());
        words.into_iter().filter(|hit| hit.target.owner() != owner).take(HIT_LIMIT).collect()
    }
}
