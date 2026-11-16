//! Binary entry point for the Simulacra desktop application.

use gpui_kit::component::TitleBar;
use gpui_kit::*;
use simulacra::prelude::*;

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
        let window_options = WindowOptions {
            titlebar: Some(TitleBar::title_bar_options()),
            app_owns_titlebar_drag: true,
            ..TitleBar::window_options()
        };

        open_window(window_options, cx, |window, cx| {
            let view = cx.new(|cx| AppView::new(window, cx));
            // Kit widgets (buttons, inputs, popovers, menus) follow the persisted app theme.
            let tokens = view.read(cx).theme.clone();
            simulacra::theme::sync_kit_theme(&tokens, cx);
            view
        })
        .expect("Failed to open Simulacra main window");
    });
}
