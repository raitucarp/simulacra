//! Undo / redo and Escape (close hint cards, then the forge) and their key bindings.

use gpui_kit::actions;
use gpui_kit::*;

actions!(forge, [Undo, Redo, Dismiss]);

/// Key context of the forge root (text inputs keep their own undo inside it).
pub const FORGE_KEY_CONTEXT: &str = "Forge";
const UNDO_KEYS: [&str; 2] = ["ctrl-z", "cmd-z"];
const REDO_KEYS: [&str; 3] = ["ctrl-shift-z", "ctrl-y", "cmd-shift-z"];
const DISMISS_KEY: &str = "escape";

/// Registers the forge bindings; call once at startup.
pub fn init_key_bindings(cx: &mut App) {
    let undo = UNDO_KEYS.map(|keys| KeyBinding::new(keys, Undo, Some(FORGE_KEY_CONTEXT)));
    let redo = REDO_KEYS.map(|keys| KeyBinding::new(keys, Redo, Some(FORGE_KEY_CONTEXT)));
    cx.bind_keys(undo);
    cx.bind_keys(redo);
    cx.bind_keys([KeyBinding::new(DISMISS_KEY, Dismiss, Some(FORGE_KEY_CONTEXT))]);
}
