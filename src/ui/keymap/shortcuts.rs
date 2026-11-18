//! Every keyboard shortcut of the app as one data table. The bindings ([`super::install_keymap`])
//! and the shortcut sheet are both generated from [`SHORTCUTS`].
//!
//! Keys use gpui's notation; `secondary-` is Ctrl on Windows and Linux and Cmd on macOS.

use crate::ui::features::command_palette::CommandId;

/// Where a shortcut is active (the key context it is bound in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyScope {
    /// Anywhere in the main window, including while typing.
    Shell,
    /// Inside a text field.
    Input,
    /// Inside the Agent Forge.
    Forge,
    /// Inside the command palette's search field.
    Palette,
    /// While the shortcut sheet is open.
    Sheet,
    /// While the image lightbox is open (sb C3).
    Lightbox,
    /// A review card waiting to be revealed or graded (sb C1).
    Review,
}

/// A key of the review session: reveal the answer, or grade (pick an option in a quiz).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewKey {
    Again,
    Hard,
    Good,
    Easy,
    Reveal,
}

/// A palette navigation key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteKey {
    Previous,
    Next,
    Confirm,
    Dismiss,
    NextGroup,
    PreviousGroup,
}

/// What a shortcut does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutTarget {
    /// Runs a registry command through the shell dispatcher.
    Command(CommandId),
    /// Publishes the composer draft.
    SubmitPost,
    ForgeUndo,
    ForgeRedo,
    ForgeDismiss,
    Palette(PaletteKey),
    CloseSheet,
    Review(ReviewKey),
    /// Moves through or closes the image lightbox (sb C3).
    Lightbox(LightboxKey),
}

/// A lightbox key (sb C3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightboxKey {
    Close,
    Previous,
    Next,
}

/// One binding: a key chord, what it does and where.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shortcut {
    pub keys: &'static str,
    pub target: ShortcutTarget,
    pub scope: KeyScope,
}

const fn shell(keys: &'static str, id: CommandId) -> Shortcut {
    Shortcut { keys, target: ShortcutTarget::Command(id), scope: KeyScope::Shell }
}

const fn palette(keys: &'static str, key: PaletteKey) -> Shortcut {
    Shortcut { keys, target: ShortcutTarget::Palette(key), scope: KeyScope::Palette }
}

const fn scoped(keys: &'static str, target: ShortcutTarget, scope: KeyScope) -> Shortcut {
    Shortcut { keys, target, scope }
}

use CommandId as C;
use PaletteKey as P;
use ReviewKey as R;
use ShortcutTarget as T;

/// The keymap. The first chord of a target is the one shown next to it in menus and the palette.
pub const SHORTCUTS: &[Shortcut] = &[
    shell("secondary-p", C::OpenPalette),
    shell("secondary-k", C::OpenPalette),
    shell("secondary-shift-p", C::OpenCommands),
    shell("secondary-shift-k", C::SemanticSearch),
    shell("secondary-/", C::ShowShortcuts),
    shell("secondary-b", C::ToggleSidebar),
    shell("secondary-alt-b", C::ToggleRightSidebar),
    shell("secondary-,", C::OpenSettings),
    shell("secondary-n", C::NewThread),
    shell("secondary-shift-space", C::NewNote),
    shell("secondary-o", C::ImportFile),
    shell("secondary-1", C::GoTimeline),
    shell("secondary-2", C::GoKnowledge),
    shell("secondary-3", C::GoAgents),
    shell("secondary-4", C::GoPersonas),
    shell("secondary-5", C::GoMindGraph),
    shell("secondary-6", C::GoReview),
    scoped("secondary-enter", T::SubmitPost, KeyScope::Input),
    scoped("secondary-z", T::ForgeUndo, KeyScope::Forge),
    scoped("secondary-shift-z", T::ForgeRedo, KeyScope::Forge),
    scoped("ctrl-y", T::ForgeRedo, KeyScope::Forge),
    scoped("escape", T::ForgeDismiss, KeyScope::Forge),
    palette("up", P::Previous),
    palette("down", P::Next),
    palette("enter", P::Confirm),
    palette("escape", P::Dismiss),
    palette("tab", P::NextGroup),
    palette("shift-tab", P::PreviousGroup),
    scoped("escape", T::CloseSheet, KeyScope::Sheet),
    scoped("escape", T::Lightbox(LightboxKey::Close), KeyScope::Lightbox),
    scoped("left", T::Lightbox(LightboxKey::Previous), KeyScope::Lightbox),
    scoped("right", T::Lightbox(LightboxKey::Next), KeyScope::Lightbox),
    scoped("space", T::Review(R::Reveal), KeyScope::Review),
    scoped("enter", T::Review(R::Reveal), KeyScope::Review),
    scoped("1", T::Review(R::Again), KeyScope::Review),
    scoped("2", T::Review(R::Hard), KeyScope::Review),
    scoped("3", T::Review(R::Good), KeyScope::Review),
    scoped("4", T::Review(R::Easy), KeyScope::Review),
];

/// The chord shown for `id` (its first binding), if it has one.
pub fn shortcut_for(id: CommandId) -> Option<&'static str> {
    SHORTCUTS.iter().find(|shortcut| shortcut.target == T::Command(id)).map(|shortcut| shortcut.keys)
}
