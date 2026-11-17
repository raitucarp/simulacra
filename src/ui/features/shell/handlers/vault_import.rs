//! Knowledge vault file import: native picker, background read, add-document path.

use super::super::view::AppView;
use crate::domain::document::record::OriginKind;
use crate::domain::document::NewDocument;
use crate::ui::features::vault::problem_message;
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
        let persona = self.state.current_profile().map(|p| p.name.clone()).unwrap_or_default();
        let network = self.state.active_network_id.clone();
        let outcome = parsed.map_err(|e| import_error_reason(&e)).and_then(|(title, content)| {
            let new = NewDocument::new(network, title.clone(), content).from_origin(OriginKind::File, file.clone()).added_by(persona);
            let added = self.state.add_new_document(new);
            added.map(|id| (id, title)).map_err(|problems| problems.into_iter().map(problem_message).collect::<Vec<_>>().join(" "))
        });
        let note = match &outcome {
            Ok((_, title)) => Notification::success(t!("vault.imported", title = title).to_string()),
            Err(reason) => Notification::error(t!("vault.import_failed", file = file, reason = reason).to_string()),
        };
        window.push_notification(note, cx);
        if let Ok((id, _)) = outcome {
            self.open_vault_document(&id, window, cx);
        }
        cx.notify();
    }
}
