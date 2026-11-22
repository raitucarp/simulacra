//! The word-match fallback: BM25 over the active network's posts and documents, scored
//! relative to the best hit so the similarity bar still reads sensibly.

use super::hits::{hits_for, SemanticHit};
use crate::agent::tools::vault_search::{Bm25Index, Chunk};
use crate::domain::embedding::{chunk_text, ChunkSpec, EmbeddingOwner, OwnerKind, ScoredOwner};
use crate::state::AppState;

/// Separates the owner kind from its id inside a BM25 chunk id.
const OWNER_SEPARATOR: char = ':';

/// The `k` best word matches for `query` in the active network.
pub fn lexical_hits(state: &AppState, query: &str, k: usize, spec: ChunkSpec) -> Vec<SemanticHit> {
    let index = Bm25Index::new(network_chunks(state, spec));
    let found = index.search(query, k * 4);
    let best = found.first().map_or(1.0, |hit| hit.score.max(f64::EPSILON));
    let mut seen: Vec<EmbeddingOwner> = Vec::new();
    let scored: Vec<ScoredOwner> = found
        .into_iter()
        .filter_map(|hit| {
            let owner = owner_of(&hit.chunk.doc_id)?;
            if seen.contains(&owner) {
                return None;
            }
            seen.push(owner.clone());
            Some(ScoredOwner { owner, score: (hit.score / best) as f32, best_chunk: hit.chunk.ordinal })
        })
        .take(k)
        .collect();
    hits_for(state, &scored, spec)
}

/// The chunks of every post and document of the active network.
fn network_chunks(state: &AppState, spec: ChunkSpec) -> Vec<Chunk> {
    let posts = state
        .current_network_posts()
        .filter(|post| !post.is_repost() && !post.is_locked()) // X2: only a placeholder to match
        .map(|post| (EmbeddingOwner::post(&post.id), String::new(), post.content.as_str()));
    let documents = state.current_documents().map(|doc| (EmbeddingOwner::document(doc.id.clone()), doc.title.clone(), doc.content.as_str()));
    posts
        .chain(documents)
        .flat_map(|(owner, title, text)| {
            let doc_id = format!("{}{OWNER_SEPARATOR}{}", owner.kind.key(), owner.id);
            chunk_text(text, spec).into_iter().map(move |chunk| Chunk { doc_id: doc_id.clone(), title: title.clone(), ordinal: chunk.index, text: chunk.text })
        })
        .collect()
}

fn owner_of(chunk_id: &str) -> Option<EmbeddingOwner> {
    let (kind, id) = chunk_id.split_once(OWNER_SEPARATOR)?;
    Some(EmbeddingOwner { kind: OwnerKind::from_key(kind)?, id: id.to_string() })
}
