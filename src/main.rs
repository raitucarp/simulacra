//! Binary entry point for the Simulacra desktop application.

use gpui_kit::*;
use simulacra::theme::ThemeTokens;
use simulacra::ui::features::splash::{splash_window_options, SplashView};

rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    application().with_assets(simulacra::ui::assets::AppAssets).run(|cx| {
        use gpui_kit::component as gpui_component;
        rust_i18n::extend!(gpui_component);
        init(cx);
        cx.bind_keys([
            KeyBinding::new("ctrl-enter", simulacra::ui::molecules::post_composer::SubmitPostDraft, Some("Input")),
            KeyBinding::new("cmd-enter", simulacra::ui::molecules::post_composer::SubmitPostDraft, Some("Input")),
        ]);
        // A small splash window loads everything, builds the shell in a hidden main window,
        // then shows that window maximized and closes itself.
        let window_options = splash_window_options(&ThemeTokens::default(), cx);
        open_window(window_options, cx, |window, cx| cx.new(|cx| SplashView::new(window, cx)))
            .expect("Failed to open the Simulacra splash window");
    });
}
