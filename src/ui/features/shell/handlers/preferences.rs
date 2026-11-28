//! Preference edits from the settings window: every change is validated, saved at once and
//! pushed to the consumers that exist (language, scheduler switch and concurrency).

use super::super::view::AppView;
use crate::domain::preferences::{Preferences, SettingField, SettingValue, ToastKind};
use crate::theme::{Density, TypeScale};
use crate::ui::features::settings::{SettingRow, SettingsTab, ThemeScale};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;

impl AppView {
    /// Sets one preference.
    pub(crate) fn edit_preference(&mut self, field: SettingField, value: SettingValue, window: &mut Window, cx: &mut Context<Self>) {
        let mut next = field.set(self.state.preferences().clone(), &value);
        // A model still at the old embedding provider's default follows the provider.
        let previous = self.state.preferences().embeddings.clone();
        if next.embeddings.provider != previous.provider {
            next.embeddings = previous.with_provider(next.embeddings.provider);
        }
        self.apply_preferences(next, window, cx);
    }

    /// Puts one row back to its default.
    pub(crate) fn reset_setting_row(&mut self, row: SettingRow, window: &mut Window, cx: &mut Context<Self>) {
        match row {
            SettingRow::Field(field) => {
                let next = field.reset(self.state.preferences().clone());
                self.apply_preferences(next, window, cx);
                self.settings_ui.sync_inputs(self.state.preferences(), window, cx);
            }
            SettingRow::Theme(scale) => self.reset_theme_scale(scale, window, cx),
            SettingRow::Action(_) | SettingRow::Path(_) | SettingRow::Embedded(_) => {}
        }
    }

    /// Adopts `next` (validated and saved by the store) and updates its consumers.
    pub(crate) fn apply_preferences(&mut self, next: Preferences, window: &mut Window, cx: &mut Context<Self>) {
        let previous = self.state.preferences().clone();
        self.state.update_preferences(next);
        let current = self.state.preferences().clone();
        if current.general.language != previous.general.language {
            self.chrome.current_language = current.general.language;
            crate::i18n::set_locale(current.general.language);
        }
        crate::ui::format::set_clock(crate::ui::format::Clock::of(&current.general));
        self.scheduler.queue.set_max_in_flight(current.agents.max_concurrent as usize);
        if current.embeddings.provider != previous.embeddings.provider {
            self.settings_ui.sync_inputs(&current, window, cx);
        }
        if current.agents.autonomous_posting != previous.agents.autonomous_posting {
            self.on_scheduler_switched(current.agents.autonomous_posting, window, cx);
        }
        self.settings_ui.confirm_reset = false;
        cx.notify();
    }

    /// Restores a theme scale of the Appearance pane to its default.
    fn reset_theme_scale(&mut self, scale: ThemeScale, window: &mut Window, cx: &mut Context<Self>) {
        let mut settings = self.state.theme_settings.clone();
        match scale {
            ThemeScale::Density => settings.density = Density::default(),
            ThemeScale::FontSize => settings.type_scale = TypeScale::default(),
        }
        self.appearance.update(cx, |pane, cx| pane.set_settings(settings.clone(), window, cx));
        self.apply_theme_settings(settings, cx);
    }

    /// Shows `tab`, clearing the search.
    pub(crate) fn open_settings_section(&mut self, tab: SettingsTab, window: &mut Window, cx: &mut Context<Self>) {
        if !self.chrome.is_settings_open {
            self.revert_settings_draft(window, cx);
            self.chrome.is_settings_open = true;
            window.focus(&self.settings_ui.rail_focus, cx);
        }
        self.chrome.active_settings_tab = tab;
        self.settings_ui.confirm_reset = false;
        self.settings_ui.clear_search(window, cx);
        cx.notify();
    }

    /// Shows `note` unless the user silenced toasts of `kind`.
    pub(crate) fn toast(&self, kind: ToastKind, note: Notification, window: &mut Window, cx: &mut App) {
        if self.state.preferences().notifications.allows(kind) {
            window.push_notification(note, cx);
        }
    }
}
