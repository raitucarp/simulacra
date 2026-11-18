//! "…" menu › Actions (sb C2): every thinking action, grouped, each opening the actions sheet
//! with its answer for this thought.

use super::handlers::PickPostAction;
use crate::domain::post_actions::{ActionGroup, ActionKind};
use crate::ui::features::post_actions::{group_title, kind_icon, kind_title};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::{PopupMenu, PopupMenuItem};
use gpui_kit::component::Icon;
use gpui_kit::{Context, Window};
use rust_i18n::t;

/// `menu` with an "Actions" submenu first (unchanged when the host offers no actions).
pub(crate) fn with_actions_submenu(menu: PopupMenu, on_pick: Option<&PickPostAction>, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let Some(on_pick) = on_pick.cloned() else { return menu };
    let label = t!("post_actions.menu").to_string();
    let icon = Some(Icon::new(LucideIcon::WandSparkles));
    menu.submenu_with_icon(icon, label, window, cx, move |submenu, _, _| build_submenu(submenu, &on_pick)).separator()
}

fn build_submenu(menu: PopupMenu, on_pick: &PickPostAction) -> PopupMenu {
    ActionGroup::ALL.iter().enumerate().fold(menu, |menu, (index, group)| {
        let menu = if index == 0 { menu } else { menu.separator() };
        let menu = menu.label(group_title(*group));
        ActionKind::in_group(*group).fold(menu, |menu, kind| menu.item(action_item(kind, on_pick)))
    })
}

fn action_item(kind: ActionKind, on_pick: &PickPostAction) -> PopupMenuItem {
    let on_pick = on_pick.clone();
    PopupMenuItem::new(kind_title(kind)).icon(Icon::new(kind_icon(kind))).on_click(move |_, window, cx| on_pick(kind, window, cx))
}
