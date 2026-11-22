//! Settings window wiring: the data it shows and the callbacks it raises.

use super::super::binder::Binder;
use super::super::scheduler_state::HEARTBEAT_INTERVAL;
use super::super::view::AppView;
use crate::ui::features::settings::{
    AiPaneProps, CredentialEditorView, CredentialRowView, CredentialsPaneProps, PersonaOption, SchedulerStatus, SettingsActions,
    SettingsData, SettingsModal,
};
use gpui_kit::App;
use std::rc::Rc;

impl AppView {
    /// Settings window, while open.
    pub(super) fn settings_modal(&self, bind: &Binder, cx: &App) -> Option<SettingsModal> {
        self.chrome.is_settings_open.then(|| {
            SettingsModal::new(
                self.theme.clone(),
                self.chrome.active_settings_tab,
                self.settings_data(cx),
                self.settings_ui.inputs(),
                settings_actions(bind),
                self.appearance.clone(),
            )
            .ai(self.ai_pane_props(bind))
            .credentials(self.credentials_pane_props(bind))
            .symbols(self.symbol_sets_props(bind), self.licence_views()) // S13
        })
    }

    /// Preferences, theme scales, personas, folders, scheduler status and flags.
    fn settings_data(&self, cx: &App) -> SettingsData {
        let state = &self.state;
        let personas = state
            .user_profiles
            .iter()
            .filter(|profile| profile.is_human_persona())
            .map(|profile| PersonaOption { id: profile.id.to_string(), name: profile.name.clone() })
            .collect();
        let ai_dirty = self.chrome.settings_draft.is_unsaved(&state.llm_settings, self.llm_inputs.values(cx));
        SettingsData {
            prefs: state.preferences().clone(),
            density: state.theme_settings.density,
            type_scale: state.theme_settings.type_scale,
            personas,
            database_path: state.db.as_ref().map(|db| db.path()).unwrap_or_default(),
            media_path: state.media_dir().display().to_string(),
            scheduler: self.scheduler_status(),
            query: self.settings_ui.query(cx),
            ai_dirty,
            confirm_reset: self.settings_ui.confirm_reset,
        }
    }

    /// Live scheduler switch, heartbeat, queue and last failure.
    fn scheduler_status(&self) -> SchedulerStatus {
        let scheduler = &self.scheduler;
        SchedulerStatus {
            enabled: scheduler.enabled,
            heartbeat_seconds: HEARTBEAT_INTERVAL.as_secs(),
            running: scheduler.queue.in_flight_len(),
            queued: scheduler.queue.pending_len(),
            last_error: scheduler.last_error.as_ref().map(|e| (e.provider.to_string(), e.message.clone())),
        }
    }

    /// The provider draft, its inputs and the provider / preset callbacks.
    fn ai_pane_props(&self, bind: &Binder) -> AiPaneProps {
        AiPaneProps {
            settings: self.chrome.settings_draft.view(&self.state.llm_settings),
            api_key: self.llm_inputs.api_key.clone(),
            model: self.llm_inputs.model.clone(),
            endpoint: self.llm_inputs.endpoint.clone(),
            on_select_provider: Box::new(bind.with(Self::select_provider)),
            on_select_preset: Box::new(bind.with(Self::select_preset_model)),
        }
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
}

/// Every callback of the settings window.
fn settings_actions(bind: &Binder) -> SettingsActions {
    let edit = bind.entity().clone();
    SettingsActions {
        on_edit: Rc::new(move |field, value, window, cx| edit.update(cx, |view, cx| view.edit_preference(field, value, window, cx))),
        on_reset: Rc::new(bind.with(AppView::reset_setting_row)),
        on_command: Rc::new(bind.with(AppView::run_settings_command)),
        on_select: Rc::new(bind.with(AppView::select_settings_tab)),
        on_close: Rc::new(bind.act(AppView::close_settings)),
        on_save: Rc::new(bind.act(AppView::save_settings)),
    }
}
