//! Post menu › Think with AI (sb C2, M1): one submenu per action group (Understand,
//! Challenge, Explore), each action opening the actions sheet with its answer for this
//! thought.

use super::handlers::PickPostAction;
use crate::domain::post_actions::{ActionGroup, ActionKind};
use crate::ui::features::post_actions::{group_icon, kind_icon};
use crate::ui::menus::{post_ids as ids, MenuItem, MenuSpec, MenuText, Submenu};

fn action_item(kind: ActionKind, on_pick: &PickPostAction) -> MenuItem {
    let on_pick = on_pick.clone();
    let label = MenuText::key(format!("post_actions.kind.{}", kind.key()));
    MenuItem::new(format!("{}{}", ids::AI_PREFIX, kind.key()), label).icon(kind_icon(kind)).on_click(move |window, cx| on_pick(kind, window, cx))
}

/// The group submenus (none when the host offers no actions, as on a locked post).
pub(crate) fn action_group_submenus(on_pick: Option<&PickPostAction>) -> Vec<Submenu> {
    let Some(on_pick) = on_pick else { return Vec::new() };
    ActionGroup::ALL.iter().map(|group| {
        let items = ActionKind::in_group(*group).map(|kind| action_item(kind, on_pick));
        let label = MenuText::key(format!("post_actions.group.{}", group.key()));
        Submenu::new(format!("{}{}", ids::AI_GROUP_PREFIX, group.key()), label, MenuSpec::of(items)).icon(group_icon(*group))
    }).collect()
}
