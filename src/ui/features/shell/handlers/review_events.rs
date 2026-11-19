//! What the Review page asks for (sb C1, items since sb C1b): answers applied to the deck
//! and saved, anchors opened, items suspended or deleted, and the page's data with the posts
//! and document titles its items and maps show.

use super::super::review_host::{learning_policy, review_today};
use super::super::view::AppView;
use crate::domain::review::{apply_answer, Anchor, Answer, ItemId, ReviewItem};
use crate::domain::PostId;
use super::learning_events::learning_post_ids;
use crate::ui::features::review::learning::LearningData;
use crate::ui::features::review::{PickerMode, PostView, PostViews, ReviewData, ReviewEvent, ReviewPanel};
use jiff::Timestamp;
use jiff::civil::Date;
use gpui_kit::*;
use std::collections::HashMap;
use std::rc::Rc;

/// Most posts the picker lists from (newest first).
const PICKER_POSTS: usize = 1_000;

impl AppView {
    pub(in crate::ui::features::shell) fn handle_review_event(&mut self, _: &Entity<ReviewPanel>, event: &ReviewEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event.clone() {
            ReviewEvent::Answered(answer) => self.apply_review_answer(answer, cx),
            ReviewEvent::OpenAnchor(anchor) => self.open_anchor(anchor, window, cx),
            ReviewEvent::RequestQuiz(kind) => self.start_review_quiz(kind, window, cx),
            ReviewEvent::OpenPicker(mode) => self.open_review_picker(mode, window, cx),
            ReviewEvent::LearnPost(id, side) => self.learn_post(id, side, window, cx),
            ReviewEvent::EditItem(id) => self.edit_review_item(&id, window, cx),
            ReviewEvent::MapFromPosts(ids) => self.map_from_posts(&ids, window, cx),
            ReviewEvent::SaveMap { id, map } => self.save_connection_map(id, map, window, cx),
            ReviewEvent::SetSuspended(id, suspended) => self.update_review_item(&id, |item| ReviewItem { suspended, ..item }, cx),
            ReviewEvent::DeleteItem(id) => self.delete_review_item(&id, cx),
            ReviewEvent::Learning(event) => self.handle_learning_event(event, window, cx),
        }
    }

    /// `post` with its author, as the stream shows it.
    pub(in crate::ui::features::shell) fn post_view(&self, id: &str) -> Option<PostView> {
        self.state.get_post_by_id(id).map(|post| PostView { post: post.clone(), author: self.state.author_of(post).clone() })
    }

    /// Views of the posts `items` refer to (anchors and map points).
    fn referenced_posts(&self, items: &[ReviewItem], learning: &LearningData) -> PostViews {
        let anchors = items.iter().filter_map(|item| item.subject.anchor()).filter_map(Anchor::post_id);
        let points = items.iter().filter_map(|item| item.subject.map()).flat_map(|map| map.points.iter().filter_map(|p| p.anchor()?.post_id()));
        let learnt = learning_post_ids(learning).into_iter().map(PostId::new);
        let views: HashMap<String, PostView> = anchors.chain(points).chain(learnt).filter_map(|id| self.post_view(id.as_str()).map(|view| (id.as_str().to_string(), view))).collect();
        Rc::new(views)
    }

    /// The page's data: the active network's items and log, today, Settings › Learning, and
    /// the posts and document titles shown.
    pub(in crate::ui::features::shell) fn review_data(&self, today: Date) -> ReviewData {
        let deck = self.review.deck.of_network(&self.state.active_network_id);
        let (limits, policy) = learning_policy(&self.state.preferences().learning);
        let learning = self.learning_data(&deck, today, &policy);
        let posts = self.referenced_posts(&deck.items, &learning);
        let documents = Rc::new(self.state.current_documents().map(|doc| (doc.id.clone(), doc.title.clone())).collect());
        ReviewData { network_id: self.state.active_network_id.clone(), items: deck.items, log: deck.log, today, limits, policy, posts, documents, learning: Rc::new(learning) }
    }

    /// Applies `answer` to its item, logs it and queues both for the vault.
    fn apply_review_answer(&mut self, answer: Answer, cx: &mut Context<Self>) {
        let Some(item) = self.review.deck.item(&answer.item_id).cloned() else { return };
        let policy = learning_policy(&self.state.preferences().learning).1;
        let (item, entry) = apply_answer(&item, &answer, Timestamp::now(), review_today(), &policy);
        if let Some(db) = &self.state.db {
            let _ = db.save_review_items(vec![item.clone()]);
            let _ = db.append_review_entry(entry.clone());
        }
        self.review.deck = std::mem::take(&mut self.review.deck).with_item(item).with_entry(entry);
        self.review_changed(cx);
    }

    /// Saves `items` (new or replaced) to the deck and the vault.
    pub(in crate::ui::features::shell) fn store_review_items(&mut self, items: Vec<ReviewItem>, cx: &mut Context<Self>) {
        if let Some(db) = &self.state.db {
            let _ = db.save_review_items(items.clone());
        }
        self.review.deck = items.into_iter().fold(std::mem::take(&mut self.review.deck), |deck, item| deck.with_item(item));
        self.review_changed(cx);
    }

    /// Replaces item `id` with `change(item)` and saves it.
    fn update_review_item(&mut self, id: &ItemId, change: impl FnOnce(ReviewItem) -> ReviewItem, cx: &mut Context<Self>) {
        if let Some(item) = self.review.deck.item(id).cloned().map(change) {
            self.store_review_items(vec![item], cx);
        }
    }

    /// Removes item `id` (its log stays for the statistics).
    pub(in crate::ui::features::shell) fn delete_review_item(&mut self, id: &ItemId, cx: &mut Context<Self>) {
        self.review.deck = std::mem::take(&mut self.review.deck).without_item(id);
        if let Some(db) = &self.state.db {
            let _ = db.delete_review_item(id);
        }
        self.review_changed(cx);
    }

    /// Opens the post or vault document an item is anchored to.
    fn open_anchor(&mut self, anchor: Anchor, window: &mut Window, cx: &mut Context<Self>) {
        match anchor {
            Anchor::Post { id } => self.reveal_post(PostId::new(id), window, cx),
            Anchor::Passage { document_id, .. } => self.open_vault_document(&document_id, window, cx),
        }
    }

    /// The picker over the network's posts, newest first.
    pub(in crate::ui::features::shell) fn open_review_picker(&mut self, mode: PickerMode, window: &mut Window, cx: &mut Context<Self>) {
        let mut posts: Vec<PostView> = self.state.current_network_posts().filter(|post| !post.is_instant_repost()).map(|post| PostView { post: post.clone(), author: self.state.author_of(post).clone() }).collect();
        posts.sort_by_key(|a| std::cmp::Reverse(a.post.created_at));
        posts.truncate(PICKER_POSTS);
        self.review.panel.update(cx, |panel, cx| panel.open_picker(mode, posts, window, cx));
    }
}
