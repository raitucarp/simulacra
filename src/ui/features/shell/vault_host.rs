//! Knowledge Vault wiring of the shell: the vault view entity, keeping its data in step
//! with the store, and carrying out its requests (save, delete, import, reading progress,
//! open a linking post).

use super::view::AppView;
use crate::domain::document::record::{fnv1a, CatalogueRecord, OriginKind};
use crate::domain::embedding::{document_neighbours, NEIGHBOURS_PER_DOCUMENT};
use crate::domain::document::NewDocument;
use crate::state::{AppState, NavigationTab};
use crate::ui::features::vault::{EditorSubmission, VaultData, VaultEvent, VaultPanel};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;

/// The vault view and the fingerprint of the store data it was last given.
pub struct VaultHost {
    pub panel: Entity<VaultPanel>,
    fingerprint: u64,
}

impl VaultHost {
    /// Creates the vault view, returning its event subscription.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Subscription) {
        let (data, theme) = (VaultData::from_state(state), state.theme_tokens.clone());
        let panel = cx.new(|cx| VaultPanel::new(data, theme, window, cx));
        let subscription = cx.subscribe_in(&panel, window, AppView::handle_vault_event);
        (Self { panel, fingerprint: VaultData::fingerprint(state) }, subscription)
    }
}

impl AppView {
    /// Hands the vault fresh data when the store changed (cheap otherwise).
    pub(super) fn sync_vault(&mut self, cx: &mut Context<Self>) {
        let fingerprint = self.vault_fingerprint();
        if fingerprint != self.vault.fingerprint {
            self.push_vault_data(cx);
        } else {
            let theme = self.theme.clone();
            self.vault.panel.update(cx, |panel, cx| panel.set_theme(&theme, cx));
        }
    }

    /// The store fingerprint plus the vector count, so new embeddings refresh related entries.
    fn vault_fingerprint(&self) -> u64 {
        fnv1a(format!("{}|{}", VaultData::fingerprint(&self.state), self.semantic.index.len()).as_bytes())
    }

    /// The vault's data with each document's neighbours in meaning.
    fn vault_data(&self) -> VaultData {
        let data = VaultData::from_state(&self.state);
        let model = self.embedding_config().model;
        let neighbours = document_neighbours(&self.semantic.index, &model, data.documents.iter().map(|doc| doc.id.as_str()), NEIGHBOURS_PER_DOCUMENT);
        data.with_semantic(neighbours)
    }

    pub(super) fn push_vault_data(&mut self, cx: &mut Context<Self>) {
        self.vault.fingerprint = self.vault_fingerprint();
        let (data, theme) = (self.vault_data(), self.theme.clone());
        self.vault.panel.update(cx, |panel, cx| panel.set_data(data, &theme, cx));
    }

    /// Shows document `id`'s entry in the vault (wikilinks, source chips, graph nodes).
    pub(crate) fn open_vault_document(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.select_nav(NavigationTab::Knowledge, window, cx);
        self.push_vault_data(cx);
        self.vault.panel.update(cx, |panel, cx| panel.open_entry(id, cx));
    }

    fn handle_vault_event(&mut self, _: &Entity<VaultPanel>, event: &VaultEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event.clone() {
            VaultEvent::Save(submission) => self.save_vault_submission(submission, window, cx),
            VaultEvent::Delete(id) => {
                let _ = self.state.delete_document(&id);
                self.push_vault_data(cx);
            }
            VaultEvent::Import => self.pick_import_files(window, cx),
            VaultEvent::SetReading(id, reading) => {
                self.state.set_reading(&id, reading);
                self.push_vault_data(cx);
            }
            VaultEvent::OpenPost(post) => self.reveal_post(post, window, cx),
            VaultEvent::MakeFlashcards(id) => self.make_flashcards_document(&id, window, cx),
        }
        cx.notify();
    }

    /// Files the submission's terms and collections, then adds or updates the document.
    fn save_vault_submission(&mut self, submission: EditorSubmission, window: &mut Window, cx: &mut Context<Self>) {
        let subjects = self.state.ensure_subjects(&submission.subject_labels);
        let collections = submission.collection_names.iter().filter_map(|name| self.state.ensure_collection(name)).collect();
        let record = CatalogueRecord { subjects, collections, ..submission.record };
        let saved = match submission.editing {
            Some(id) => self.state.update_document(&id, &submission.title, &submission.content, record).map(|_| id),
            None => {
                let persona = self.state.current_profile().map(|p| p.name.clone()).unwrap_or_default();
                let new = NewDocument::new(self.state.active_network_id.clone(), submission.title.clone(), submission.content)
                    .with_record(record)
                    .from_origin(OriginKind::Typed, "")
                    .added_by(persona);
                self.state.add_new_document(new)
            }
        };
        match saved {
            Ok(id) => {
                self.push_vault_data(cx);
                self.vault.panel.update(cx, |panel, cx| panel.saved(&id, cx));
                window.push_notification(Notification::success(t!("vault.saved", title = submission.title).to_string()), cx);
            }
            Err(problems) => self.vault.panel.update(cx, |panel, cx| panel.save_failed(problems, cx)),
        }
    }
}
