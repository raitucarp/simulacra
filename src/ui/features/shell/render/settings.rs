//! Settings modal wiring.

use super::super::binder::Binder;
use super::super::scheduler_state::HEARTBEAT_INTERVAL;
use super::super::view::AppView;
use crate::ui::features::settings::{CredentialEditorView, CredentialRowView, CredentialsPaneProps, GeneralPaneProps, SettingsModal};
use std::rc::Rc;

impl AppView {
    /// Settings modal, while open.
    pub(super) fn settings_modal(&self, bind: &Binder) -> Option<SettingsModal> {
        self.chrome.is_settings_open.then(|| {
            SettingsModal::new(
                self.theme.clone(),
                self.chrome.settings_draft.view(&self.state.llm_settings),
                self.chrome.active_settings_tab,
                self.llm_inputs.api_key.clone(),
                self.llm_inputs.model.clone(),
                self.llm_inputs.endpoint.clone(),
                bind.with(Self::select_settings_tab),
                self.appearance.clone(),
                bind.with(Self::select_provider),
                bind.with(Self::select_preset_model),
                bind.act(Self::save_settings),
                bind.act(Self::close_settings),
            )
            .general(self.general_pane_props(bind))
            .credentials(self.credentials_pane_props(bind))
        })
    }

    /// Stored credentials (masked), the open form and its callbacks.
    fn credentials_pane_props(&self, bind: &Binder) -> CredentialsPaneProps {
        let inputs = &self.llm_inputs.credential;
        let editor = inputs.editing.as_ref().map(|edit| CredentialEditorView {
            is_new: edit.id.is_none(),
            kind: edit.kind,
            label: inputs.label.clone(),
            secret: inputs.secret.clone(),
            endpoint: inputs.endpoint.clone(),
        });
        CredentialsPaneProps {
            rows: self.state.credentials.items.iter().map(CredentialRowView::of).collect(),
            editor,
            on_add: Rc::new(bind.act(Self::add_credential)),
            on_edit: Rc::new(bind.with(Self::edit_credential)),
            on_remove: Rc::new(bind.with(Self::remove_credential)),
            on_pick_kind: Rc::new(bind.with(Self::pick_credential_kind)),
            on_commit: Rc::new(bind.act(Self::commit_credential)),
            on_cancel: Rc::new(bind.act(Self::cancel_credential)),
        }
    }

    /// Live scheduler status, vault path and switch callback for the General pane.
    fn general_pane_props(&self, bind: &Binder) -> GeneralPaneProps {
        let scheduler = &self.scheduler;
        GeneralPaneProps {
            scheduler_enabled: scheduler.enabled,
            heartbeat_seconds: HEARTBEAT_INTERVAL.as_secs(),
            running: scheduler.queue.in_flight_len(),
            queued: scheduler.queue.pending_len(),
            last_error: scheduler.last_error.as_ref().map(|e| (e.provider.to_string(), e.message.clone())),
            db_path: self.state.db.as_ref().map(|db| db.path()).unwrap_or_default(),
            language_label: self.chrome.current_language.native_label(),
            on_toggle_scheduler: Some(bind.boxed(Self::set_scheduler_enabled)),
            on_replay_welcome: Some(bind.boxed_act(Self::replay_welcome)),
        }
    }
}
