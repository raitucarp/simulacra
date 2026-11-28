//! Startup: a small splash window, then the maximized main window.
//!
//! [`SplashView`] is the root of a small, centred, chrome-less window. It paints the splash on
//! its very first frame (themed from the persisted theme and the system appearance, read
//! straight from the opened database), then loads the store in visible stages, one per frame,
//! builds the shell in a hidden, maximized main window ([`AppRoot`]), warms up the avatars,
//! keeps the splash up for at least `splash_min_duration_ms`, fades it out, shows the main
//! window and closes itself.
//!
//! - `stage`: pure stage sequencing and timing.
//! - `boot`: the staged loading and the hand-over to the main window.
//! - `window`: the options of both windows.
//! - `warm`: avatar warm-up.
//! - `reveal`: the main window's root, fading in over the shell.
//! - `render`: the splash visuals (brand mark, Mnemo, progress line, version).

mod boot;
mod render;
mod reveal;
mod stage;
mod warm;
mod window;

#[cfg(test)]
mod tests;

pub use reveal::AppRoot;
pub use stage::SplashStage;
pub use window::{main_window_options, splash_window_options};

use crate::state::AppState;
use crate::theme::{PaletteMode, ThemeSettings, ThemeTokens};
use boot::Boot;
use gpui_kit::*;
use std::time::Instant;

/// Whether the splash is still fully visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SplashPhase {
    /// Loading, or loaded and waiting out the minimum time.
    Showing,
    /// Fading out before the main window is shown.
    Fading,
}

/// Root view of the splash window.
pub struct SplashView {
    theme: ThemeTokens,
    boot: Boot,
    phase: SplashPhase,
    shown_at: Instant,
}

impl SplashView {
    /// Opens the database and themes the splash (and the kit widgets) from the persisted
    /// theme in the system's appearance; the rest loads over the next frames.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let database = AppState::open_default_database();
        let mut settings: ThemeSettings = database.as_ref().map(|db| db.load_theme_settings()).unwrap_or_default();
        settings.apply_appearance(PaletteMode::of_window(window.appearance()));
        let theme = settings.tokens();
        let _ = crate::ui::illustration::set_active_palette(&settings.effective_palette().colors);
        if let Ok(db) = database.as_ref() {
            let _ = crate::ui::illustration::configure_packs(&crate::packs::packs_dir_for(&db.path()), &db.load_preferences().appearance.illustration_pack); // WS5
        }
        crate::theme::sync_kit_theme(&theme, cx);
        cx.on_next_frame(window, Self::advance);
        Self { theme, boot: Boot::Opened(database), phase: SplashPhase::Showing, shown_at: Instant::now() }
    }

    /// The stage the splash currently shows.
    pub fn stage(&self) -> SplashStage {
        self.boot.stage()
    }
}

impl Render for SplashView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().relative().bg(self.theme.bg_app).child(self.render_splash())
    }
}
