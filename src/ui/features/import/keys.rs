//! Paste anywhere: with no text field focused, Ctrl+V (Cmd+V) captures the clipboard. The
//! other import shortcuts (quick capture, import a file) live in the central keymap.

use crate::ui::keymap::SHELL_KEY_CONTEXT;
use gpui_kit::{actions, App, KeyBinding};

actions!(simulacra_import, [PasteAnywhere]);

/// Paste chords.
pub const PASTE_KEYS: [&str; 2] = ["ctrl-v", "cmd-v"];

/// The paste bindings, scoped to the shell like every other app-wide shortcut. gpui ranks a
/// binding without a key context as if it sat on the focused element, so an unscoped binding
/// would beat a focused text input's own paste and turn the clipboard into a post. Scoped to
/// the shell, it ranks below any input inside it and fires only when no input takes the keys.
pub fn paste_bindings() -> [KeyBinding; 2] {
    PASTE_KEYS.map(|keys| KeyBinding::new(keys, PasteAnywhere, Some(SHELL_KEY_CONTEXT)))
}

/// Registers the paste binding.
pub fn init_key_bindings(cx: &mut App) {
    cx.bind_keys(paste_bindings());
}
