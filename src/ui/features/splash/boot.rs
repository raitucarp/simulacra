//! Staged startup: each step runs right after the frame showing its stage label, so the
//! splash visibly advances while the store loads, the hidden main window is built and the
//! avatars rasterize. Then the splash fades, the main window is shown maximized and the
//! splash window closes.

use super::reveal::AppRoot;
use super::stage::{fade_delay, millis, warm_up_done, SplashStage};
use super::warm::{avatar_images, still_loading};
use super::window::main_window_options;
use super::{SplashPhase, SplashView};
use crate::error::Result;
use crate::state::AppState;
use crate::storage::{Database, Vault};
use gpui_kit::gpui::Image;
use gpui_kit::*;
use std::sync::Arc;
use std::time::Instant;

/// What has been loaded so far.
pub(super) enum Boot {
    /// The database is open (or failed to open); the vault worker starts next.
    Opened(Result<Database>),
    /// The vault worker is running (or failed); the store loads next.
    Started(Result<Vault>),
    /// The store is loaded; the main window is built next.
    Loaded(Box<AppState>),
    /// The main window is built (hidden); these avatars are still rasterizing.
    Warming(AnyWindowHandle, Vec<Arc<Image>>, Instant),
    /// Everything is ready; the main window waits to be shown.
    Built(AnyWindowHandle),
    /// The main window could not be opened.
    Failed,
}

impl Boot {
    /// The splash stage describing the next step.
    pub(super) fn stage(&self) -> SplashStage {
        match self {
            Self::Opened(_) => SplashStage::OpeningVault,
            Self::Started(_) => SplashStage::LoadingNetworks,
            Self::Loaded(_) => SplashStage::WakingAgents,
            Self::Warming(..) => SplashStage::WarmingUp,
            Self::Built(_) | Self::Failed => SplashStage::Ready,
        }
    }
}

impl SplashView {
    /// Runs the step of the current stage, shows the next stage and schedules its step.
    pub(super) fn advance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.boot = match std::mem::replace(&mut self.boot, Boot::Failed) {
            Boot::Opened(database) => Boot::Started(database.and_then(Vault::spawn)),
            Boot::Started(vault) => Boot::Loaded(Box::new(AppState::from_vault(vault))),
            Boot::Loaded(state) => self.open_main_window(*state, cx),
            Boot::Warming(main, images, started) => self.warm(main, images, started, window, cx),
            done => done,
        };
        cx.notify();
        match &self.boot {
            Boot::Built(main) => self.finish(*main, window, cx),
            Boot::Failed => window.remove_window(),
            _ => cx.on_next_frame(window, Self::advance),
        }
    }

    /// Opens the main window hidden and builds the shell in it.
    fn open_main_window(&mut self, state: AppState, cx: &mut Context<Self>) -> Boot {
        let images = avatar_images(&state, &self.theme);
        let options = main_window_options(&self.theme, cx);
        match open_window(options, cx, |window, cx| cx.new(|cx| AppRoot::new(state, window, cx))) {
            Ok((main, _)) => Boot::Warming(main, images, Instant::now()),
            Err(err) => {
                eprintln!("simulacra: failed to open the main window: {err}");
                Boot::Failed
            }
        }
    }

    /// One warm-up poll: done once every avatar is rasterized or the time limit passed.
    fn warm(&mut self, main: AnyWindowHandle, images: Vec<Arc<Image>>, started: Instant, window: &mut Window, cx: &mut Context<Self>) -> Boot {
        let pending = still_loading(images, window, cx);
        if warm_up_done(pending.len(), started.elapsed(), millis(self.theme.splash_warm_up_limit_ms)) {
            Boot::Built(main)
        } else {
            Boot::Warming(main, pending, started)
        }
    }

    /// Keeps the splash up for its minimum time, fades it out, then shows the main window
    /// (maximized) and closes the splash window.
    fn finish(&mut self, main: AnyWindowHandle, window: &mut Window, cx: &mut Context<Self>) {
        let wait = fade_delay(self.shown_at.elapsed(), millis(self.theme.splash_min_duration_ms));
        let fade = millis(self.theme.splash_fade_duration_ms);
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(wait).await;
            let _ = this.update(cx, |this, cx| this.set_phase(SplashPhase::Fading, cx));
            cx.background_executor().timer(fade).await;
            let _ = main.update(cx, |_, window, _| window.activate_window());
            let _ = this.update_in(cx, |_, window, _| window.remove_window());
        })
        .detach();
    }

    fn set_phase(&mut self, phase: SplashPhase, cx: &mut Context<Self>) {
        self.phase = phase;
        cx.notify();
    }
}
