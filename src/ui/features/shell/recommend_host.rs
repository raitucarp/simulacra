//! Recommendations in the window, all from post embeddings: the "For you" order and its
//! ranking, the focused post's related posts, and the selected profile's recommended posts.
//! Each result is cached until the posts or the user's likes, the vector index, the model or
//! the hour changes.

use super::binder::Binder;
use super::view::AppView;
use crate::domain::embedding::{EmbeddingOwner, ScoredOwner};
use crate::domain::{PostId, ProfileId};
use crate::state::recommend::Recommender;
use crate::state::semantic::{hits_for, SemanticHit};
use crate::state::{FeedRanking, FeedTab};
use crate::ui::features::feed::{FeedDiscovery, FeedOrder};
use gpui_kit::*;
use jiff::Timestamp;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Related posts under the inspector's conversation, and recommendations on a profile.
const RELATED_LIMIT: usize = 5;
const PROFILE_LIMIT: usize = 10;
const SECONDS_PER_HOUR: i64 = 3_600;

/// What a cached result was computed from.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Stamp {
    /// Post index revision, post count and like changes.
    posts: (u64, usize, u64),
    vectors: usize,
    model: String,
    hour: i64,
    /// The post or profile it is about (the network and acting profile for the ranking).
    subject: String,
}

type Cached<T> = RefCell<Option<(Stamp, T)>>;

/// The "For you" order and the cached recommendations.
#[derive(Default)]
pub struct RecommendHost {
    pub order: FeedOrder,
    ranking: Cached<FeedRanking>,
    related: Cached<Rc<[SemanticHit]>>,
    profile: Cached<Rc<[SemanticHit]>>,
    generation: Cell<u64>,
}

/// `cache`'s value for `stamp`, computing it with `make` when the stamp changed.
fn cached<T: Clone>(cache: &Cached<T>, stamp: Stamp, make: impl FnOnce() -> T) -> T {
    if let Some((_, value)) = cache.borrow().as_ref().filter(|(at, _)| at == &stamp) {
        return value.clone();
    }
    let value = make();
    *cache.borrow_mut() = Some((stamp, value.clone()));
    value
}

impl AppView {
    fn recommend_stamp(&self, model: &str, subject: String) -> Stamp {
        let posts = (self.state.index.revision(), self.state.posts.len(), self.state.engagement);
        let hour = Timestamp::now().as_second() / SECONDS_PER_HOUR;
        Stamp { posts, vectors: self.semantic.index.len(), model: model.to_owned(), hour, subject }
    }

    fn recommender<'a>(&'a self, model: &'a str) -> Recommender<'a> {
        Recommender { state: &self.state, index: &self.semantic.index, model, now: Timestamp::now() }
    }

    /// The "For you" ranking while the recommended order is on and "For you" is shown.
    pub(crate) fn feed_ranking(&self) -> Option<FeedRanking> {
        if self.recommend.order != FeedOrder::Recommended || self.feed.feed_tab != FeedTab::ForYou {
            return None;
        }
        let model = self.embedding_config().model;
        let subject = format!("{}|{}", self.state.active_network_id.as_str(), self.state.active_profile_id.as_str());
        let host = &self.recommend;
        Some(cached(&host.ranking, self.recommend_stamp(&model, subject), || {
            host.generation.set(host.generation.get() + 1);
            FeedRanking { generation: host.generation.get(), scores: Rc::new(self.recommender(&model).for_you()) }
        }))
    }

    /// The posts related to `post` (from other conversations), best first.
    pub(crate) fn related_hits(&self, post: Option<&PostId>) -> Rc<[SemanticHit]> {
        let Some(post) = post.and_then(|id| self.state.get_post(id)) else { return Rc::from([]) };
        let model = self.embedding_config().model;
        cached(&self.recommend.related, self.recommend_stamp(&model, post.id.as_str().to_owned()), || {
            Rc::from(hits_for(&self.state, &self.recommender(&model).related(post, RELATED_LIMIT), self.chunk_spec()))
        })
    }

    /// The posts recommended to profile `id`, best first. The bars show the blended score they
    /// are ranked by (meaning, then freshness), so they never read out of order.
    pub(crate) fn profile_hits(&self, id: Option<&ProfileId>) -> Rc<[SemanticHit]> {
        let Some(profile) = id.and_then(|id| self.state.user_profiles.iter().find(|p| &p.id == id)) else { return Rc::from([]) };
        let model = self.embedding_config().model;
        cached(&self.recommend.profile, self.recommend_stamp(&model, profile.id.as_str().to_owned()), || {
            let ranked: Vec<ScoredOwner> = self.recommender(&model).for_profile(profile, PROFILE_LIMIT).into_iter().map(|r| ScoredOwner { owner: EmbeddingOwner::post(&r.post), score: r.score, best_chunk: 0 }).collect();
            Rc::from(hits_for(&self.state, &ranked, self.chunk_spec()))
        })
    }

    /// Stream header › Latest / Recommended.
    pub(crate) fn select_feed_order(&mut self, order: FeedOrder, _: &mut Window, cx: &mut Context<Self>) {
        self.recommend.order = order;
        self.sync_stream(cx);
        cx.notify();
    }

    /// The timeline's recommendations: the order switch and the related posts of the post the
    /// inspector shows (the selected one, else the top of the stream).
    pub(super) fn feed_discovery(&self, bind: &Binder) -> FeedDiscovery {
        let top = || self.feed.stream.rows().post(&self.state, 0).or_else(|| self.state.current_network_posts().next()).map(|post| post.id.clone());
        let focused = self.feed.selected_thread_post_id.clone().or_else(top).map(|id| self.state.action_target(&id));
        FeedDiscovery { order: self.recommend.order, on_select_order: Rc::new(bind.with(Self::select_feed_order)), related: self.related_hits(focused.as_ref()) }
    }
}
