//! Connection maps (sb C1b): seeded from a post's thread (post "…" menu), from posts picked
//! on the Review page, or from a vault entry's related entries and linking posts ("Learn
//! connections"), opened in the Review page's editor, and saved as one review item.

use super::super::review_host::review_today;
use super::super::view::AppView;
use crate::domain::document::links::{backlinks, related_entries};
use crate::domain::review::connections::seed::{map_from_document, map_from_posts, thread_posts, DocumentNeighbourhood};
use crate::domain::review::connections::{ConnectionMap, MapPoint, MAX_POINTS};
use crate::domain::review::item::{new_map_item, NewItemContext};
use crate::domain::review::quiz::piece_text;
use crate::domain::review::{ItemId, ReviewItem, Subject};
use crate::domain::{Post, PostId};
use crate::state::NavigationTab;
use crate::ui::features::review::{PickerMode, PostView};
use jiff::Timestamp;
use gpui_kit::*;
use rust_i18n::t;
use std::collections::HashMap;
use std::rc::Rc;

/// Characters of a post naming a thread map.
const TITLE_CHARS: usize = 48;

impl AppView {
    /// Shows the Review page and opens its editor on `map`.
    pub(in crate::ui::features::shell) fn open_map_editor(&mut self, map: ConnectionMap, existing: Option<ItemId>, window: &mut Window, cx: &mut Context<Self>) {
        let posts: HashMap<String, PostView> = map.points.iter().filter_map(|p| match p {
            MapPoint::Post { id } => self.post_view(id).map(|view| (id.clone(), view)),
            _ => None,
        }).collect();
        self.select_nav(NavigationTab::Review, window, cx);
        self.review.panel.update(cx, |panel, cx| panel.open_map_editor(map, existing, Rc::new(posts), window, cx));
    }

    /// Shows the Review page with its post picker.
    pub(in crate::ui::features::shell) fn open_review_tab_picker(&mut self, mode: PickerMode, window: &mut Window, cx: &mut Context<Self>) {
        self.select_nav(NavigationTab::Review, window, cx);
        self.open_review_picker(mode, window, cx);
    }

    /// Post "…" menu › Learn › This thread's connections.
    pub(in crate::ui::features::shell) fn map_from_thread(&mut self, post_id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let line = thread_posts(self.state.posts(), post_id);
        let Some(first) = line.first() else { return };
        let title = t!("review.map.thread_title", start = piece_text(&first.content, TITLE_CHARS)).to_string();
        let map = map_from_posts(title, &line);
        self.open_map_editor(map, None, window, cx);
    }

    /// Posts picked on the Review page.
    pub(in crate::ui::features::shell) fn map_from_posts(&mut self, ids: &[PostId], window: &mut Window, cx: &mut Context<Self>) {
        let posts: Vec<&Post> = ids.iter().filter_map(|id| self.state.get_post(id)).collect();
        let map = map_from_posts(t!("review.map.selection_title").to_string(), &posts);
        self.open_map_editor(map, None, window, cx);
    }

    /// A vault entry's "Learn connections": the entry, its related entries and linking posts.
    pub(crate) fn learn_connections(&mut self, doc_id: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(doc) = self.state.documents.iter().find(|doc| doc.id == doc_id) else { return };
        let tree = self.state.library.tree();
        let related = related_entries(doc, &self.state.documents, &tree, &[], MAX_POINTS);
        let related_ids: Vec<String> = related.iter().map(|entry| entry.id.as_str().to_string()).collect();
        let around = DocumentNeighbourhood { document_id: &doc.id, related: related_ids.iter().map(String::as_str).collect(), linking_posts: backlinks(doc, &self.state.posts) };
        let documents = &self.state.documents;
        let related_pair = |a: &str, b: &str| documents.iter().find(|d| d.id == a).is_some_and(|da| related_entries(da, documents, &tree, &[], MAX_POINTS).iter().any(|r| r.id.as_str() == b));
        let map = map_from_document(t!("review.map.document_title", title = doc.title.as_str()).to_string(), &around, &related_pair);
        self.open_map_editor(map, None, window, cx);
    }

    /// Saves a map: a new item, or the edited item's map (its schedule kept).
    pub(in crate::ui::features::shell) fn save_connection_map(&mut self, id: Option<ItemId>, map: ConnectionMap, _: &mut Window, cx: &mut Context<Self>) {
        let existing = id.and_then(|id| self.review.deck.item(&id).cloned());
        let item = match existing {
            Some(item) => ReviewItem { subject: Subject::Connections { map }, ..item },
            None => {
                let context = NewItemContext { network_id: self.state.active_network_id.clone(), created_at: Timestamp::now(), today: review_today() };
                new_map_item(map, &context, ItemId::generate())
            }
        };
        self.store_review_items(vec![item], cx);
    }
}
