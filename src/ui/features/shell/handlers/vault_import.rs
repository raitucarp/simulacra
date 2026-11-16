//! Knowledge vault file import: native picker, background read, add-document path.

use super::super::view::AppView;
use crate::ui::components::{
    pick_document_path, read_document_file, DocumentImportError, MAX_DOCUMENT_KB,
};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::path::Path;

/// Localized reason a picked file could not be imported.
fn import_error_reason(err: &DocumentImportError) -> String {
    match err {
        DocumentImportError::Unreadable(reason) => t!("vault.import_unreadable", reason = reason),
        DocumentImportError::Empty => t!("vault.import_empty"),
        DocumentImportError::TooLarge => t!("vault.import_too_large", max = MAX_DOCUMENT_KB),
    }
    .to_string()
}

impl AppView {
    /// Opens a native text-file picker and adds the chosen file to the active network's vault.
    pub(crate) fn import_document(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let title = t!("vault.import_dialog_title").to_string();
        cx.spawn_in(window, async move |entity, cx| {
            let Some(path) = pick_document_path(&title).await else { return };
            let read_path = path.clone();
            let parsed = cx.background_executor().spawn(async move { read_document_file(&read_path) }).await;
            let _ = entity.update_in(cx, |this, window, cx| {
                this.apply_imported_document(&path, parsed, window, cx);
            });
        })
        .detach();
    }

    /// Stores a read document and reports the outcome with a toast.
    fn apply_imported_document(
        &mut self,
        path: &Path,
        parsed: Result<(String, String), DocumentImportError>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let file = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let outcome = parsed.map_err(|e| import_error_reason(&e)).and_then(|(title, content)| {
            self.state.add_document(&title, &content).map(|_| title).map_err(|e| e.to_string())
        });
        let note = match outcome {
            Ok(title) => Notification::success(t!("vault.imported", title = title).to_string()),
            Err(reason) => Notification::error(t!("vault.import_failed", file = file, reason = reason).to_string()),
        };
        self.state.sync_network_counts();
        window.push_notification(note, cx);
        cx.notify();
    }
}
