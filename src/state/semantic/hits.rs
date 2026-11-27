//! Ranked owners as displayable hits: a title, the best matching passage and the score.

use crate::domain::embedding::{chunk_text, ChunkSpec, EmbeddingOwner, OwnerKind, ScoredOwner};
use crate::domain::post::readable_line;
use crate::domain::PostId;
use crate::state::AppState;

/// Characters of the excerpt under the title.
const EXCERPT_CHARS: usize = 180;
const ELLIPSIS: char = '…';
/// What a heading ends with in a one-line passage.
const HEADING_END: &str = " ·";
const MAX_HEADING_LEVEL: usize = 6;
const QUOTE_MARK: &str = "> ";

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
            let excerpt = shortened(&passage(&readable_line(&post.content), ranked.best_chunk, spec), EXCERPT_CHARS);
            let author = state.author_of(post);
            let (title, detail) = (author.name.clone(), author.display_handle());
            Some(SemanticHit { target: HitTarget::Post(post.id.clone()), title, detail, excerpt, score })
        }
        OwnerKind::Document => {
            let doc = state.documents.iter().find(|doc| doc.id == ranked.owner.id)?;
            let excerpt = shortened(&plain_markdown(&passage(&doc.content, ranked.best_chunk, spec)), EXCERPT_CHARS);
            Some(SemanticHit { target: HitTarget::Document(doc.id.clone()), title: doc.title.clone(), detail: String::new(), excerpt, score })
        }
    }
}

/// Chunk `index` of `text` (the first chunk when it no longer exists). A post is first read
/// as plain text (no `[[post_…]]` ids or to-do markup), then chunked the same way.
fn passage(text: &str, index: usize, spec: ChunkSpec) -> String {
    let chunks = chunk_text(text, spec);
    let chunk = chunks.iter().find(|chunk| chunk.index == index).or(chunks.first());
    chunk.map_or(text, |chunk| chunk.text.as_str()).to_string()
}

/// A document passage without Markdown's line marks: a heading reads as its words and a
/// middle dot before what follows, a quote as its words (pure).
pub fn plain_markdown(text: &str) -> String {
    let lines: Vec<String> = text.lines().map(str::trim).filter(|line| !line.is_empty()).map(plain_line).collect();
    lines.join(" ").trim_end_matches(HEADING_END).to_string()
}

fn plain_line(line: &str) -> String {
    let words = line.trim_start_matches('#');
    let marks = line.len() - words.len();
    if (1..=MAX_HEADING_LEVEL).contains(&marks) && words.starts_with(' ') {
        return format!("{}{HEADING_END}", words.trim());
    }
    line.strip_prefix(QUOTE_MARK).unwrap_or(line).to_string()
}

/// The first `max_chars` characters of `text` on one line, marked when shortened.
pub fn shortened(text: &str, max_chars: usize) -> String {
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    match line.char_indices().nth(max_chars) {
        Some((cut, _)) => format!("{}{ELLIPSIS}", line[..cut].trim_end()),
        None => line,
    }
}
