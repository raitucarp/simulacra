//! Paste anywhere: with no text field focused, Ctrl+V (Cmd+V) captures the clipboard. The
//! other import shortcuts (quick capture, import a file) live in the central keymap.

use gpui_kit::{actions, App, KeyBinding};

actions!(simulacra_import, [PasteAnywhere]);

/// Paste chords. They have no key context, so a focused text input keeps its own paste.
pub const PASTE_KEYS: [&str; 2] = ["ctrl-v", "cmd-v"];

/// Registers the paste binding.
pub fn init_key_bindings(cx: &mut App) {
    cx.bind_keys(PASTE_KEYS.map(|keys| KeyBinding::new(keys, PasteAnywhere, None)));
}
