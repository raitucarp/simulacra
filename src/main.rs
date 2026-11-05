//! Binary entry point for the Simulacra desktop application.

use gpui_kit::component::{Theme, ThemeMode, TitleBar};
use gpui_kit::*;
use simulacra::prelude::*;

rust_i18n::i18n!("locales", fallback = "en");

fn main() {
    application().with_assets(assets::AllAssets).run(|cx| {
        use gpui_kit::component as gpui_component;
        rust_i18n::extend!(gpui_component);
        init(cx);
        cx.bind_keys([
            KeyBinding::new("ctrl-enter", simulacra::ui::molecules::post_composer::SubmitPostDraft, Some("Input")),
            KeyBinding::new("cmd-enter", simulacra::ui::molecules::post_composer::SubmitPostDraft, Some("Input")),
        ]);
        Theme::change(ThemeMode::Dark, None, cx);
        Theme::update(cx, |theme| {
            theme.font_family = "Segoe UI".into();
        });

        let window_options = WindowOptions {
            titlebar: Some(TitleBar::title_bar_options()),
            app_owns_titlebar_drag: true,
            ..TitleBar::window_options()
        };

        open_window(window_options, cx, |window, cx| {
            cx.new(|cx| AppView::new(window, cx))
        })
        .expect("Failed to open Simulacra main window");
    });
}
