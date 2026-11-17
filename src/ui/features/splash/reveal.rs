//! [`AppRoot`]: the main window's root view. It builds the shell around the loaded store while
//! the window is still hidden, and once the window is shown fades a cover in the app
//! background color off the shell, continuing the splash window's fade.

use super::stage::millis;
use crate::state::AppState;
use crate::ui::features::shell::AppView;
use gpui_kit::*;

/// Element id of the cover (also keys its fade animation).
const COVER_ID: &str = "app-reveal-cover";

/// Whether the cover still hides the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cover {
    /// Window not painted yet.
    Up,
    /// Fading off the shell.
    Fading,
    /// Gone; only the shell is rendered.
    Gone,
}

/// Root view of the main window: the shell, under a cover that fades once it is shown.
pub struct AppRoot {
    app: Entity<AppView>,
    cover: Cover,
}

impl AppRoot {
    /// Builds the shell and hands the kit its theme; the cover starts fading on the first
    /// frame the window actually paints.
    pub fn new(state: AppState, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let app = cx.new(|cx| AppView::with_state(state, window, cx));
        let theme = app.read(cx).theme.clone();
        crate::theme::sync_kit_theme(&theme, cx);
        cx.on_next_frame(window, |this, window, cx| this.reveal(window, cx));
        Self { app, cover: Cover::Up }
    }

    /// The shell.
    pub fn app(&self) -> &Entity<AppView> {
        &self.app
    }

    fn reveal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_cover(Cover::Fading, cx);
        let fade = millis(self.app.read(cx).theme.splash_fade_duration_ms);
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(fade).await;
            let _ = this.update(cx, |this, cx| this.set_cover(Cover::Gone, cx));
        })
        .detach();
    }

    fn set_cover(&mut self, cover: Cover, cx: &mut Context<Self>) {
        self.cover = cover;
        cx.notify();
    }

    fn render_cover(&self, cx: &App) -> Option<AnyElement> {
        let theme = &self.app.read(cx).theme;
        let cover = div().id(COVER_ID).absolute().inset_0().occlude().bg(theme.bg_app);
        match self.cover {
            Cover::Up => Some(cover.into_any_element()),
            Cover::Fading => {
                let fade = Animation::new(millis(theme.splash_fade_duration_ms));
                Some(cover.with_animation(COVER_ID, fade, |cover, delta| cover.opacity(1.0 - delta)).into_any_element())
            }
            Cover::Gone => None,
        }
    }
}

impl Render for AppRoot {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let background = self.app.read(cx).theme.bg_app;
        div().size_full().relative().bg(background).child(self.app.clone()).children(self.render_cover(cx))
    }
}
