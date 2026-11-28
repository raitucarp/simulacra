//! The view switcher: one "View" button carrying the active view's icon, opening a menu that
//! names every view the scope offers, in three sections (Layouts, Lenses, Collections), and
//! ticks the active one. A row of sixteen bare icons asked people to remember what each
//! glyph meant; names in a menu only ask them to read. The thread inspector's header and the
//! profile page both show it.

use super::registry::{view_icon, view_label, view_menu_key, view_section, ViewSection};
use crate::domain::views::{PostView, ViewScope};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{render_menu, MenuItem, MenuSection, MenuSpec, MenuText};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::DropdownMenu;
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

fn menu_entry(view: PostView, active: PostView, on_pick: &PickView) -> MenuItem {
    let on_pick = on_pick.clone();
    MenuItem::new(format!("view.{}", view.key()), MenuText::key(view_menu_key(view))).icon(view_icon(view)).checked(view == active).on_click(move |window, cx| on_pick(view, window, cx))
}

/// The view menu of `scope`: its views by section, `active` ticked.
pub fn view_menu_spec(scope: ViewScope, active: PostView, on_pick: &PickView) -> MenuSpec {
    let views = PostView::in_scope(scope);
    ViewSection::ALL.iter().fold(MenuSpec::new(), |spec, section| {
        let entries = views.iter().filter(|view| view_section(**view) == *section).map(|view| menu_entry(*view, active, on_pick));
        spec.section(MenuSection::labelled(section.title_key()).entries(entries))
    })
}

/// The "View" button of `scope`'s views, `active` shown on it and ticked in its menu.
pub fn render_view_switcher(scope: ViewScope, active: PostView, id_prefix: &str, on_pick: &PickView, theme: &ThemeTokens) -> Div {
    let offered = PostView::in_scope(scope).contains(&active);
    let icon = if offered { view_icon(active) } else { LucideIcon::LayoutGrid };
    let on_pick = on_pick.clone();
    let trigger = Button::new(switcher_trigger_id(id_prefix))
        .ghost()
        .xsmall()
        .icon(Icon::new(icon).text_color(theme.accent_brand_text))
        .label(t!("views.switcher.label").to_string())
        .dropdown_caret(true)
        .tooltip(view_label(active))
        .dropdown_menu_with_anchor(Anchor::TopRight, move |menu, window, cx| render_menu(menu, &view_menu_spec(scope, active, &on_pick), window, cx));
    div().flex_none().child(trigger)
}
