//! The right-click menu of a roster row (M1): the overview's actions on that profile: open
//! it, edit it in the Forge, run a turn (an agent) or switch to it (a persona), and remove it
//! after the same confirmation.

use super::overview::confirm_remove;
use super::params::{ProfileCallback, RosterCallbacks};
use crate::domain::{Profile, ProfileId};
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{MenuItem, MenuSection, MenuSpec, MenuText};

fn profile_item(key: &'static str, icon: LucideIcon, id: &ProfileId, run: &ProfileCallback) -> MenuItem {
    let (id, run) = (id.clone(), run.clone());
    MenuItem::new(format!("roster.{key}"), MenuText::key(key)).icon(icon).on_click(move |window, cx| run(id.clone(), window, cx))
}

/// The menu of `profile`'s row.
pub(super) fn profile_menu_spec(profile: &Profile, callbacks: &RosterCallbacks) -> MenuSpec {
    let id = &profile.id;
    let primary = match profile.is_agent() {
        true => profile_item("trigger.open", LucideIcon::Zap, id, &callbacks.trigger_agent),
        false => profile_item("lab.use_persona", LucideIcon::UserCheck, id, &callbacks.activate_persona),
    };
    let open = MenuSection::new().entry(profile_item("data.saved.open", LucideIcon::ExternalLink, id, &callbacks.select)).entry(profile_item("lab.edit_in_forge", LucideIcon::Edit, id, &callbacks.edit)).entry(primary);
    let (name, remove_id, delete) = (profile.name.clone(), id.clone(), callbacks.delete.clone());
    let remove = MenuItem::new("roster.delete", MenuText::key("lab.delete_tooltip")).icon(LucideIcon::Trash).destructive().on_click(move |window, cx| confirm_remove(name.clone(), remove_id.clone(), delete.clone(), window, cx));
    MenuSpec::new().section(open).section(MenuSection::new().entry(remove))
}
