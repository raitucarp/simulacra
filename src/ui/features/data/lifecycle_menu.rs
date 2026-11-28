//! The lifecycle entries of the user's own post: Archive or Unarchive (Y5), the "Archive
//! automatically" submenu (Y6) with its current choice ticked, and the "Delete
//! automatically" submenu (Y7), "Never" ticked while the post keeps itself.

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use crate::domain::post::auto_delete::DeleteAfter;
use crate::domain::preferences::{ArchiveAfter, Choice};
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuEntry, MenuItem, MenuSpec, MenuText, Submenu};

/// Prefix of the auto-archive option labels (shared with the Writing setting).
const AUTO_ARCHIVE_PREFIX: &str = "prefs.option.auto_archive";
/// Prefix of the auto-delete option labels.
const AUTO_DELETE_PREFIX: &str = "data.auto_delete";

fn auto_archive_submenu(controls: &DataControls) -> Submenu {
    let choices = ArchiveAfter::ALL.iter().map(|after| {
        let label = MenuText::key(format!("{AUTO_ARCHIVE_PREFIX}.{}", after.key()));
        let event = DataEvent::SetAutoArchive(controls.post_id.clone(), *after);
        item(format!("{}{}", ids::AUTO_ARCHIVE_PREFIX, after.key()), label, LucideIcon::Clock, event, &controls.on_event).checked(controls.auto_archive == Some(*after))
    });
    Submenu::new(ids::AUTO_ARCHIVE, MenuText::key("data.menu.auto_archive"), MenuSpec::of(choices)).icon(LucideIcon::Clock)
}

/// "Never" (ticked while the post keeps itself) and every delay, counted from now.
pub(super) fn auto_delete_submenu(controls: &DataControls) -> Option<Submenu> {
    if !controls.own {
        return None;
    }
    let post = &controls.post_id;
    let never = item(ids::AUTO_DELETE_NEVER, MenuText::key("data.auto_delete.never"), LucideIcon::Timer, DataEvent::SetAutoDelete(post.clone(), None), &controls.on_event).checked(controls.lifecycle.delete_at.is_none());
    let delays = DeleteAfter::ALL.iter().map(|after| {
        let label = MenuText::key(format!("{AUTO_DELETE_PREFIX}.{}", after.key()));
        item(format!("{}{}", ids::AUTO_DELETE_PREFIX, after.key()), label, LucideIcon::Timer, DataEvent::SetAutoDelete(post.clone(), Some(*after)), &controls.on_event)
    });
    let spec = MenuSpec::of(std::iter::once(never).chain(delays));
    Some(Submenu::new(ids::AUTO_DELETE, MenuText::key("data.menu.auto_delete"), spec).icon(LucideIcon::Timer).destructive())
}

/// Archive (or Unarchive) and, while the post is in the feed, "Archive automatically".
pub(super) fn lifecycle_entries(controls: &DataControls) -> Vec<MenuEntry> {
    if !controls.own {
        return Vec::new();
    }
    let archived = controls.lifecycle.is_archived();
    let (id, key, icon) = if archived { (ids::UNARCHIVE, "data.menu.unarchive", LucideIcon::ArchiveRestore) } else { (ids::ARCHIVE, "data.menu.archive", LucideIcon::Archive) };
    let archive: MenuItem = item(id, MenuText::key(key), icon, DataEvent::SetArchived(controls.post_id.clone(), !archived), &controls.on_event);
    let mut entries = vec![MenuEntry::from(archive)];
    if !archived {
        entries.push(auto_archive_submenu(controls).into());
    }
    entries
}
