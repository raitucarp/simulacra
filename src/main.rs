//! Binary entry point for the Simulacra desktop application.

// Release builds on Windows are GUI apps: no console window opens behind the main window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui_kit::*;
use simulacra::theme::ThemeTokens;
use simulacra::ui::features::splash::{splash_window_options, SplashView};

rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    application().with_assets(simulacra::ui::assets::AppAssets).run(|cx| {
        use gpui_kit::component as gpui_component;
        rust_i18n::extend!(gpui_component);
        init(cx);
        simulacra::ui::keymap::install_keymap(cx);
        // A small splash window loads everything, builds the shell in a hidden main window,
        // then shows that window maximized and closes itself.
        let window_options = splash_window_options(&ThemeTokens::default(), cx);
        open_window(window_options, cx, |window, cx| cx.new(|cx| SplashView::new(window, cx)))
            .expect("Failed to open the Simulacra splash window");
    });
}
