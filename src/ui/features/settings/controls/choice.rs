//! One-of-many controls: chips for short option lists, a menu button for long ones, and the
//! persona picker.

use super::super::callbacks::EditAction;
use super::super::catalog::SettingRow;
use super::super::consts::MAX_INLINE_CHOICES;
use super::super::row::{element_id_of, RowContext};
use super::labels::option_label;
use crate::domain::preferences::{SettingField, SettingValue};
use crate::ui::components::{chip_row, choice_chip};
use gpui_kit::component::button::Button;
use gpui_kit::component::menu::{DropdownMenu, PopupMenuItem};
use gpui_kit::component::{Disableable, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const MENU_PART: &str = "menu";

/// `(value, label)` options of a menu.
type MenuOptions = Vec<(SettingValue, String)>;

/// Chips (short lists) or a menu (long lists) for a choice preference.
pub(super) fn render(ctx: &RowContext, row: SettingRow, field: SettingField, keys: &'static [&'static str], current: &str, enabled: bool) -> AnyElement {
    if keys.len() > MAX_INLINE_CHOICES {
        let options = keys.iter().map(|key| (SettingValue::Choice(key), option_label(field, key))).collect();
        return menu(ctx, row, field, options, option_label(field, current), enabled);
    }
    let chips = keys.iter().map(|&key| {
        let on_edit = ctx.actions.on_edit.clone();
        let id = element_id_of(row, key);
        choice_chip(id, &option_label(field, key), key == current, ctx.theme, move |window, cx| {
            if enabled {
                on_edit(field, SettingValue::Choice(key), window, cx)
            }
        })
    });
    chip_row(ctx.theme).justify_end().children(chips).into_any_element()
}

/// The persona picker: "last used" or one of the human personas.
pub(super) fn persona(ctx: &RowContext, row: SettingRow, field: SettingField, current: &str) -> AnyElement {
    let last_used = t!("prefs.ui.persona_last_used").to_string();
    let options: MenuOptions = std::iter::once((SettingValue::Text(String::new()), last_used.clone()))
        .chain(ctx.data.personas.iter().map(|persona| (SettingValue::Text(persona.id.clone()), persona.name.clone())))
        .collect();
    let label = ctx.data.personas.iter().find(|persona| persona.id == current).map_or(last_used, |persona| persona.name.clone());
    menu(ctx, row, field, options, label, true)
}

/// A button showing `label` that opens a checked menu of `options`.
fn menu(ctx: &RowContext, row: SettingRow, field: SettingField, options: MenuOptions, label: String, enabled: bool) -> AnyElement {
    let (on_edit, current) = (ctx.actions.on_edit.clone(), field.get(&ctx.data.prefs));
    Button::new(element_id_of(row, MENU_PART))
        .outline()
        .small()
        .label(label)
        .dropdown_caret(true)
        .disabled(!enabled)
        .dropdown_menu_with_anchor(Anchor::TopRight, move |menu, _, _| {
            options.iter().fold(menu, |menu, (value, label)| menu.item(menu_item(field, value, label, &current, &on_edit)))
        })
        .into_any_element()
}

fn menu_item(field: SettingField, value: &SettingValue, label: &str, current: &SettingValue, on_edit: &EditAction) -> PopupMenuItem {
    let (on_edit, value) = (on_edit.clone(), value.clone());
    PopupMenuItem::new(label.to_string())
        .checked(value == *current)
        .on_click(move |_, window, cx| on_edit(field, value.clone(), window, cx))
}
