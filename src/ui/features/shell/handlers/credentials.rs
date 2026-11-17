//! Credential store handlers (Settings › AI Intelligence › Credentials). Changes are saved
//! immediately and forget every failure backoff, so agents retry with the new keys.

use super::super::credential_state::{credential_from_form, CredentialEdit};
use super::super::view::AppView;
use crate::agent::credentials::{Credential, CredentialKind, CredentialStore, DEFAULT_CREDENTIAL_ID};
use gpui_kit::*;

impl AppView {
    /// Opens the form for a new credential.
    pub(crate) fn add_credential(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.llm_inputs.credential.fill("", "", window, cx);
        self.llm_inputs.credential.editing = Some(CredentialEdit { id: None, kind: CredentialKind::default() });
        cx.notify();
    }

    /// Opens the form on the stored credential `id`.
    pub(crate) fn edit_credential(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        let Some(stored) = self.state.credentials.get(&id).cloned() else { return };
        self.llm_inputs.credential.fill(&stored.label, &stored.endpoint, window, cx);
        self.llm_inputs.credential.editing = Some(CredentialEdit { id: Some(id), kind: stored.kind });
        cx.notify();
    }

    /// Picks the service of the open form.
    pub(crate) fn pick_credential_kind(&mut self, kind: CredentialKind, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(edit) = self.llm_inputs.credential.editing.as_mut() {
            edit.kind = kind;
            cx.notify();
        }
    }

    /// Stores the form and closes it.
    pub(crate) fn commit_credential(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(edit) = self.llm_inputs.credential.editing.clone() else { return };
        let values = self.llm_inputs.credential.values(cx);
        let existing = edit.id.as_deref().and_then(|id| self.state.credentials.get(id));
        let credential = credential_from_form(&edit, existing, values);
        if credential.id == DEFAULT_CREDENTIAL_ID {
            self.mirror_default_credential(&credential, window, cx);
        }
        let mut store = self.state.credentials.clone();
        store.upsert(credential);
        self.save_credentials(store);
        self.cancel_credential(window, cx);
    }

    /// The default credential mirrors the global settings, so editing it edits them too.
    fn mirror_default_credential(&mut self, credential: &Credential, window: &mut Window, cx: &mut Context<Self>) {
        let Some(provider) = credential.kind.provider() else { return };
        let mut settings = self.state.llm_settings.clone();
        if settings.provider != provider {
            settings.model = provider.default_model().to_string();
        }
        settings.provider = provider;
        settings.api_key = credential.secret.clone();
        settings.custom_endpoint = credential.endpoint.clone();
        self.state.update_llm_settings(settings);
        self.revert_settings_draft(window, cx);
    }

    /// Removes the credential `id` (closing the form if it was being edited).
    pub(crate) fn remove_credential(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        let mut store = self.state.credentials.clone();
        if store.remove(&id) {
            self.save_credentials(store);
        }
        let editing_it = self.llm_inputs.credential.editing.as_ref().is_some_and(|edit| edit.id.as_deref() == Some(id.as_str()));
        if editing_it {
            self.cancel_credential(window, cx);
        }
        cx.notify();
    }

    /// Closes the form and clears its inputs.
    pub(crate) fn cancel_credential(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.llm_inputs.credential.editing = None;
        self.llm_inputs.credential.fill("", "", window, cx);
        cx.notify();
    }

    fn save_credentials(&mut self, store: CredentialStore) {
        self.state.update_credentials(store);
        self.reset_llm_failures();
    }
}
