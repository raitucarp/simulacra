//! The primary "New" button and its menu. Each entry is a registry command dispatched to the
//! shell root (so the menu shows each shortcut); planned commands stay hidden until a later
//! wave marks them ready.

use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::{spec, CommandId};
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::RunCommand;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::{DropdownMenu, PopupMenu};
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

const NEW_BUTTON_ID: &str = "sidebar-new-button";
const NEW_LABEL_KEY: &str = "sidebar.new";

/// The menu's commands, top to bottom.
const NEW_MENU_COMMANDS: [CommandId; 6] = [CommandId::NewThread, CommandId::NewNote, CommandId::NewVoiceNote, CommandId::NewDrawing, CommandId::ImportFile, CommandId::NewAgent];

pub(super) fn render_new_button(is_rail: bool, shell_focus: FocusHandle, theme: &ThemeTokens) -> impl IntoElement {
    let label = t!(NEW_LABEL_KEY).to_string();
    let button = Button::new(NEW_BUTTON_ID).primary().icon(Icon::new(LucideIcon::Plus));
    let button = if is_rail { button.tooltip(label) } else { button.label(label).dropdown_caret(true).w_full() };
    let menu_width = theme.new_menu_width;
    button.dropdown_menu(move |menu, _, _| new_menu(menu, shell_focus.clone()).min_w(menu_width))
}

/// The ready entries of [`NEW_MENU_COMMANDS`].
pub(super) fn new_menu_entries() -> impl Iterator<Item = CommandId> {
    NEW_MENU_COMMANDS.into_iter().filter(|id| spec(*id).is_listed())
}

fn new_menu(menu: PopupMenu, shell_focus: FocusHandle) -> PopupMenu {
    new_menu_entries().map(spec).fold(menu.action_context(shell_focus), |menu, command| {
        menu.menu_with_icon(command.title(), Icon::new(command.icon), Box::new(RunCommand(command.id)))
    })
}
