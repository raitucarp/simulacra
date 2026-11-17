//! Callback signatures accepted by the settings window.

use super::catalog::{SettingRow, SettingsCommand};
use super::tabs::SettingsTab;
use crate::agent::LlmProvider;
use crate::domain::preferences::{SettingField, SettingValue};
use gpui_kit::{App, Window};
use std::rc::Rc;

pub type SelectProviderCallback = Box<dyn Fn(LlmProvider, &mut Window, &mut App) + 'static>;
pub type SelectPresetModelCallback = Box<dyn Fn(&'static str, &mut Window, &mut App) + 'static>;

/// A no-argument action (close, save).
pub type PlainAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
/// Sets one preference.
pub type EditAction = Rc<dyn Fn(SettingField, SettingValue, &mut Window, &mut App) + 'static>;
/// Acts on one row (reset to default).
pub type RowAction = Rc<dyn Fn(SettingRow, &mut Window, &mut App) + 'static>;
/// Runs a settings command.
pub type CommandAction = Rc<dyn Fn(SettingsCommand, &mut Window, &mut App) + 'static>;
/// Shows a category (and clears the search).
pub type TabAction = Rc<dyn Fn(SettingsTab, &mut Window, &mut App) + 'static>;

/// Everything the settings window can ask its owner to do.
#[derive(Clone)]
pub struct SettingsActions {
    pub on_edit: EditAction,
    pub on_reset: RowAction,
    pub on_command: CommandAction,
    pub on_select: TabAction,
    pub on_close: PlainAction,
    /// Persists the AI provider draft.
    pub on_save: PlainAction,
}
