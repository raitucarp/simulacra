//! N3, N6–N10: what a post's "…" menu offers beyond its data, read from the post (and the
//! store) by the feed for every card.

use crate::domain::post::garden::Maturity;
use crate::domain::post::pinned::QuoteDrift;
use crate::domain::Post;
use crate::state::AppState;

/// The facts of one post its menu needs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PostFacts {
    /// Its garden stage, once planted (N3).
    pub garden: Option<Maturity>,
    /// Its pinned quote's source changed but still holds the words: it can be re-pinned (N7).
    pub repinnable: bool,
}

impl PostFacts {
    /// The facts of `post` in `state`.
    pub fn of(post: &Post, state: &AppState) -> Self {
        Self { garden: post.extras.garden.map(|note| note.maturity), repinnable: state.quote_drift(post) == Some(QuoteDrift::SourceChanged) }
    }
}
