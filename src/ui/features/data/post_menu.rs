//! A post's data controls: in the "…" menu, "Edit" (the user's own posts), "History" (once
//! edited), "Reply in the margin…" (N2, never on a locked post), "Bookmark" and the "Add to
//! collection" submenu. The card's "edited" mark is `edited_mark`.

use super::events::{DataAction, DataEvent};
use super::text::tr;
use crate::domain::document::CollectionId;
use crate::domain::post::PostLifecycle;
use crate::domain::preferences::ArchiveAfter;
use crate::domain::PostId;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::{PopupMenu, PopupMenuItem};
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// What the feed knows to build every card's data controls.
#[derive(Clone)]
pub struct DataSource {
    /// Earlier versions per post id.
    pub revisions: Rc<HashMap<String, usize>>,
    pub bookmarks: Rc<HashSet<String>>,
    /// The active network's collections, by name.
    pub collections: Rc<Vec<(CollectionId, String)>>,
    /// The collections holding each post, by post id.
    pub memberships: Rc<HashMap<String, Vec<CollectionId>>>,
    /// The history sheet is open (the "edited" marks drop their tooltips), sb P16.
    pub history_open: bool,
    pub on_event: DataAction,
}

impl DataSource {
    /// The controls of `post`'s card; `own` when the user wrote it (it can be edited).
    pub fn controls(&self, post: &PostId, own: bool) -> DataControls {
        DataControls {
            post_id: post.clone(),
            own,
            revisions: self.revisions.get(post.as_str()).copied().unwrap_or_default(),
            bookmarked: self.bookmarks.contains(post.as_str()),
            collections: self.collections.clone(),
            member_of: self.memberships.get(post.as_str()).cloned().unwrap_or_default(),
            history_open: self.history_open,
            lifecycle: PostLifecycle::default(),
            auto_archive: None,
            symbols: Default::default(),
            version: None,
            replied_to: None,
            locked: false,
            on_event: self.on_event.clone(),
        }
    }
}

/// The data controls of one post card.
#[derive(Clone)]
pub struct DataControls {
    pub post_id: PostId,
    pub own: bool,
    pub revisions: usize,
    pub bookmarked: bool,
    pub collections: Rc<Vec<(CollectionId, String)>>,
    pub member_of: Vec<CollectionId>,
    pub history_open: bool,
    /// Where the post stands (archived, Y5); set by the feed from the post itself.
    pub lifecycle: PostLifecycle,
    /// The "Archive automatically" choice the post follows, if any (Y6).
    pub auto_archive: Option<ArchiveAfter>,
    /// The post's symbols (S7: its entry's state moves); set by the feed.
    pub symbols: crate::domain::symbols::PostSymbols,
    /// The version the post shows once edited (V8); set by the feed.
    pub version: Option<crate::domain::data::history::SemVer>,
    /// The older major version this reply answered, once its parent moved on (V12).
    pub replied_to: Option<crate::domain::data::history::SemVer>,
    /// Its text is sealed (X4): nothing quotes it (N2); set by the feed.
    pub locked: bool,
    pub on_event: DataAction,
}

impl DataControls {
    /// Whether the "edited" mark offers its tooltip: not while the history sheet is open, so a
    /// tooltip that was pending when the mark was clicked never shows over the sheet.
    pub fn edited_tooltip(&self) -> bool {
        !self.history_open
    }

    /// The card's edited mark: the version it shows (`v1.2.0`, V8), or "edited" when the
    /// number is unknown; `None` before any edit.
    pub fn edited_mark(&self) -> Option<String> {
        (self.revisions > 0).then(|| self.version.map_or_else(|| tr("data.edited"), crate::ui::features::composer::version_label))
    }
}

pub(super) fn item(label: String, icon: LucideIcon, event: DataEvent, on_event: &DataAction) -> PopupMenuItem {
    let on_event = on_event.clone();
    PopupMenuItem::new(label).icon(Icon::new(icon)).on_click(move |_, window, cx| on_event(event.clone(), window, cx))
}

fn collections_submenu(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let post = &controls.post_id;
    let menu = controls.collections.iter().fold(menu, |menu, (id, name)| {
        let member = controls.member_of.contains(id);
        let event = if member { DataEvent::RemoveFromCollection(post.clone(), id.clone()) } else { DataEvent::AddToCollection(post.clone(), id.clone()) };
        let icon = if member { LucideIcon::Check } else { LucideIcon::Folder };
        menu.item(item(name.clone(), icon, event, &controls.on_event))
    });
    let menu = if controls.collections.is_empty() { menu } else { menu.separator() };
    menu.item(item(tr("data.menu.new_collection"), LucideIcon::FolderPlus, DataEvent::NewCollectionFor(post.clone()), &controls.on_event))
}

/// `menu` with the post's data items (unchanged when the host offers none).
pub fn with_data_items(menu: PopupMenu, controls: Option<&DataControls>, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let Some(controls) = controls.cloned() else { return menu };
    let post = controls.post_id.clone();
    let mut menu = menu.separator();
    if controls.own {
        menu = menu.item(item(tr("data.menu.edit"), LucideIcon::Pencil, DataEvent::EditPost(post.clone()), &controls.on_event));
        menu = super::lifecycle_menu::with_lifecycle_items(menu, &controls, window, cx); // Y5, Y6
        menu = super::state_menu::with_state_items(menu, &controls, window, cx); // S7
    }
    if controls.revisions > 0 {
        let label = t!("data.menu.history", count = controls.revisions).to_string();
        menu = menu.item(item(label, LucideIcon::RotateCcwClock, DataEvent::OpenPostHistory(post.clone()), &controls.on_event));
    }
    if !controls.locked {
        menu = menu.item(item(tr("post.margin.menu"), LucideIcon::MessageSquareReply, DataEvent::ReplyInMargin(post.clone()), &controls.on_event)); // N2
    }
    let (label, icon) = if controls.bookmarked { (tr("data.menu.unbookmark"), LucideIcon::BookmarkCheck) } else { (tr("data.menu.bookmark"), LucideIcon::Bookmark) };
    let menu = menu.item(item(label, icon, DataEvent::ToggleBookmark(post), &controls.on_event));
    let icon = Some(Icon::new(LucideIcon::FolderPlus));
    menu.submenu_with_icon(icon, tr("data.menu.add_to_collection"), window, cx, move |submenu, _, _| collections_submenu(submenu, &controls))
}
