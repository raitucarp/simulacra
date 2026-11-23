//! N3, N6–N10: what a post's "…" menu offers beyond its data, read from the post (and the
//! store) by the feed for every card, and the lists every menu shares.

use crate::domain::post::audience::Audience;
use crate::domain::post::garden::Maturity;
use crate::domain::post::pinned::QuoteDrift;
use crate::domain::{Post, PostId, ProfileId};
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
    /// The title of the series it is part of (N10).
    pub series: Option<String>,
    /// Its voice note can be transcribed: it has none yet, and the post is not locked (N11).
    pub transcribable: bool,
    /// It was retracted or corrected (N14).
    pub correction: Option<crate::domain::post::correction::CorrectionKind>,
}

impl PostFacts {
    /// The facts of `post` in `state`.
    pub fn of(post: &Post, state: &AppState) -> Self {
        Self {
            garden: post.extras.garden.map(|note| note.maturity),
            repinnable: state.quote_drift(post) == Some(QuoteDrift::SourceChanged),
            audience: post.extras.audience.as_deref().cloned(),
            asked: post.extras.question.is_some(),
            series: post.extras.series.as_ref().map(|entry| entry.title.clone()),
            transcribable: !post.is_locked() && post.extras.voice.as_ref().is_some_and(|voice| voice.transcript.is_none()),
            correction: post.extras.correction.as_ref().map(|c| c.kind),
        }
    }
}

/// What every card's menu shares, gathered once per render.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IdeaSource {
    /// The active network's agents, by name: who a post can be shared with (N8).
    pub agents: Vec<(ProfileId, String)>,
    /// The titles of the active network's series, most recently added to first (N10).
    pub series: Vec<String>,
    /// N11: the post whose voice note is playing in the app.
    pub playing: Option<PostId>,
    /// N11: the post whose voice note is being transcribed.
    pub transcribing: Option<PostId>,
    /// N13: the claim the AI is asked about, and its answers by post.
    pub rechecking: Option<PostId>,
    pub rechecks: Vec<(PostId, String)>,
}
