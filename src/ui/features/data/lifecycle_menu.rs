//! The lifecycle items of the user's own post in its "…" menu: Archive or Unarchive (Y5) and
//! the "Archive automatically" submenu (Y6), its current choice ticked.

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::text::tr;
use crate::domain::preferences::{ArchiveAfter, Choice};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::component::Icon;
use gpui_kit::*;

/// Prefix of the auto-archive option labels (shared with the Writing setting).
const AUTO_ARCHIVE_PREFIX: &str = "prefs.option.auto_archive";

fn auto_archive_submenu(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    ArchiveAfter::ALL.iter().fold(menu, |menu, after| {
        let icon = if controls.auto_archive == Some(*after) { LucideIcon::Check } else { LucideIcon::Clock };
        let label = tr(&format!("{AUTO_ARCHIVE_PREFIX}.{}", after.key()));
        menu.item(item(label, icon, DataEvent::SetAutoArchive(controls.post_id.clone(), *after), &controls.on_event))
    })
}

/// `menu` with Archive (or Unarchive) and, while the post is in the feed, the
/// "Archive automatically" submenu.
pub(super) fn with_lifecycle_items(menu: PopupMenu, controls: &DataControls, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let archived = controls.lifecycle.is_archived();
    let (label, icon) = if archived { (tr("data.menu.unarchive"), LucideIcon::ArchiveRestore) } else { (tr("data.menu.archive"), LucideIcon::Archive) };
    let menu = menu.item(item(label, icon, DataEvent::SetArchived(controls.post_id.clone(), !archived), &controls.on_event));
    if archived {
        return menu;
    }
    let controls = controls.clone();
    let icon = Some(Icon::new(LucideIcon::Clock));
    menu.submenu_with_icon(icon, tr("data.menu.auto_archive"), window, cx, move |submenu, _, _| auto_archive_submenu(submenu, &controls))
}
