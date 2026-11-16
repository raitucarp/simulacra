//! Undo / redo actions and their key bindings inside the forge.

use gpui_kit::actions;
use gpui_kit::*;

actions!(forge, [Undo, Redo]);

/// Key context of the forge root (text inputs keep their own undo inside it).
pub const FORGE_KEY_CONTEXT: &str = "Forge";
const UNDO_KEYS: [&str; 2] = ["ctrl-z", "cmd-z"];
const REDO_KEYS: [&str; 3] = ["ctrl-shift-z", "ctrl-y", "cmd-shift-z"];

/// Registers the forge bindings; call once at startup.
pub fn init_key_bindings(cx: &mut App) {
    let undo = UNDO_KEYS.map(|keys| KeyBinding::new(keys, Undo, Some(FORGE_KEY_CONTEXT)));
    let redo = REDO_KEYS.map(|keys| KeyBinding::new(keys, Redo, Some(FORGE_KEY_CONTEXT)));
    cx.bind_keys(undo);
    cx.bind_keys(redo);
}
