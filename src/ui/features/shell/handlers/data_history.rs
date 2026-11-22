//! The history sheet in the shell (sb F2): opened on a post (its "…" menu or "edited" mark)
//! or a vault document (its entry), and "Restore this version", which is itself an edit, so
//! the version it replaces is kept too. Posts switch the version they show instead (V9).

use super::super::view::AppView;
use crate::domain::data::history::{current_index, version_list, RevisionSubject, VersionKind};
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
            DataEvent::SelectVersion(index) => { self.data.history.iter_mut().for_each(|sheet| sheet.pick(index, false)); }
            DataEvent::CompareWith(index) => { self.data.history.iter_mut().for_each(|sheet| sheet.pick(index, true)); } // V6
            DataEvent::RestoreVersion(index) => self.restore_version(index, window, cx),
            DataEvent::EditFromVersion(index) => self.edit_from_version(index, window, cx), // V4
            DataEvent::SwitchToVersion(index) => self.switch_to_version(index, window, cx), // V9
            _ => self.data.history = None,
        }
    }

    fn open_post_history(&mut self, id: PostId) {
        let Some(post) = self.state.get_post(&id) else { return };
        let (title, content) = (shortened(&post.content, TITLE_CHARS), post.content.clone());
        let (revisions, head) = (self.state.post_revisions(&id), self.state.post_head(&id));
        let subject = RevisionSubject::Post(id);
        self.data.history = Some(HistorySheet::new(subject, title, version_list("", &content, head.as_ref(), &revisions)));
    }

    fn open_document_history(&mut self, id: String) {
        let Some(doc) = self.state.documents.iter().find(|d| d.id == id) else { return };
        let (title, content) = (doc.title.clone(), doc.content.clone());
        let subject = RevisionSubject::Document(id);
        let revisions = self.state.db.as_ref().and_then(|db| db.revisions_of(subject.clone()).ok()).unwrap_or_default();
        self.data.history = Some(HistorySheet::new(subject, title.clone(), version_list(&title, &content, None, &revisions)));
    }

    /// Makes version `index` current again.
    fn restore_version(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.data.history.take() else { return };
        let Some(version) = sheet.versions.get(index).filter(|v| v.kind != VersionKind::Current).cloned() else { return };
        let restored = match &sheet.subject {
            RevisionSubject::Post(id) => self.state.edit_post(id, &version.content).map(|revision| {
                if revision.is_some() {
                    *self.data.revision_counts.entry(id.as_str().to_string()).or_default() += 1;
                }
            }).map_err(|err| err.to_string()),
            RevisionSubject::Document(id) => {
                let current = self.state.documents.iter().find(|d| &d.id == id).map(|d| d.record.clone()).unwrap_or_default();
                let record = serde_json::from_str::<CatalogueRecord>(&version.record_json).unwrap_or(current);
                let title = if version.title.is_empty() { sheet.versions[current_index(&sheet.versions)].title.clone() } else { version.title.clone() };
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

impl AppView {
    /// Closes the sheet and opens the composer on version `index` of its post: saving starts
    /// a branch from it (V4).
    fn edit_from_version(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.data.history.take() else { return };
        let (RevisionSubject::Post(id), Some(from)) = (&sheet.subject, sheet.versions.get(index).and_then(|v| v.revision.clone())) else { return };
        self.open_post_editor_from(id, Some(&from), window, cx);
    }
}

impl AppView {
    /// Closes the sheet and makes version `index` the one its post shows (V9).
    fn switch_to_version(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.data.history.take() else { return };
        let (RevisionSubject::Post(id), Some(to)) = (&sheet.subject, sheet.versions.get(index).and_then(|v| v.revision.clone())) else { return };
        match self.state.switch_post_head(id, &to, jiff::Timestamp::now()) {
            Ok(head) => {
                let text = rust_i18n::t!("data.history.switched", version = crate::ui::features::composer::version_label(head.version)).to_string();
                self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
            }
            Err(err) => window.push_notification(Notification::error(rust_i18n::t!("data.history.switch_failed", reason = err.to_string()).to_string()), cx),
        }
    }
}
