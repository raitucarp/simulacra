//! The language menu of the title bar (M1): every language under its native name, the
//! current one ticked, on the kit's popup menu (arrow keys, Enter, Escape, scrolling).

use crate::i18n::Language;
use crate::ui::menus::{MenuItem, MenuSection, MenuSpec, MenuText};
use gpui_kit::{App, Window};
use std::rc::Rc;

/// Applies the language picked in the menu.
pub type SelectLangCallback = Rc<dyn Fn(Language, &mut Window, &mut App) + 'static>;

/// The language menu, `current` ticked.
pub fn language_menu_spec(current: Language, on_select: &SelectLangCallback) -> MenuSpec {
    let items = Language::ALL.iter().map(|&language| {
        let on_select = on_select.clone();
        MenuItem::new(format!("language.{}", language.code()), MenuText::plain(language.display_label().to_string()))
            .checked(language == current)
            .on_click(move |window, cx| on_select(language, window, cx))
    });
    MenuSpec::new().section(MenuSection::labelled("language.select").entries(items)).scrollable()
}
