//! Boxed callback signatures accepted by the settings modal.

use super::tabs::SettingsTab;
use crate::agent::LlmProvider;
use gpui_kit::{App, Window};

pub type ModalCloseCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type SelectProviderCallback = Box<dyn Fn(LlmProvider, &mut Window, &mut App) + 'static>;
pub type SelectPresetModelCallback = Box<dyn Fn(&'static str, &mut Window, &mut App) + 'static>;
pub type SaveSettingsCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type SelectTabCallback = Box<dyn Fn(SettingsTab, &mut Window, &mut App) + 'static>;
pub type ToggleSchedulerCallback = Box<dyn Fn(bool, &mut Window, &mut App) + 'static>;
