//! Surfacing vault problems: a vault that could not be opened at startup gets one lasting
//! notice, and failed writes are reported on a channel the shell drains for the lifetime of
//! the window.

use super::super::view::AppView;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;

/// Notification id type for the one-time "vault could not be opened" notice.
struct VaultLoadNotice;

impl AppView {
    /// Shows the startup load failure (if any) once the window is up, then toasts every
    /// write the vault worker fails to apply.
    pub(crate) fn watch_vault_failures(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.report_load_failure(window, cx);
        let Some(mut failures) = self.state.db.as_ref().and_then(|vault| vault.take_failures()) else {
            return;
        };
        cx.spawn_in(window, async move |entity, cx| {
            while let Some(err) = failures.recv().await {
                let shown = entity.update_in(cx, |this, window, cx| this.report_storage_failure(&err, window, cx));
                if shown.is_err() {
                    break;
                }
            }
        })
        .detach();
    }

    /// Takes the store's startup load error and shows it as a notice that stays until closed
    /// (deferred to the first frame, when the window's notification layer exists).
    fn report_load_failure(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(err) = self.state.load_error.take() else { return };
        let message = t!("storage.load_failed", error = err.to_string()).to_string();
        let title = t!("storage.load_failed_title").to_string();
        let note = Notification::error(message).title(title).autohide(false).id::<VaultLoadNotice>();
        cx.spawn_in(window, async move |_, cx| {
            let _ = cx.update(|window, cx| window.push_notification(note, cx));
        })
        .detach();
    }
}
