//! The first-run welcome: showing it, opening the forge from it, skipping it and replaying it
//! from Settings › General. Completing or skipping is remembered in the settings table.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::state::NavigationTab;
use crate::ui::features::onboarding::WelcomeOverlay;
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// The welcome overlay, while open (never over an open forge).
    pub(in crate::ui::features::shell) fn welcome_overlay(&self, bind: &Binder) -> Option<WelcomeOverlay> {
        (self.chrome.is_onboarding_open && self.forge.is_none()).then(|| {
            WelcomeOverlay::new(&self.theme, Rc::new(bind.act(Self::start_from_welcome)), Rc::new(bind.act(Self::skip_welcome)))
        })
    }

    /// "Create your first thinker": marks the welcome done and opens the forge's class picker.
    pub(crate) fn start_from_welcome(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.finish_welcome(cx);
        self.state.active_tab = NavigationTab::AgentLab;
        self.open_forge_new(window, cx);
    }

    /// "Skip for now" (or a click on the scrim).
    pub(crate) fn skip_welcome(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.finish_welcome(cx);
    }

    /// Shows the welcome again (Settings › General), closing the settings modal.
    pub(crate) fn replay_welcome(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.close_settings(window, cx);
        self.chrome.is_onboarding_open = true;
        cx.notify();
    }

    fn finish_welcome(&mut self, cx: &mut Context<Self>) {
        self.chrome.is_onboarding_open = false;
        self.state.set_onboarding_done(true);
        cx.notify();
    }
}
