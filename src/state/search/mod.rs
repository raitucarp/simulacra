//! The search executor (Q3): one ranked list of posts for a parsed [`PostQuery`].
//!
//! - **Text** — words (as prefixes) and phrases matched by the full-text index (bm25), or in
//!   memory when there is no vault.
//! - **Meaning** — posts ranked by the embedded query, when the caller has them.
//! - **Fusion** — both rankings merge by reciprocal rank, so a post found both ways rises.
//! - **Filters** — `from:` `kind:` `symbol:` `tint:` `has:` `is:` `before:` `after:` and every
//!   negated part narrow the list; a filter-only query lists the matching posts newest first.
//!
//! Only the active network's published and archived posts are searched; scheduled posts stay
//! out until due. A locked post (X2) is never found by its text or meaning (its text is sealed,
//! only the placeholder is stored); it is found only by filters such as `is:locked`.

mod dates;
mod filters;
mod fuse;
mod text;
#[cfg(test)]
mod tests;

pub use dates::resolve_date;
pub use fuse::RRF_K;

use crate::domain::embedding::{OwnerKind, ScoredOwner};
use crate::domain::search_query::PostQuery;
use crate::domain::PostId;
use crate::state::AppState;
use jiff::civil::Date;
use jiff::tz::TimeZone;

/// Text matches read before filters narrow them.
pub const TEXT_CANDIDATES: usize = 5_000;

/// What to search for.
#[derive(Debug, Clone)]
pub struct SearchRequest<'a> {
    pub query: &'a PostQuery,
    /// Owners ranked by meaning, best first (empty when the query was not embedded).
    pub meaning: &'a [ScoredOwner],
    /// Most hits returned.
    pub limit: usize,
    /// The local day `before:` / `after:` words such as `yesterday` are read from.
    pub today: Date,
    /// The zone a post's day is read in.
    pub tz: TimeZone,
}

/// One result.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchHit {
    pub post_id: PostId,
    /// Fused score: higher is better.
    pub score: f64,
    /// Found by its words.
    pub by_text: bool,
    /// Found by its meaning.
    pub by_meaning: bool,
}

impl AppState {
    /// The posts answering `request`, best first (none for a blank query).
    pub fn search(&self, request: &SearchRequest<'_>) -> Vec<SearchHit> {
        let query = request.query;
        if query.is_empty() {
            return Vec::new();
        }
        let text = text::ranked(self, query);
        let meaning = meaning_ranked(request.meaning);
        let fused = if query.words.is_empty() && query.phrases.is_empty() && meaning.is_empty() {
            fuse::listed(self.current_network_posts().map(|post| post.id.clone()))
        } else {
            fuse::fuse(&text, &meaning)
        };
        let when = filters::When { today: request.today, tz: request.tz.clone() };
        fused
            .into_iter()
            .filter(|hit| self.get_post(&hit.post_id).is_some_and(|post| filters::admits(self, post, hit, query, &when)))
            .take(request.limit)
            .collect()
    }
}

/// The posts of a meaning ranking that point the same way, best first.
fn meaning_ranked(scored: &[ScoredOwner]) -> Vec<PostId> {
    scored.iter().filter(|ranked| ranked.owner.kind == OwnerKind::Post && ranked.score > 0.0).map(|ranked| PostId::new(ranked.owner.id.clone())).collect()
}
