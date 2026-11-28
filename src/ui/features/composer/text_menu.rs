//! M1: the composers' right-click menu: Cut, Copy and Paste (Cut and Copy only with a
//! selection), Select all, then the formatting marks the toolbar and shortcuts apply, each
//! dispatched as its action so the text area handles it as usual.

use super::format::format_label_key;
use super::format_keys::format_action;
use crate::domain::post::format::FormatMark;
use gpui_kit::base::input::{Copy as CopyText, Cut, Paste, SelectAll};
use gpui_kit::component::input::TextareaState;
use gpui_kit::component::native_menu::NativeMenu;
use gpui_kit::{App, Entity, Window};
use rust_i18n::t;

/// The menu builder of the draft in `input`.
pub(super) fn composer_text_menu(input: &Entity<TextareaState>) -> impl Fn(NativeMenu, &mut Window, &mut App) -> NativeMenu + 'static {
    let input = input.clone();
    move |menu, _, cx| {
        let selected = !input.read(cx).selected_range().is_empty();
        let format = FormatMark::ALL.into_iter().fold(NativeMenu::new(), |menu, mark| menu.menu(t!(format_label_key(mark)).to_string(), format_action(mark)));
        menu.menu_with_disabled(t!("menus.text.cut").to_string(), !selected, Box::new(Cut))
            .menu_with_disabled(t!("menus.text.copy").to_string(), !selected, Box::new(CopyText))
            .menu(t!("menus.text.paste").to_string(), Box::new(Paste))
            .separator()
            .menu(t!("menus.text.select_all").to_string(), Box::new(SelectAll))
            .separator()
            .submenu(t!("menus.text.format").to_string(), format)
    }
}
