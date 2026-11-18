//! Grouping of the keymap for the shortcut sheet: sections, row labels and merged chords.

use super::shortcuts::{LightboxKey, PaletteKey, ShortcutTarget, SHORTCUTS};
use crate::ui::features::command_palette::{spec, CommandId};
use rust_i18n::t;

/// A section of the shortcut sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutGroup {
    General,
    Navigation,
    Writing,
    Palette,
    Forge,
}

impl ShortcutGroup {
    /// Every section, in sheet order.
    pub const ALL: [ShortcutGroup; 5] = [Self::General, Self::Navigation, Self::Writing, Self::Palette, Self::Forge];

    /// Locale key of the section title.
    pub const fn title_key(self) -> &'static str {
        match self {
            Self::General => "keymap.group.general",
            Self::Navigation => "keymap.group.navigation",
            Self::Writing => "keymap.group.writing",
            Self::Palette => "keymap.group.palette",
            Self::Forge => "keymap.group.forge",
        }
    }
}

/// The sheet section of a target.
pub fn group_of(target: ShortcutTarget) -> ShortcutGroup {
    match target {
        ShortcutTarget::Command(id) => command_group(id),
        ShortcutTarget::SubmitPost => ShortcutGroup::Writing,
        ShortcutTarget::ForgeUndo | ShortcutTarget::ForgeRedo | ShortcutTarget::ForgeDismiss => ShortcutGroup::Forge,
        ShortcutTarget::Palette(_) => ShortcutGroup::Palette,
        ShortcutTarget::CloseSheet | ShortcutTarget::Lightbox(_) => ShortcutGroup::General,
    }
}

fn command_group(id: CommandId) -> ShortcutGroup {
    match id {
        CommandId::GoTimeline | CommandId::GoKnowledge | CommandId::GoAgents | CommandId::GoPersonas
        | CommandId::GoMindGraph | CommandId::OpenInbox | CommandId::ToggleSidebar | CommandId::ToggleRightSidebar => {
            ShortcutGroup::Navigation
        }
        CommandId::NewThread | CommandId::NewNote | CommandId::ImportFile | CommandId::NewAgent => ShortcutGroup::Writing,
        _ => ShortcutGroup::General,
    }
}

/// Locale-resolved description of a target (a command's registry title).
pub fn target_label(target: ShortcutTarget) -> String {
    match (target, target_label_key(target)) {
        (ShortcutTarget::Command(id), _) => spec(id).title(),
        (_, Some(key)) => t!(key).to_string(),
        (_, None) => String::new(),
    }
}

/// Locale key of a non-command target's description.
pub const fn target_label_key(target: ShortcutTarget) -> Option<&'static str> {
    Some(match target {
        ShortcutTarget::Command(_) => return None,
        ShortcutTarget::SubmitPost => "keymap.target.submit_post",
        ShortcutTarget::ForgeUndo => "keymap.target.forge_undo",
        ShortcutTarget::ForgeRedo => "keymap.target.forge_redo",
        ShortcutTarget::ForgeDismiss => "keymap.target.forge_dismiss",
        ShortcutTarget::CloseSheet => "keymap.target.close_sheet",
        ShortcutTarget::Palette(key) => palette_label_key(key),
        ShortcutTarget::Lightbox(LightboxKey::Close) => "keymap.target.lightbox_close",
        ShortcutTarget::Lightbox(LightboxKey::Previous) => "keymap.target.lightbox_previous",
        ShortcutTarget::Lightbox(LightboxKey::Next) => "keymap.target.lightbox_next",
    })
}

const fn palette_label_key(key: PaletteKey) -> &'static str {
    match key {
        PaletteKey::Previous => "keymap.target.palette_previous",
        PaletteKey::Next => "keymap.target.palette_next",
        PaletteKey::Confirm => "keymap.target.palette_confirm",
        PaletteKey::Dismiss => "keymap.target.palette_dismiss",
        PaletteKey::NextGroup => "keymap.target.palette_next_group",
        PaletteKey::PreviousGroup => "keymap.target.palette_previous_group",
    }
}

/// One sheet row: a target and every chord bound to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetRow {
    pub target: ShortcutTarget,
    pub chords: Vec<&'static str>,
}

/// The rows of `group`, one per target (aliases merged), in keymap order.
pub fn sheet_rows(group: ShortcutGroup) -> Vec<SheetRow> {
    SHORTCUTS.iter().filter(|shortcut| group_of(shortcut.target) == group).fold(Vec::new(), |mut rows: Vec<SheetRow>, shortcut| {
        match rows.iter_mut().find(|row| row.target == shortcut.target) {
            Some(row) => row.chords.push(shortcut.keys),
            None => rows.push(SheetRow { target: shortcut.target, chords: vec![shortcut.keys] }),
        }
        rows
    })
}
