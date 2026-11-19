//! Learning on posts and passages (sb C1b): the post "…" menu's Learn submenu and the
//! palette open the learn sheet on a post (a new question or answer, or the item it already
//! has); a vault entry's "Learn passages" opens it on the document's sentences and
//! paragraphs; saving makes or rewrites items; stopping removes a post's item (its log stays).

use super::super::review_host::review_today;
use super::super::view::AppView;
use crate::domain::preferences::ToastKind;
use crate::domain::review::item::{edited_item, new_item, NewItemContext};
use crate::domain::review::{Anchor, AnchorSide, ItemDraft, ItemId, Subject};
use crate::domain::PostId;
use crate::i18n::t_count;
use crate::ui::features::post::LearnRequestKind;
use crate::ui::features::review::{LearnRequest, LearnSource, Passage};
use chrono::Utc;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Post "…" menu › Learn › … on `post_id`.
    pub(crate) fn learn_post_menu(&mut self, post_id: PostId, kind: LearnRequestKind, window: &mut Window, cx: &mut Context<Self>) {
        match kind {
            LearnRequestKind::AddQuestion => self.learn_post(post_id, AnchorSide::Back, window, cx),
            LearnRequestKind::AddAnswer => self.learn_post(post_id, AnchorSide::Front, window, cx),
            LearnRequestKind::Edit => self.learn_post(post_id, AnchorSide::Back, window, cx),
            LearnRequestKind::Stop => self.stop_learning_post(&post_id, window, cx),
            LearnRequestKind::ThreadConnections => self.map_from_thread(&post_id, window, cx),
        }
    }

    fn open_learn_sheet(&mut self, request: LearnRequest, window: &mut Window, cx: &mut Context<Self>) {
        self.review.learn.update(cx, |sheet, cx| sheet.open(request, window, cx));
        cx.notify();
    }

    /// The sheet on `post_id`: its item when it is already learnt, else a new `side`.
    pub(in crate::ui::features::shell) fn learn_post(&mut self, post_id: PostId, side: AnchorSide, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(existing) = self.review.deck.on_post(&self.state.active_network_id, post_id.as_str()).map(|item| item.id.clone()) {
            return self.edit_review_item(&existing, window, cx);
        }
        let Some(view) = self.post_view(post_id.as_str()) else { return };
        self.open_learn_sheet(LearnRequest::new(LearnSource::Post(view), side), window, cx);
    }

    /// The palette's "Learn this post": the thought open in the inspector, else the picker.
    pub(crate) fn learn_focused_post(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.feed.selected_thread_post_id.clone() {
            Some(post_id) => self.learn_post(post_id, AnchorSide::Back, window, cx),
            None => self.open_review_tab_picker(crate::ui::features::review::PickerMode::Learn, window, cx),
        }
    }

    /// The sheet on an anchored item, its fields filled.
    pub(in crate::ui::features::shell) fn edit_review_item(&mut self, id: &ItemId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(item) = self.review.deck.item(id).cloned() else { return };
        let Subject::Anchored { anchor, side, written } = item.subject else { return };
        let (source, passage) = match anchor {
            Anchor::Post { id } => (self.post_view(&id).map(LearnSource::Post), None),
            Anchor::Passage { document_id, locator, text } => (self.document_source(&document_id), Some(Passage { text, locator })),
        };
        let Some(source) = source else { return };
        let request = LearnRequest { source, existing: Some(item.id), side, written, tags: item.tags, passage };
        self.open_learn_sheet(request, window, cx);
    }

    /// A vault entry's "Learn passages".
    pub(crate) fn learn_passages(&mut self, doc_id: &str, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(source) = self.document_source(doc_id) {
            self.open_learn_sheet(LearnRequest::new(source, AnchorSide::Back), window, cx);
        }
    }

    /// Stops learning `post_id`: its item is removed, its answers stay in the statistics.
    fn stop_learning_post(&mut self, post_id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(id) = self.review.deck.on_post(&self.state.active_network_id, post_id.as_str()).map(|item| item.id.clone()) else { return };
        self.delete_review_item(&id, cx);
        self.toast(ToastKind::Confirmation, Notification::success(t!("review.learn.stopped").to_string()), window, cx);
    }

    /// Makes (or rewrites) the items of `drafts`, closes the sheet and confirms.
    pub(in crate::ui::features::shell) fn save_learn_drafts(&mut self, drafts: Vec<ItemDraft>, existing: Option<ItemId>, window: &mut Window, cx: &mut Context<Self>) {
        let context = NewItemContext { network_id: self.state.active_network_id.clone(), created_at: Utc::now(), today: review_today() };
        let edited = existing.and_then(|id| self.review.deck.item(&id).cloned());
        let items: Vec<_> = match edited {
            Some(item) => drafts.first().and_then(|draft| edited_item(&item, draft)).into_iter().collect(),
            None => drafts.iter().filter_map(|draft| new_item(draft, &context, ItemId::generate())).collect(),
        };
        let count = items.len();
        self.store_review_items(items, cx);
        self.review.learn.update(cx, |sheet, cx| sheet.close(cx));
        self.toast(ToastKind::Confirmation, Notification::success(t_count("review.learn.saved", count)), window, cx);
    }
}
