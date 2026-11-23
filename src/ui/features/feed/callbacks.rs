//! Shared post interaction callbacks and the post card renderer wired to them.

use super::params::{
    DeletePostCallback, FindSimilarCallback, LearnPostCallback, LearningPosts, LikePostCallback, PostActionCallback, QuotePostCallback,
    ReplyPostCallback, RepostPostCallback, SelectPostCallback,
};
use crate::domain::post::PostLifecycle;
use crate::domain::preferences::ArchiveAfter;
use crate::domain::{Post, PostId};
use crate::ui::features::data::DataControls;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::post::{render_post_item, BodyLinks, LearnControls, PostAction, PostCardHandlers, ThreadlineStyle};
use gpui_kit::{AnyElement, App, IntoElement, Window};
use std::rc::Rc;

/// Reference-counted post interaction callbacks shared by every rendered card.
#[derive(Clone)]
pub(crate) struct PostCallbacks {
    pub on_like: Rc<LikePostCallback>,
    pub on_reply: Rc<ReplyPostCallback>,
    pub on_select: Rc<SelectPostCallback>,
    pub on_repost: Rc<RepostPostCallback>,
    pub on_quote: Rc<QuotePostCallback>,
    pub on_delete: Rc<DeletePostCallback>,
    pub on_find_similar: Rc<FindSimilarCallback>,
    pub on_learn: Rc<LearnPostCallback>,
    /// Posts in review (sb C1b).
    pub learning: LearningPosts,
    /// sb C2: "…" menu › Actions.
    pub on_post_action: Rc<PostActionCallback>,
    /// Hashtag and wikilink clicks inside post bodies.
    pub links: BodyLinks,
    /// sb E1: credence chips of the user's own posts.
    pub credence: Option<crate::ui::features::thinking::credence::CredenceSource>,
    /// sb F2: edit, history, bookmarks and collections.
    pub data: Option<crate::ui::features::data::DataSource>,
    /// Epic R: reactions.
    pub reactions: Option<crate::ui::features::reactions::ReactionSource>,
    /// X4: unlocking locked posts.
    pub locks: Option<crate::ui::features::post::LockSource>,
}

/// Visual and interaction mode of a rendered card.
#[derive(Clone, Copy)]
pub(crate) struct CardMode {
    pub is_focused: bool,
    pub style: ThreadlineStyle,
    /// Whether clicking the card selects it (the inspected root card is not selectable).
    pub selectable: bool,
}

/// Binds a `(PostId, Window, App)` callback to one post.
fn bind<F: Fn(PostId, &mut Window, &mut App) + ?Sized + 'static>(cb: &Rc<Box<F>>, id: &PostId) -> PostAction {
    let (cb, id) = (cb.clone(), id.clone());
    Rc::new(move |window, cx| cb(id.clone(), window, cx))
}

/// Binds the actions callback to one post (sb C2).
fn bind_action(cb: &Rc<PostActionCallback>, id: &PostId) -> crate::ui::features::post::PickPostAction {
    let (cb, id) = (cb.clone(), id.clone());
    Rc::new(move |kind, window, cx| cb(id.clone(), kind, window, cx))
}

