//! The primary "New" button and its menu (M1): Write, Capture, Import and Create sections,
//! and every other creation command under Create › More. Each entry is a registry command
//! dispatched to the shell root, so the menu shows each shortcut; planned commands stay
//! hidden until a later wave marks them ready.

use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::CommandId;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{command_items, render_menu, MenuSection, MenuSpec, MenuText, Submenu};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

const NEW_BUTTON_ID: &str = "sidebar-new-button";
const NEW_LABEL_KEY: &str = "sidebar.new";

use CommandId as C;

/// Each section's commands, top to bottom.
const WRITE: [CommandId; 1] = [C::NewThread];
const CAPTURE: [CommandId; 3] = [C::NewNote, C::NewVoiceNote, C::NewDrawing];
const IMPORT: [CommandId; 1] = [C::ImportFile];
const CREATE: [CommandId; 1] = [C::NewAgent];
/// Creation commands that were only in the palette, gathered under Create › More.
const MORE: [CommandId; 7] = [C::NewJournalEntry, C::LogMood, C::NewTimeCapsule, C::NewDebate, C::NewThoughtExperiment, C::NewConnectionMap, C::CreateNetwork];

pub(super) fn render_new_button(is_rail: bool, shell_focus: FocusHandle, theme: &ThemeTokens) -> impl IntoElement {
    let label = t!(NEW_LABEL_KEY).to_string();
    let button = Button::new(NEW_BUTTON_ID).primary().icon(Icon::new(LucideIcon::Plus));
    let button = if is_rail { button.tooltip(label) } else { button.label(label).dropdown_caret(true).w_full() };
    let menu_width = theme.new_menu_width;
    button.dropdown_menu(move |menu, window, cx| render_menu(menu, &new_menu_spec().action_context(shell_focus.clone()).min_width(menu_width), window, cx))
}

/// The New menu (its actions go to the shell once the button sets its action context).
pub fn new_menu_spec() -> MenuSpec {
    let more = Submenu::new("new.more", MenuText::key("menus.new.more"), MenuSpec::of(command_items(&MORE))).icon(LucideIcon::Plus);
    MenuSpec::new()
        .section(MenuSection::labelled("menus.new.write").entries(command_items(&WRITE)))
        .section(MenuSection::labelled("menus.new.capture").entries(command_items(&CAPTURE)))
        .section(MenuSection::labelled("menus.new.import").entries(command_items(&IMPORT)))
        .section(MenuSection::labelled("menus.new.create").entries(command_items(&CREATE)).entry(more))
}
