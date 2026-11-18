//! Key contexts the keymap binds into, one per [`KeyScope`].

use super::shortcuts::KeyScope;
use crate::ui::features::forge::view::FORGE_KEY_CONTEXT;
use crate::ui::features::review::REVIEW_KEY_CONTEXT;

/// Key context of the main window's root element.
pub const SHELL_KEY_CONTEXT: &str = "Shell";
/// Key context of the command palette overlay.
pub const PALETTE_KEY_CONTEXT: &str = "CommandPalette";
/// Key context of the shortcut sheet overlay.
pub const SHEET_KEY_CONTEXT: &str = "ShortcutSheet";
/// Key context of the image lightbox (sb C3).
pub const LIGHTBOX_KEY_CONTEXT: &str = "Lightbox";
/// The kit's text field context.
const INPUT_KEY_CONTEXT: &str = "Input";
/// The palette's search field: deeper than the kit's own `Input` bindings (arrows, Enter,
/// Tab, Escape), so the palette's win while typing.
const PALETTE_INPUT_CONTEXT: &str = "CommandPalette > Input";

/// The key context predicate of `scope`.
pub const fn context_of(scope: KeyScope) -> &'static str {
    match scope {
        KeyScope::Shell => SHELL_KEY_CONTEXT,
        KeyScope::Input => INPUT_KEY_CONTEXT,
        KeyScope::Forge => FORGE_KEY_CONTEXT,
        KeyScope::Palette => PALETTE_INPUT_CONTEXT,
        KeyScope::Sheet => SHEET_KEY_CONTEXT,
        KeyScope::Lightbox => LIGHTBOX_KEY_CONTEXT,
        KeyScope::Review => REVIEW_KEY_CONTEXT,
    }
}
