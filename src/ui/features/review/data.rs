//! What the shell hands the Review page: the active network's items and log, today's review
//! day, the limits and retention of Settings › Learning, and the posts and document titles
//! the items and maps refer to (a post face is drawn from its current text and author).

use crate::domain::post::readable_line;
use crate::domain::review::queue::{plan_today, DailyLimits, TodayPlan};
use crate::domain::review::schedule::Policy;
use crate::domain::review::{ItemId, ReviewEntry, ReviewItem};
use crate::domain::{NetworkId, Post, Profile};
use crate::ui::features::review::learning::LearningData;
use jiff::civil::Date;
use std::collections::HashMap;
use std::rc::Rc;

/// A post with its author, as the stream shows it.
#[derive(Debug, Clone, PartialEq)]
pub struct PostView {
    pub post: Post,
    pub author: Profile,
    /// The posts its `[[post_…]]` links point at, so a link reads as their opening words.
    pub linked: Vec<Post>,
}

impl PostView {
    /// `post` as `state` holds it: its author and the posts it links to.
    pub fn of(state: &crate::state::AppState, post: &Post) -> Self {
        let targets = crate::domain::post::wikilink_targets(&crate::domain::post::tokenize_post_body(&post.content));
        let linked = targets.iter().filter_map(|target| state.get_post_by_id(target.trim())).cloned().collect();
        Self { post: post.clone(), author: state.author_of(post).clone(), linked }
    }
}

/// Posts by id.
pub type PostViews = Rc<HashMap<String, PostView>>;

/// The Review page's data.
#[derive(Debug, Clone)]
pub struct ReviewData {
    pub network_id: NetworkId,
    /// The network's items, retired ones included (the Items list shows them).
    pub items: Vec<ReviewItem>,
    pub log: Vec<ReviewEntry>,
    pub today: Date,
    pub limits: DailyLimits,
    pub policy: Policy,
    /// Posts the items and maps refer to.
    pub posts: PostViews,
    /// Vault document titles by id.
    pub documents: Rc<HashMap<String, String>>,
    /// The Learn tab's goals, sessions, palaces, radar and weekly digest (sb E2).
    pub learning: Rc<LearningData>,
}

impl ReviewData {
    /// An empty page for `network_id`.
    pub fn empty(network_id: NetworkId, today: Date, limits: DailyLimits, policy: Policy) -> Self {
        Self { network_id, items: Vec::new(), log: Vec::new(), today, limits, policy, posts: Rc::default(), documents: Rc::default(), learning: Rc::default() }
    }

    pub fn item(&self, id: &ItemId) -> Option<&ReviewItem> {
        self.items.iter().find(|item| &item.id == id)
    }

    /// Items still in reviews or suspended (not retired).
    pub fn live_items(&self) -> Vec<ReviewItem> {
        self.items.iter().filter(|item| !item.retired).cloned().collect()
    }

    /// Today's counts within the limits.
    pub fn plan(&self) -> TodayPlan {
        plan_today(&self.items, &self.log, self.today, self.limits)
    }

    pub fn post(&self, id: &str) -> Option<&PostView> {
        self.posts.get(id)
    }

    /// A post's current text as a reader says it, without markup or `[[post_…]]` links (none
    /// once deleted).
    pub fn post_text(&self, id: &str) -> Option<String> {
        self.posts.get(id).map(|view| readable_line(&view.post.content))
    }

    pub fn document_title(&self, id: &str) -> Option<&String> {
        self.documents.get(id)
    }

    /// The answer of `item` as text, posts resolved to their current text.
    pub fn answer_text(&self, item: &ReviewItem) -> String {
        item.answer_text(&|id| self.post_text(id))
    }

    /// The prompt of `item` as text.
    pub fn prompt_text(&self, item: &ReviewItem) -> String {
        item.prompt_text(&|id| self.post_text(id))
    }
}
