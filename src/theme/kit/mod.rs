//! Bridge from the app's [`ThemeTokens`] to gpui-component's global `Theme`, so kit widgets
//! (buttons, inputs, popovers, dialogs, menus, lists, scrollbars, charts) follow the app theme.

mod colors;

use crate::theme::ThemeTokens;
use gpui_kit::component::{Theme, ThemeConfig, ThemeConfigColors, ThemeMode};
use gpui_kit::App;
use std::rc::Rc;

/// Name of the generated kit theme.
pub const KIT_THEME_NAME: &str = "Simulacra";

/// Pure mapping of the tokens onto a gpui-component theme configuration.
pub fn kit_theme_config(tokens: &ThemeTokens) -> ThemeConfig {
    let mut colors = ThemeConfigColors::default();
    colors::apply_base(&mut colors, tokens);
    colors::apply_buttons(&mut colors, tokens);
    colors::apply_statuses(&mut colors, tokens);
    colors::apply_surfaces(&mut colors, tokens);
    colors::apply_charts(&mut colors, tokens);
    ThemeConfig {
        name: KIT_THEME_NAME.into(),
        mode: if tokens.mode.is_dark() { ThemeMode::Dark } else { ThemeMode::Light },
        font_family: Some(tokens.font_family_ui.clone().into()),
        mono_font_family: Some(tokens.font_family_mono.clone().into()),
        radius: Some(tokens.radius_sm.as_f32().round() as usize),
        radius_lg: Some(tokens.radius_md.as_f32().round() as usize),
        colors,
        ..ThemeConfig::default()
    }
}

/// Installs the tokens as gpui-component's active theme and refreshes every window.
pub fn sync_kit_theme(tokens: &ThemeTokens, cx: &mut App) {
    let config = Rc::new(kit_theme_config(tokens));
    Theme::update(cx, |theme| theme.apply_config(&config));
}

#[cfg(test)]
mod tests;
