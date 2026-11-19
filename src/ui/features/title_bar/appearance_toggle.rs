//! The title bar's appearance toggle: a sun, moon or sun-and-moon icon cycling Dark, Light
//! and System.

use super::TitleBarAction;
use crate::theme::AppearanceMode;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use rust_i18n::t;

const APPEARANCE_TOGGLE_ID: &str = "appearance-mode-toggle-btn";

/// Icon of a mode: moon for Dark, sun for Light, sun-and-moon for System.
pub(super) const fn mode_icon(mode: AppearanceMode) -> LucideIcon {
    match mode {
        AppearanceMode::Dark => LucideIcon::Moon,
        AppearanceMode::Light => LucideIcon::Sun,
        AppearanceMode::System => LucideIcon::SunMoon,
    }
}

/// Translation key of a mode's label (shared with the Appearance settings pane).
pub fn mode_label_key(mode: AppearanceMode) -> String {
    format!("appearance.mode_{}", mode.id())
}

/// The translated label of a mode.
pub fn mode_label(mode: AppearanceMode) -> String {
    let key = mode_label_key(mode);
    t!(key.as_str()).to_string()
}

/// Ghost icon button showing `mode`; clicking switches to the next mode.
pub(super) fn render_appearance_toggle(mode: AppearanceMode, on_cycle: TitleBarAction) -> Button {
    let (current, next) = (mode_label(mode), mode_label(mode.next()));
    Button::new(APPEARANCE_TOGGLE_ID)
        .ghost()
        .compact()
        .icon(Icon::new(mode_icon(mode)))
        .tooltip(t!("appearance.toggle_tooltip", current = current, next = next).to_string())
        .on_click(move |_, window, cx| on_cycle(window, cx))
}
