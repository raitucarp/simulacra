//! The central keymap: every shortcut as data ([`SHORTCUTS`]), the bindings generated from it
//! ([`install_keymap`]), key-cap display and the "Keyboard shortcuts" sheet.
//!
//! - `shortcuts`: the table, its scopes and targets.
//! - `contexts`: the gpui key context of each scope.
//! - `install`: table to `KeyBinding`s, and the [`RunCommand`] action the shell dispatches.
//! - `display` / `caps`: chord to localized key caps.
//! - `groups` / `sheet`: the shortcut sheet.

mod caps;
mod contexts;
mod display;
mod groups;
mod install;
mod sheet;
mod shortcuts;

#[cfg(test)]
mod tests;

pub use caps::{chord_text, render_chord, render_key_cap};
pub use contexts::{context_of, PALETTE_KEY_CONTEXT, SHEET_KEY_CONTEXT, SHELL_KEY_CONTEXT};
pub use display::{chord_labels, is_mac, key_caps, KeyCap};
pub use groups::{group_of, sheet_rows, target_label, target_label_key, SheetRow, ShortcutGroup};
pub use install::{install_keymap, RunCommand};
pub use sheet::{SheetAction, ShortcutSheet};
pub use shortcuts::{shortcut_for, KeyScope, PaletteKey, Shortcut, ShortcutTarget, SHORTCUTS};
