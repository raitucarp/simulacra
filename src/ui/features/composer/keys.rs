//! Submit action, Enter-key identifiers and the Ctrl/Cmd+Enter chord shared by both composers.

use gpui_kit::actions;
use gpui_kit::*;

actions!(simulacra, [SubmitPostDraft]);

pub const KEY_ENTER: &str = "enter";
pub const KEY_ENTER_CR: &str = "\r";
pub const KEY_ENTER_LF: &str = "\n";

/// Keybinding that submits the draft on Windows/Linux.
pub(super) const SUBMIT_BINDING_CTRL: &str = "ctrl-enter";
/// Keybinding that submits the draft on macOS.
pub(super) const SUBMIT_BINDING_CMD: &str = "cmd-enter";
/// Key context in which the submit bindings are active.
pub(super) const INPUT_KEY_CONTEXT: &str = "Input";

/// True when the keystroke is Enter combined with Ctrl (or Cmd on macOS).
pub fn is_submit_chord(stroke: &Keystroke) -> bool {
    let is_enter =
        stroke.key == KEY_ENTER || stroke.key == KEY_ENTER_CR || stroke.key == KEY_ENTER_LF;
    let is_ctrl =
        stroke.modifiers.control || (cfg!(target_os = "macos") && stroke.modifiers.platform);
    is_enter && is_ctrl
}
