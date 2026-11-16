//! Surfacing failed vault writes: the vault worker reports them on a channel that the
//! shell drains for the lifetime of the window.

use super::super::view::AppView;
use gpui_kit::*;

impl AppView {
    /// Toasts every write the vault worker fails to apply.
    pub(crate) fn watch_vault_failures(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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
}
