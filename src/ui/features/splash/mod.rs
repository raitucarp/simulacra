//! Splash screen and the window's root view.
//!
//! [`AppRoot`] paints the splash on the very first frame (themed from the persisted theme,
//! read straight from the opened database), then loads the store in visible stages, one per
//! frame, builds the [`AppView`] shell, keeps the splash up for at least
//! `splash_min_duration_ms` and fades it out over the shell.
//!
//! - `stage`: pure stage sequencing and timing.
//! - `boot`: the staged loading itself.
//! - `render`: the splash visuals (brand mark, Mnemo, progress line, version).

mod boot;
mod render;
mod stage;

#[cfg(test)]
mod tests;

pub use stage::SplashStage;

use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::shell::AppView;
use boot::Boot;
use gpui_kit::*;
use std::time::Instant;

/// Whether the splash is still covering the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SplashPhase {
    /// Loading, or loaded and waiting out the minimum time.
    Showing,
    /// Fading out over the shell.
    Fading,
    /// Gone; only the shell is rendered.
    Done,
}

/// Root view of the main window: the splash while loading, then the shell.
pub struct AppRoot {
    theme: ThemeTokens,
    boot: Boot,
    app: Option<Entity<AppView>>,
    phase: SplashPhase,
    shown_at: Instant,
}

impl AppRoot {
    /// Opens the database and themes the splash (and the kit widgets) from the persisted
    /// theme; the rest loads over the next frames.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let database = AppState::open_default_database();
        let settings = database.as_ref().map(|db| db.load_theme_settings()).unwrap_or_default();
        let theme = settings.tokens();
        let _ = crate::ui::illustration::set_active_palette(&settings.effective_palette().colors);
        crate::theme::sync_kit_theme(&theme, cx);
        cx.on_next_frame(window, Self::advance);
        Self { theme, boot: Boot::Opened(database), app: None, phase: SplashPhase::Showing, shown_at: Instant::now() }
    }

    /// The shell, once it is built.
    pub fn app(&self) -> Option<&Entity<AppView>> {
        self.app.as_ref()
    }

    /// The stage the splash currently shows.
    pub fn stage(&self) -> SplashStage {
        self.boot.stage()
    }
}

impl Render for AppRoot {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let splash = (self.phase != SplashPhase::Done).then(|| self.render_splash(window));
        div().size_full().relative().bg(self.theme.bg_app).children(self.app.clone()).children(splash)
    }
}
