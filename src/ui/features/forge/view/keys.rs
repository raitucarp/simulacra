//! Undo / redo and Escape (close hint cards, then the forge). Their key bindings are rows of
//! the central keymap (`crate::ui::keymap`).

use gpui_kit::actions;

actions!(forge, [Undo, Redo, Dismiss]);

/// Key context of the forge root (text inputs keep their own undo inside it).
pub const FORGE_KEY_CONTEXT: &str = "Forge";
