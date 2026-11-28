//! The view switcher: one "View" button carrying the active view's icon, opening a menu that
//! names every view the scope offers and ticks the active one. A row of sixteen bare icons
//! asked people to remember what each glyph meant; names in a menu only ask them to read.
//! The thread inspector's header and the profile page both show it.

use super::registry::{view_icon, view_label};
use crate::domain::views::{PostView, ViewScope};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::{DropdownMenu, PopupMenuItem};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::{div, Anchor, App, Div, ParentElement, SharedString, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

/// Picks a view.
pub type PickView = Rc<dyn Fn(PostView, &mut Window, &mut App) + 'static>;

/// Element id of `view`'s menu entry in a switcher whose ids start with `prefix`.
pub fn switcher_button_id(prefix: &str, view: PostView) -> SharedString {
    SharedString::from(format!("{prefix}-{}", view.key()))
}

/// Element id of the switcher's "View" button.
pub fn switcher_trigger_id(prefix: &str) -> SharedString {
    SharedString::from(format!("{prefix}-menu"))
}

/// The active view's entry carries the tick in place of its icon (the menu draws a tick
/// only where an entry has no icon of its own), as the other ticked menus of the app do.
fn menu_entry(view: PostView, active: PostView, on_pick: &PickView) -> PopupMenuItem {
    let on_pick = on_pick.clone();
    let entry = PopupMenuItem::new(view_label(view)).checked(view == active).on_click(move |_, window, cx| on_pick(view, window, cx));
    if view == active {
        entry
    } else {
        entry.icon(Icon::new(view_icon(view)))
    }
}

/// The "View" button of `scope`'s views, `active` shown on it and ticked in its menu.
pub fn render_view_switcher(scope: ViewScope, active: PostView, id_prefix: &str, on_pick: &PickView, theme: &ThemeTokens) -> Div {
    let views = PostView::in_scope(scope);
    let on_pick = on_pick.clone();
    let icon = if views.contains(&active) { view_icon(active) } else { LucideIcon::LayoutGrid };
    let trigger = Button::new(switcher_trigger_id(id_prefix))
        .ghost()
        .xsmall()
        .icon(Icon::new(icon).text_color(theme.accent_brand_text))
        .label(t!("views.switcher.label").to_string())
        .dropdown_caret(true)
        .tooltip(view_label(active))
        .dropdown_menu_with_anchor(Anchor::TopRight, move |menu, _, _| {
            views.iter().fold(menu, |menu, view| menu.item(menu_entry(*view, active, &on_pick)))
        });
    div().flex_none().child(trigger)
}
