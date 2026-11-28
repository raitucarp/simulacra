//! Menu items that run a registry command: the command's title and icon, and its action, so
//! the row shows the command's shortcut and the shell runs it (menus set `action_context` to
//! the shell's focus).

use super::item::MenuItem;
use super::text::MenuText;
use crate::ui::features::command_palette::{spec, CommandId};
use crate::ui::keymap::RunCommand;

/// Id prefix of command items (`command.<wire key>`).
pub const COMMAND_PREFIX: &str = "command.";

/// The item running `id`.
pub fn command_item(id: CommandId) -> MenuItem {
    let command = spec(id);
    MenuItem::new(format!("{COMMAND_PREFIX}{}", id.wire_key()), MenuText::keyed(command.title_key(), command.title())).icon(command.icon).action(Box::new(RunCommand(id)))
}

/// The items of the ready commands among `ids`, in order (planned ones stay hidden).
pub fn command_items(ids: &[CommandId]) -> Vec<MenuItem> {
    ids.iter().copied().filter(|id| spec(*id).is_listed()).map(command_item).collect()
}
