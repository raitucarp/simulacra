//! Binary entry point for the Simulacra desktop application.

use gpui_kit::component::TitleBar;
use gpui_kit::*;
use simulacra::theme::ThemeTokens;
use simulacra::ui::features::splash::AppRoot;

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
        // Opens maximized; un-maximizing restores a centered window of the default size.
        let tokens = ThemeTokens::default();
        let restore = size(tokens.window_default_width, tokens.window_default_height);
        let window_options = WindowOptions {
            titlebar: Some(TitleBar::title_bar_options()),
            app_owns_titlebar_drag: true,
            window_bounds: Some(WindowBounds::Maximized(Bounds::centered(None, restore, cx))),
            ..TitleBar::window_options()
        };

        // The root view paints the splash first, then loads the vault and builds the shell
        // over the next frames (it also keeps the kit theme in sync with the persisted one).
        open_window(window_options, cx, |window, cx| cx.new(|cx| AppRoot::new(window, cx)))
            .expect("Failed to open Simulacra main window");
    });
}
