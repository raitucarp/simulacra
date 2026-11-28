//! A post's data controls: what the feed knows of every card (revisions, bookmarks,
//! collections and the facts its menu needs), the item helper of the menu entries built in
//! this module, and the "Add to collection" submenu. The card's "edited" mark is
//! `edited_mark`.

use super::events::{DataAction, DataEvent};
use super::text::tr;
use crate::domain::document::CollectionId;
use crate::domain::post::PostLifecycle;
use crate::domain::preferences::ArchiveAfter;
use crate::domain::PostId;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuItem, MenuSection, MenuSpec, MenuText, Submenu};
use gpui_kit::SharedString;
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
    /// N5: the depth dial of every post.
    pub depth: Rc<super::depth_dial::DepthView>,
    /// N6–N10: what every card's menu shares.
    pub ideas: Rc<super::post_facts::IdeaSource>,
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
            facts: Default::default(),
            ideas: self.ideas.clone(),
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
    /// What the menu offers beyond the data (N3 garden stage, N6–N10); set by the feed.
    pub facts: super::post_facts::PostFacts,
    pub ideas: Rc<super::post_facts::IdeaSource>,
    pub on_event: DataAction,
}

impl DataControls {
    /// Whether the "edited" mark offers its tooltip: not while the history sheet is open, so a
    /// tooltip that was pending when the mark was clicked never shows over the sheet.
    pub fn edited_tooltip(&self) -> bool {
        !self.history_open
    }

    /// The card's edited mark, "edited" (V8); `None` before any edit. The version number
    /// (`v1.2.0`) reads as software jargon on a thought, so it stays in the history sheet.
    pub fn edited_mark(&self) -> Option<String> {
        (self.revisions > 0).then(|| tr("data.edited"))
    }
}

pub(super) fn item(id: impl Into<SharedString>, label: MenuText, icon: LucideIcon, event: DataEvent, on_event: &DataAction) -> MenuItem {
    let on_event = on_event.clone();
    MenuItem::new(id, label).icon(icon).on_click(move |window, cx| on_event(event.clone(), window, cx))
}

/// "Add to collection": every collection of the network, ticked when it holds the post (a
/// click adds or removes it), then "New collection…".
pub(super) fn collections_submenu(controls: &DataControls) -> Submenu {
    let post = &controls.post_id;
    let toggles = controls.collections.iter().map(|(id, name)| {
        let member = controls.member_of.contains(id);
        let event = if member { DataEvent::RemoveFromCollection(post.clone(), id.clone()) } else { DataEvent::AddToCollection(post.clone(), id.clone()) };
        item(ids::COLLECTION_TOGGLE, MenuText::plain(name.clone()), LucideIcon::Folder, event, &controls.on_event).checked(member)
    });
    let new = item(ids::COLLECTION_NEW, MenuText::key("data.menu.new_collection"), LucideIcon::FolderPlus, DataEvent::NewCollectionFor(post.clone()), &controls.on_event);
    let spec = MenuSpec::new().section(MenuSection::new().entries(toggles)).section(MenuSection::new().entry(new)).scrollable();
    Submenu::new(ids::COLLECTIONS, MenuText::key("data.menu.add_to_collection"), spec).icon(LucideIcon::FolderPlus)
}