impl PostCallbacks {
    /// Card handlers for `post`. Like, reply, repost, quote and open act on `target` (the
    /// original of an instant repost); delete stays on the card's own thought and is offered
    /// only on thoughts written by a user persona.
    fn handlers_for(&self, post: &Post, state: &AppState, target: CardTarget, selectable: bool) -> PostCardHandlers {
        let author = state.author_of(post);
        let CardTarget { id: target_id, handle, .. } = target.clone();
        let (reply, reply_id) = (self.on_reply.clone(), target_id.clone());
        let select = self.on_select.clone();
        PostCardHandlers {
            on_select: if selectable { bind(&self.on_select, &target_id) } else { Rc::new(|_, _| {}) },
            on_like: bind(&self.on_like, &target_id),
            on_reply: Rc::new(move |window, cx| reply(reply_id.clone(), &handle, window, cx)),
            on_repost: bind(&self.on_repost, &target_id),
            on_quote: bind(&self.on_quote, &target_id),
            on_find_similar: bind(&self.on_find_similar, &target_id),
            learn: self.learn_controls(&target_id),
            on_open_post: Rc::new(move |id, window, cx| select(id, window, cx)),
            on_delete: author.is_human_persona().then(|| bind(&self.on_delete, &post.id)),
            links: self.links.clone(),
            on_post_action: (!target.locked).then(|| bind_action(&self.on_post_action, &target_id)), // X4: a locked post is never sent to a model
            lock: self.locks.as_ref().filter(|_| target.locked).map(|source| source.controls(&target_id)),
            credence: self.credence.as_ref().filter(|_| target.human).map(|source| source.controls(&target_id)),
            data: self.data.as_ref().map(|source| DataControls { lifecycle: target.lifecycle, auto_archive: target.auto_archive, symbols: target.symbols.clone(), version: target.version, replied_to: target.replied_to, locked: target.locked, facts: target.facts.clone(), ..source.controls(&target_id, target.human) }),
            reactions: self.reactions.as_ref().map(|source| source.controls(&target_id, state)),
        }
    }

    /// The learn submenu and indicator of the post `id` (sb C1b).
    fn learn_controls(&self, id: &PostId) -> LearnControls {
        let (on_learn, post) = (self.on_learn.clone(), id.clone());
        LearnControls { status: self.learning.get(id.as_str()).copied(), on_learn: Rc::new(move |kind, window, cx| on_learn(post.clone(), kind, window, cx)) }
    }

    /// Renders `post` as a card whose actions dispatch to these callbacks.
    pub(crate) fn render_post(
        &self,
        post: &Post,
        state: &AppState,
        theme: &ThemeTokens,
        mode: CardMode,
    ) -> AnyElement {
        let target = CardTarget::resolve(post, state);
        let handlers = self.handlers_for(post, state, target, mode.selectable);
        render_post_item(post, state, theme, mode.is_focused, mode.style, handlers).into_any_element()
    }
}

/// The thought a card's actions act on, with its author's handle (for reply mentions).
#[derive(Clone)]
struct CardTarget {
    id: PostId,
    handle: String,
    /// The thought was written by one of the user's personas (sb E1: credences).
    human: bool,
    /// Where the target stands (archived, Y5).
    lifecycle: PostLifecycle,
    /// The "Archive automatically" choice it follows (Y6).
    auto_archive: Option<ArchiveAfter>,
    /// Its symbols (S7).
    symbols: crate::domain::symbols::PostSymbols,
    /// The version it shows once edited (V8).
    version: Option<crate::domain::data::history::SemVer>,
    /// The older major version it answered, once its parent moved on (V12).
    replied_to: Option<crate::domain::data::history::SemVer>,
    /// Its text is sealed (X4).
    locked: bool,
    facts: crate::ui::features::data::PostFacts, // N3, N6–N10: what its menu offers
}

impl CardTarget {
    fn resolve(post: &Post, state: &AppState) -> Self {
        let id = state.action_target(&post.id);
        let author = state.get_post(&id).map_or_else(|| state.author_of(post), |p| state.author_of(p));
        let target = state.get_post(&id).unwrap_or(post);
        let (lifecycle, auto_archive) = (target.lifecycle, ArchiveAfter::of(target.created_at, target.lifecycle.archive_at));
        Self { id, handle: author.handle.clone(), human: author.is_human_persona(), lifecycle, auto_archive, symbols: target.extras.symbols.clone(), version: target.extras.head.as_ref().map(|head| head.version), replied_to: replied_to(target, state), locked: target.is_locked(), facts: crate::ui::features::data::PostFacts::of(target, state) }
    }
}

/// The version `reply` answered, when its parent has had a major edit since (V12).
fn replied_to(reply: &Post, state: &AppState) -> Option<crate::domain::data::history::SemVer> {
    let parent = state.get_post(reply.reply_to_id.as_ref()?)?;
    let now = parent.extras.head.as_ref().map_or_else(Default::default, |head| head.version);
    crate::domain::data::history::replied_to_older(reply.extras.replied_version, now)
}
