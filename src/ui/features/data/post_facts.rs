//! N3, N6–N10: what a post's "…" menu offers beyond its data, read from the post (and the
//! store) by the feed for every card, and the lists every menu shares.

use crate::domain::post::audience::Audience;
use crate::domain::post::garden::Maturity;
use crate::domain::post::pinned::QuoteDrift;
use crate::domain::{Post, ProfileId};
use crate::state::AppState;

/// The facts of one post its menu needs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PostFacts {
    /// Its garden stage, once planted (N3).
    pub garden: Option<Maturity>,
    /// Its pinned quote's source changed but still holds the words: it can be re-pinned (N7).
    pub repinnable: bool,
    /// Who may read it (`None`: everyone, N8).
    pub audience: Option<Audience>,
    /// It was asked of the room already (N9).
    pub asked: bool,
}

impl PostFacts {
    /// The facts of `post` in `state`.
    pub fn of(post: &Post, state: &AppState) -> Self {
        Self {
            garden: post.extras.garden.map(|note| note.maturity),
            repinnable: state.quote_drift(post) == Some(QuoteDrift::SourceChanged),
            audience: post.extras.audience.as_deref().cloned(),
            asked: post.extras.question.is_some(),
        }
    }
}

/// What every card's menu shares, gathered once per render.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IdeaSource {
    /// The active network's agents, by name: who a post can be shared with (N8).
    pub agents: Vec<(ProfileId, String)>,
}
