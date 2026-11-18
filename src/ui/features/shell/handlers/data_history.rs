//! The history sheet in the shell (sb F2): opened on a post (its "…" menu or "edited" mark)
//! or a vault document (its entry), and "Restore this version", which is itself an edit, so
//! the version it replaces is kept too.

use super::super::view::AppView;
use crate::domain::data::history::{version_list, RevisionSubject};
use crate::domain::document::CatalogueRecord;
use crate::domain::preferences::ToastKind;
use crate::domain::PostId;
use crate::state::semantic::hits::shortened;
use crate::ui::features::data::{tr, DataEvent, HistorySheet};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;

/// Characters of a post shown as the sheet's subtitle.
const TITLE_CHARS: usize = 80;

impl AppView {
    pub(in crate::ui::features::shell) fn on_history_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            DataEvent::OpenPostHistory(id) => self.open_post_history(id),
            DataEvent::OpenDocumentHistory(id) => self.open_document_history(id),
            DataEvent::SelectVersion(index) => {
                if let Some(sheet) = self.data.history.as_mut() {
                    sheet.selected = index.min(sheet.versions.len().saturating_sub(1));
                }
            }
            DataEvent::RestoreVersion(index) => self.restore_version(index, window, cx),
            _ => self.data.history = None,
        }
    }

    fn open_post_history(&mut self, id: PostId) {
        let Some(post) = self.state.get_post(&id) else { return };
        let (title, content) = (shortened(&post.content, TITLE_CHARS), post.content.clone());
        let subject = RevisionSubject::Post(id);
        let revisions = self.state.db.as_ref().and_then(|db| db.revisions_of(subject.clone()).ok()).unwrap_or_default();
        self.data.history = Some(HistorySheet::new(subject, title, version_list("", &content, &revisions)));
    }

    fn open_document_history(&mut self, id: String) {
        let Some(doc) = self.state.documents.iter().find(|d| d.id == id) else { return };
        let (title, content) = (doc.title.clone(), doc.content.clone());
        let subject = RevisionSubject::Document(id);
        let revisions = self.state.db.as_ref().and_then(|db| db.revisions_of(subject.clone()).ok()).unwrap_or_default();
        self.data.history = Some(HistorySheet::new(subject, title.clone(), version_list(&title, &content, &revisions)));
    }

    /// Makes version `index` current again.
    fn restore_version(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.data.history.take() else { return };
        let Some(version) = sheet.versions.get(index).filter(|_| index > 0).cloned() else { return };
        let restored = match &sheet.subject {
            RevisionSubject::Post(id) => self.state.edit_post(id, &version.content).map(|revision| {
                if revision.is_some() {
                    *self.data.revision_counts.entry(id.as_str().to_string()).or_default() += 1;
                }
            }).map_err(|err| err.to_string()),
            RevisionSubject::Document(id) => {
                let current = self.state.documents.iter().find(|d| &d.id == id).map(|d| d.record.clone()).unwrap_or_default();
                let record = serde_json::from_str::<CatalogueRecord>(&version.record_json).unwrap_or(current);
                let title = if version.title.is_empty() { sheet.versions[0].title.clone() } else { version.title.clone() };
                let saved = self.state.update_document(id, &title, &version.content, record).map_err(|problems| format!("{problems:?}"));
                self.push_vault_data(cx);
                saved
            }
        };
        match restored {
            Ok(()) => self.toast(ToastKind::Confirmation, Notification::success(tr("data.history.restored")), window, cx),
            Err(reason) => window.push_notification(Notification::error(rust_i18n::t!("data.history.restore_failed", reason = reason).to_string()), cx),
        }
    }
}
