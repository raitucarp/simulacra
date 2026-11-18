//! A post's data controls: in the "…" menu, "Edit" (the user's own posts), "History" (once
//! edited), "Bookmark" and the "Add to collection" submenu; on the card, a quiet "edited"
//! mark that opens the history.

use super::events::{DataAction, DataEvent};
use super::text::tr;
use crate::domain::document::CollectionId;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::{PopupMenu, PopupMenuItem};
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

const EDITED_ID_PREFIX: &str = "post-edited-";

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
    pub on_event: DataAction,
}

fn item(label: String, icon: LucideIcon, event: DataEvent, on_event: &DataAction) -> PopupMenuItem {
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
    }
    if controls.revisions > 0 {
        let label = t!("data.menu.history", count = controls.revisions).to_string();
        menu = menu.item(item(label, LucideIcon::RotateCcwClock, DataEvent::OpenPostHistory(post.clone()), &controls.on_event));
    }
    let (label, icon) = if controls.bookmarked { (tr("data.menu.unbookmark"), LucideIcon::BookmarkCheck) } else { (tr("data.menu.bookmark"), LucideIcon::Bookmark) };
    let menu = menu.item(item(label, icon, DataEvent::ToggleBookmark(post), &controls.on_event));
    let icon = Some(Icon::new(LucideIcon::FolderPlus));
    menu.submenu_with_icon(icon, tr("data.menu.add_to_collection"), window, cx, move |submenu, _, _| collections_submenu(submenu, &controls))
}

/// The quiet "edited" mark of an edited post, opening its history; `None` before any edit.
pub fn render_edited_mark(controls: &DataControls, theme: &ThemeTokens) -> Option<AnyElement> {
    if controls.revisions == 0 {
        return None;
    }
    let (on_event, post) = (controls.on_event.clone(), controls.post_id.clone());
    let hover = theme.text_secondary;
    let mark = div()
        .id(SharedString::from(format!("{EDITED_ID_PREFIX}{post}")))
        .flex_none()
        .cursor_pointer()
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .hover(move |style| style.text_color(hover))
        .tooltip(move |window, cx| gpui_kit::component::tooltip::Tooltip::new(tr("data.edited_tooltip")).build(window, cx))
        .on_click(move |_, window, cx| on_event(DataEvent::OpenPostHistory(post.clone()), window, cx))
        .child(tr("data.edited"));
    Some(mark.into_any_element())
}
