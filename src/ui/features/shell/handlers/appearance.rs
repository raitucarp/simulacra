//! Appearance-mode handlers: the title bar's Dark / Light / System toggle and following the
//! operating system's appearance live while the mode is System.

use super::super::view::AppView;
use crate::state::AppState;
use crate::theme::{AppearanceMode, PaletteMode, ThemeSettings};
use gpui_kit::*;

/// The operating system's appearance as a palette mode.
pub(in crate::ui::features::shell) fn system_mode(window: &Window) -> PaletteMode {
    PaletteMode::of_window(window.appearance())
}

/// Shows the palette of the effective mode at startup (persisting it when it changed).
pub(in crate::ui::features::shell) fn resolve_startup_appearance(state: &mut AppState, window: &Window) {
    let mut settings = state.theme_settings.clone();
    settings.apply_appearance(system_mode(window));
    if settings != state.theme_settings {
        state.update_theme_settings(settings);
    }
}

impl AppView {
    /// Title bar toggle: Dark, then Light, then System, then Dark again.
    pub(crate) fn cycle_appearance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut settings = self.state.theme_settings.clone();
        settings.set_appearance(settings.appearance.next(), system_mode(window));
        self.apply_theme_everywhere(settings, window, cx);
    }

    /// The operating system switched between light and dark: follow it in System mode.
    pub(crate) fn follow_system_appearance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.theme_settings.appearance != AppearanceMode::System {
            return;
        }
        let mut settings = self.state.theme_settings.clone();
        settings.apply_appearance(system_mode(window));
        self.apply_theme_everywhere(settings, window, cx);
    }

    /// Applies `settings` (when they changed) and shows them in the Appearance pane too.
    fn apply_theme_everywhere(&mut self, settings: ThemeSettings, window: &mut Window, cx: &mut Context<Self>) {
        if settings == self.state.theme_settings {
            return;
        }
        self.appearance.update(cx, |pane, cx| pane.set_settings(settings.clone(), window, cx));
        self.apply_theme_settings(settings, cx);
    }
}
