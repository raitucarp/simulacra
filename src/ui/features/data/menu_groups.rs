//! The post's data items in the "…" menu, grouped so the menu fits a window: Edit and
//! Bookmark stay at the top level; keeping and sorting the post ("Organize") and working on
//! its ideas ("Think it over") open as submenus. A flat list of every item ran taller than
//! the window and asked people to scan two dozen entries for the one they wanted.

use super::events::DataEvent;
use super::post_menu::{collections_submenu, item, DataControls};
use super::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

/// `menu` with the post's data items (unchanged when the host offers none).
pub fn with_data_items(menu: PopupMenu, controls: Option<&DataControls>, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let Some(controls) = controls.cloned() else { return menu };
    let post = controls.post_id.clone();
    let mut menu = menu.separator();
    if controls.own {
        menu = menu.item(item(tr("data.menu.edit"), LucideIcon::Pencil, DataEvent::EditPost(post.clone()), &controls.on_event));
    }
    let (label, icon) = if controls.bookmarked { (tr("data.menu.unbookmark"), LucideIcon::BookmarkCheck) } else { (tr("data.menu.bookmark"), LucideIcon::Bookmark) };
    menu = menu.item(item(label, icon, DataEvent::ToggleBookmark(post), &controls.on_event));
    let organize = controls.clone();
    menu = menu.submenu_with_icon(Some(Icon::new(LucideIcon::Archive)), tr("data.menu.organize"), window, cx, move |submenu, window, cx| organize_items(submenu, &organize, window, cx));
    menu.submenu_with_icon(Some(Icon::new(LucideIcon::Lightbulb)), tr("data.menu.think"), window, cx, move |submenu, window, cx| think_items(submenu, &controls, window, cx))
}

/// Archive and its timers, the entry's state, the garden, a pinned quote, series and audience
/// (own posts), the edit history and the collections.
fn organize_items(menu: PopupMenu, controls: &DataControls, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let mut menu = menu;
    if controls.own {
        menu = super::lifecycle_menu::with_lifecycle_items(menu, controls, window, cx); // Y5, Y6
        menu = super::state_menu::with_state_items(menu, controls, window, cx); // S7
        menu = super::garden_menu::with_garden_items(menu, controls, window, cx); // N3
        menu = super::idea_menu::with_idea_items(menu, controls, window, cx); // N7–N10
    }
    if controls.revisions > 0 {
        let label = t!("data.menu.history", count = controls.revisions).to_string();
        menu = menu.item(item(label, LucideIcon::RotateCcwClock, DataEvent::OpenPostHistory(controls.post_id.clone()), &controls.on_event));
    }
    let scoped = controls.clone();
    menu.submenu_with_icon(Some(Icon::new(LucideIcon::FolderPlus)), tr("data.menu.add_to_collection"), window, cx, move |submenu, _, _| collections_submenu(submenu, &scoped))
}

/// Corrections, transcripts and summaries, glossary, lenses and translation, asking the room,
/// a margin reply and a what-if fork.
fn think_items(menu: PopupMenu, controls: &DataControls, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let menu = super::care_menu::with_care_items(menu, controls); // N11–N15
    let menu = super::thought_menu::with_thought_items(menu, controls, window, cx); // N16–N20
    if controls.locked {
        return menu;
    }
    let post = controls.post_id.clone();
    let menu = match controls.own && !controls.facts.asked {
        true => menu.item(item(tr("posts.question.menu"), LucideIcon::HelpCircle, DataEvent::Question(super::question::QuestionEvent::Ask(post.clone())), &controls.on_event)), // N9
        false => menu,
    };
    menu.separator()
        .item(item(tr("post.margin.menu"), LucideIcon::MessageSquareReply, DataEvent::ReplyInMargin(post.clone()), &controls.on_event)) // N2
        .item(item(tr("posts.branch.menu"), LucideIcon::GitBranch, DataEvent::Branch(super::branch_sheet::BranchEvent::Fork(post)), &controls.on_event)) // N6
}
