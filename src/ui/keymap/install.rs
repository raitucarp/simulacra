//! Turns [`SHORTCUTS`] into gpui key bindings. Call [`install_keymap`] once at startup, after
//! the kit's own `init` so these bindings take precedence at equal depth.

use super::contexts::context_of;
use super::shortcuts::{PaletteKey, ReviewKey, Shortcut, ShortcutTarget, SHORTCUTS};
use crate::ui::features::command_palette::{
    CommandId, ConfirmSelection, DismissPalette, NextGroup, PreviousGroup, SelectNext, SelectPrevious,
};
use crate::ui::features::composer::SubmitPostDraft;
use crate::ui::features::forge::view::{Dismiss, Redo, Undo};
use crate::ui::features::review::keys::{GradeAgain, GradeEasy, GradeGood, GradeHard, RevealAnswer};
use gpui_kit::*;
use std::rc::Rc;

/// Runs a registry command; the shell's root handles it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Action)]
#[action(namespace = simulacra, no_json)]
pub struct RunCommand(pub CommandId);

/// Registers every shortcut of the keymap (`secondary-` resolves for the host OS).
pub fn install_keymap(cx: &mut App) {
    let bindings: Vec<KeyBinding> = SHORTCUTS.iter().filter_map(|shortcut| key_binding(shortcut, cx)).collect();
    cx.bind_keys(bindings);
}

fn key_binding(shortcut: &Shortcut, cx: &App) -> Option<KeyBinding> {
    let context = KeyBindingContextPredicate::parse(context_of(shortcut.scope)).ok().map(Rc::new);
    let binding = KeyBinding::load(shortcut.keys, action_of(shortcut.target), context, false, None, cx);
    binding.map_err(|err| eprintln!("simulacra: shortcut {} not bound: {err}", shortcut.keys)).ok()
}

/// The gpui action a target dispatches.
pub fn action_of(target: ShortcutTarget) -> Box<dyn Action> {
    match target {
        ShortcutTarget::Command(id) => Box::new(RunCommand(id)),
        ShortcutTarget::SubmitPost => Box::new(SubmitPostDraft),
        ShortcutTarget::ForgeUndo => Box::new(Undo),
        ShortcutTarget::ForgeRedo => Box::new(Redo),
        ShortcutTarget::ForgeDismiss => Box::new(Dismiss),
        ShortcutTarget::CloseSheet => Box::new(RunCommand(CommandId::ShowShortcuts)),
        ShortcutTarget::Palette(key) => palette_action(key),
        ShortcutTarget::Review(key) => review_action(key),
    }
}

fn review_action(key: ReviewKey) -> Box<dyn Action> {
    match key {
        ReviewKey::Again => Box::new(GradeAgain),
        ReviewKey::Hard => Box::new(GradeHard),
        ReviewKey::Good => Box::new(GradeGood),
        ReviewKey::Easy => Box::new(GradeEasy),
        ReviewKey::Reveal => Box::new(RevealAnswer),
    }
}

fn palette_action(key: PaletteKey) -> Box<dyn Action> {
    match key {
        PaletteKey::Previous => Box::new(SelectPrevious),
        PaletteKey::Next => Box::new(SelectNext),
        PaletteKey::Confirm => Box::new(ConfirmSelection),
        PaletteKey::Dismiss => Box::new(DismissPalette),
        PaletteKey::NextGroup => Box::new(NextGroup),
        PaletteKey::PreviousGroup => Box::new(PreviousGroup),
    }
}
