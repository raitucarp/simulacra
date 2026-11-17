//! Dedicated frameless window title bar organism.
//! Features 100% Lucide SVG icons and decoupled drag/interactive zones.

mod appearance_toggle;
mod render;
mod sections;
mod window_controls;

use crate::domain::Network;
use crate::i18n::Language;
use crate::theme::{AppearanceMode, ThemeTokens};
use gpui_kit::{App, IntoElement, Window};

pub use appearance_toggle::{mode_label, mode_label_key};

pub type TitleBarAction = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Atomic Window Titlebar component.
#[derive(IntoElement)]
pub struct AppTitleBar {
    active_network: Option<Network>,
    current_language: Language,
    theme: ThemeTokens,
    on_toggle_left_sidebar: TitleBarAction,
    on_toggle_right_sidebar: TitleBarAction,
    on_toggle_lang: TitleBarAction,
    on_toggle_settings: TitleBarAction,
    /// Current appearance mode and the toggle cycling it.
    appearance: Option<(AppearanceMode, TitleBarAction)>,
}

impl AppTitleBar {
    /// Creates a new AppTitleBar with sidebar toggles and Lucide icon controls.
    pub fn new(
        active_network: Option<Network>,
        current_language: Language,
        theme: ThemeTokens,
        on_toggle_left_sidebar: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_right_sidebar: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_lang: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_settings: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            active_network,
            current_language,
            theme,
            on_toggle_left_sidebar: Box::new(on_toggle_left_sidebar),
            on_toggle_right_sidebar: Box::new(on_toggle_right_sidebar),
            on_toggle_lang: Box::new(on_toggle_lang),
            on_toggle_settings: Box::new(on_toggle_settings),
            appearance: None,
        }
    }

    /// Adds the Dark / Light / System toggle showing `mode`.
    pub fn appearance_toggle(mut self, mode: AppearanceMode, on_cycle: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.appearance = Some((mode, Box::new(on_cycle)));
        self
    }
}
