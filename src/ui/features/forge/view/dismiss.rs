//! Leaving the forge: Escape (hint cards first, then a focused text field) and the close button.

use super::keys::Dismiss;
use super::{ForgeEvent, ForgeScreen, ForgeView};
use crate::ui::features::forge::logic::{dismiss_step, DismissStep};
use crate::ui::features::forge::widgets::close_explainers;
use gpui_kit::*;

impl ForgeView {
    /// Escape: closes any open hint card first; else, inside a text field, only leaves the
    /// field (focus returns to the forge); otherwise leaves the forge.
    pub(crate) fn on_dismiss(&mut self, _: &Dismiss, window: &mut Window, cx: &mut Context<Self>) {
        match dismiss_step(close_explainers(cx), &window.context_stack()) {
            DismissStep::CloseHints => cx.notify(),
            DismissStep::LeaveTextField => {
                self.focus.focus(window, cx);
                cx.notify();
            }
            DismissStep::CloseForge => self.close(window, cx),
        }
    }

    /// Leaves the forge, keeping unsaved work in the autosave slot (and an autosave still
    /// waiting on the resume prompt untouched).
    pub fn close(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.screen, ForgeScreen::Resume(_)) {
            // The stored draft stays for next time.
        } else if self.draft.is_dirty() {
            cx.emit(ForgeEvent::Autosave(Box::new(self.draft.clone())));
        } else {
            cx.emit(ForgeEvent::ClearAutosave);
        }
        cx.emit(ForgeEvent::Close);
    }
}
