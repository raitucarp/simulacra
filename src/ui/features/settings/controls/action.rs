//! A command button; "Reset all settings" is destructive and asks for a second click.

use super::super::catalog::{SettingRow, SettingsCommand};
use super::super::row::{element_id_of, RowContext};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use super::super::text::tr;
use rust_i18n::t;

const BUTTON_PART: &str = "run";
/// Prefix of the button label keys (`prefs.command.<id>`).
const LABEL_PREFIX: &str = "prefs.command";

const fn icon(command: SettingsCommand) -> LucideIcon {
    match command {
        SettingsCommand::ReplayWelcome => LucideIcon::Sparkles,
        SettingsCommand::ReindexEmbeddings => LucideIcon::RefreshCw,
        SettingsCommand::CancelReindex => LucideIcon::X,
        SettingsCommand::OpenDatabaseFolder | SettingsCommand::OpenMediaFolder => LucideIcon::Folder,
        SettingsCommand::Backup => LucideIcon::Archive,
        SettingsCommand::Export => LucideIcon::Share2,
        SettingsCommand::ClearCaches => LucideIcon::Trash,
        SettingsCommand::OpenKeymap => LucideIcon::Keyboard,
        SettingsCommand::ResetAll => LucideIcon::Undo2,
    }
}

/// The translated label of `command`'s button.
pub(super) fn label(command: SettingsCommand) -> String {
    tr(&format!("{LABEL_PREFIX}.{}", command.id()))
}

/// The button running `command`.
pub(super) fn render(ctx: &RowContext, row: SettingRow, command: SettingsCommand) -> AnyElement {
    let on_command = ctx.actions.on_command.clone();
    let confirming = command.is_destructive() && ctx.data.confirm_reset;
    let text = if confirming { t!("prefs.ui.confirm_reset").to_string() } else { label(command) };
    let button = Button::new(element_id_of(row, BUTTON_PART))
        .small()
        .icon(Icon::new(icon(command)).size(ctx.theme.icon_size_sm))
        .label(text)
        .on_click(move |_, window, cx| on_command(command, window, cx));
    match (command.is_destructive(), confirming) {
        (true, true) => button.danger(),
        (true, false) => button.outline().danger(),
        _ => button.outline(),
    }
    .into_any_element()
}
