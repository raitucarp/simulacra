//! Tooltips of the sidebar's icon-only controls, with the command's shortcut when it has one.

use crate::ui::features::command_palette::CommandId;
use crate::ui::keymap::{RunCommand, SHELL_KEY_CONTEXT};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::*;

/// Adds a hover tooltip showing `label` (and the binding of `command`).
pub(super) fn with_tooltip<E: StatefulInteractiveElement>(element: E, label: String, command: Option<CommandId>) -> E {
    element.tooltip(move |window, cx| {
        let tooltip = Tooltip::new(label.clone());
        let tooltip = match command {
            Some(id) => tooltip.action(&RunCommand(id), Some(SHELL_KEY_CONTEXT)),
            None => tooltip,
        };
        tooltip.build(window, cx)
    })
}
