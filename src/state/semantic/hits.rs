//! Ranked owners as displayable hits: a title, the best matching passage and the score.

use crate::domain::embedding::{chunk_text, ChunkSpec, EmbeddingOwner, OwnerKind, ScoredOwner};
use crate::domain::PostId;
use crate::state::AppState;

/// Characters of the excerpt under the title.
const EXCERPT_CHARS: usize = 180;
const ELLIPSIS: char = '…';

/// What a hit opens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HitTarget {
    Post(PostId),
    Document(String),
}

impl HitTarget {
    /// The embedding owner behind the target.
    pub fn owner(&self) -> EmbeddingOwner {
        match self {
            Self::Post(id) => EmbeddingOwner::post(id),
            Self::Document(id) => EmbeddingOwner::document(id.clone()),
        }
    }
}

/// One search or "find similar" result.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticHit {
    pub target: HitTarget,
    /// The author's name for posts, the title for documents.
    pub title: String,
    /// The author's handle for posts, blank for documents.
    pub detail: String,
    /// The best matching passage.
    pub excerpt: String,
    /// Similarity in `[0, 1]` (cosine, or a word-match score relative to the best).
    pub score: f32,
}

impl AppState {
    /// Whether `owner` still exists in the active network.
    pub fn is_in_active_network(&self, owner: &EmbeddingOwner) -> bool {
        match owner.kind {
            OwnerKind::Post => self.get_post_by_id(&owner.id).is_some_and(|post| post.network_id == self.active_network_id),
            OwnerKind::Document => self.current_documents().any(|doc| doc.id == owner.id),
        }
    }
}

/// Hits for ranked owners that still exist and point the same way (positive score), in
/// rank order.
pub fn hits_for(state: &AppState, scored: &[ScoredOwner], spec: ChunkSpec) -> Vec<SemanticHit> {
    scored.iter().filter(|ranked| ranked.score > 0.0).filter_map(|ranked| hit(state, ranked, spec)).collect()
}

fn hit(state: &AppState, ranked: &ScoredOwner, spec: ChunkSpec) -> Option<SemanticHit> {
    let score = ranked.score.clamp(0.0, 1.0);
    match ranked.owner.kind {
        OwnerKind::Post => {
            let post = state.get_post_by_id(&ranked.owner.id)?;
            let excerpt = passage(&post.content, ranked.best_chunk, spec);
            let author = state.author_of(post);
            let (title, detail) = (author.name.clone(), author.display_handle());
            Some(SemanticHit { target: HitTarget::Post(post.id.clone()), title, detail, excerpt, score })
        }
        OwnerKind::Document => {
            let doc = state.documents.iter().find(|doc| doc.id == ranked.owner.id)?;
            let excerpt = passage(&doc.content, ranked.best_chunk, spec);
            Some(SemanticHit { target: HitTarget::Document(doc.id.clone()), title: doc.title.clone(), detail: String::new(), excerpt, score })
        }
    }
}

/// Chunk `index` of `text` (the first chunk when it no longer exists), shortened.
fn passage(text: &str, index: usize, spec: ChunkSpec) -> String {
    let chunks = chunk_text(text, spec);
    let chunk = chunks.iter().find(|chunk| chunk.index == index).or(chunks.first());
    shortened(chunk.map_or(text, |chunk| chunk.text.as_str()), EXCERPT_CHARS)
}

/// The first `max_chars` characters of `text` on one line, marked when shortened.
pub fn shortened(text: &str, max_chars: usize) -> String {
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    match line.char_indices().nth(max_chars) {
        Some((cut, _)) => format!("{}{ELLIPSIS}", line[..cut].trim_end()),
        None => line,
    }
}
