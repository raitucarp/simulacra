//! Options of the two startup windows: the small, chrome-less splash window and the main
//! window, which opens hidden and maximized and is shown once everything has loaded.

use crate::theme::ThemeTokens;
use gpui_kit::component::TitleBar;
use gpui_kit::*;

/// A small, centred, fixed-size window without title bar chrome (the splash paints its own
/// drag area).
pub fn splash_window_options(theme: &ThemeTokens, cx: &App) -> WindowOptions {
    let splash = size(theme.splash_window_width, theme.splash_window_height);
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(None, splash, cx))),
        is_resizable: false,
        is_minimizable: false,
        ..TitleBar::window_options()
    }
}

/// The main window: maximized, restoring to a centred window of the default size, and hidden
/// until [`Window::activate_window`] shows it.
pub fn main_window_options(theme: &ThemeTokens, cx: &App) -> WindowOptions {
    let restore = size(theme.window_default_width, theme.window_default_height);
    WindowOptions {
        window_bounds: Some(WindowBounds::Maximized(Bounds::centered(None, restore, cx))),
        window_min_size: Some(size(theme.window_min_width, theme.window_min_height)),
        show: false,
        ..TitleBar::window_options()
    }
}
