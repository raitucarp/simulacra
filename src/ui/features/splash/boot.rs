//! Staged startup: each step runs right after the frame showing its stage label, so the
//! splash visibly advances while the store loads and the shell is built.

use super::stage::{fade_delay, millis, SplashStage};
use super::{AppRoot, SplashPhase};
use crate::error::Result;
use crate::state::AppState;
use crate::storage::{Database, Vault};
use crate::ui::features::shell::AppView;
use gpui_kit::*;

/// What has been loaded so far.
pub(super) enum Boot {
    /// The database is open (or failed to open); the vault worker starts next.
    Opened(Result<Database>),
    /// The vault worker is running (or failed); the store loads next.
    Started(Result<Vault>),
    /// The store is loaded; the shell is built next.
    Loaded(Box<AppState>),
    /// The shell is built.
    Built,
}

impl Boot {
    /// The splash stage describing the next step.
    pub(super) fn stage(&self) -> SplashStage {
        match self {
            Self::Opened(_) => SplashStage::OpeningVault,
            Self::Started(_) => SplashStage::LoadingNetworks,
            Self::Loaded(_) => SplashStage::WakingAgents,
            Self::Built => SplashStage::Ready,
        }
    }
}

impl AppRoot {
    /// Runs the step of the current stage, shows the next stage and schedules its step.
    pub(super) fn advance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.boot = match std::mem::replace(&mut self.boot, Boot::Built) {
            Boot::Opened(database) => Boot::Started(database.and_then(Vault::spawn)),
            Boot::Started(vault) => Boot::Loaded(Box::new(AppState::from_vault(vault))),
            Boot::Loaded(state) => {
                self.build_shell(*state, window, cx);
                Boot::Built
            }
            Boot::Built => return,
        };
        cx.notify();
        match self.boot {
            Boot::Built => self.finish(window, cx),
            _ => cx.on_next_frame(window, Self::advance),
        }
    }

    /// Builds the shell around the loaded store and hands the kit its theme.
    fn build_shell(&mut self, state: AppState, window: &mut Window, cx: &mut Context<Self>) {
        let app = cx.new(|cx| AppView::with_state(state, window, cx));
        self.theme = app.read(cx).theme.clone();
        crate::theme::sync_kit_theme(&self.theme, cx);
        self.app = Some(app);
    }

    /// Keeps the splash up for its minimum time, then fades it out and drops it.
    fn finish(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let wait = fade_delay(self.shown_at.elapsed(), millis(self.theme.splash_min_duration_ms));
        let fade = millis(self.theme.splash_fade_duration_ms);
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(wait).await;
            let _ = this.update(cx, |this, cx| this.set_phase(SplashPhase::Fading, cx));
            cx.background_executor().timer(fade).await;
            let _ = this.update(cx, |this, cx| this.set_phase(SplashPhase::Done, cx));
        })
        .detach();
    }

    fn set_phase(&mut self, phase: SplashPhase, cx: &mut Context<Self>) {
        self.phase = phase;
        cx.notify();
    }
}
