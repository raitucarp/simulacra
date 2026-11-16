//! Agent & Persona Lab profile creation and the async avatar picker.

use super::super::consts::{
    CuratedIdentity, CURATED_AGENTS, CURATED_AGENT_OFFSET, CURATED_PERSONAS,
    CURATED_PERSONA_OFFSET, FALLBACK_AGENT, FALLBACK_PERSONA, IMAGE_EXTENSIONS, IMAGE_FILTER_NAME,
    PICK_AVATAR_TITLE,
};
use super::super::editor_field::EditorTextField;
use super::super::view::AppView;
use crate::domain::ProfileId;
use crate::state::NavigationTab;
use crate::ui::features::agent_editor::AgentConfigTab;
use gpui_kit::*;
use std::path::Path;

impl AppView {
    /// Creates the next curated persona (Persona Lab) or agent and opens it in the editor.
    pub(crate) fn lab_create_profile(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.save_agent_editor_text_inputs(cx);
        let res = if self.state.active_tab == NavigationTab::PersonaLab {
            let count = self.state.current_network_personas().len() + 1;
            let pick = curated(&CURATED_PERSONAS, CURATED_PERSONA_OFFSET, FALLBACK_PERSONA, count);
            let (name, handle, role) = pick;
            let handle = self.unique_handle(handle, count);
            self.state.create_human_persona(name, &handle, role)
        } else {
            let count = self.state.current_network_agents().len() + 1;
            let pick = curated(&CURATED_AGENTS, CURATED_AGENT_OFFSET, FALLBACK_AGENT, count);
            let (name, handle, role) = pick;
            let handle = self.unique_handle(handle, count);
            self.state.create_network_agent(name, &handle, role)
        };
        if let Ok(new_id) = res {
            self.lab.active_config_tab = AgentConfigTab::IdentityAndBio;
            self.lab.active_sub_tab = 0;
            self.select_lab_profile(new_id, window, cx);
        }
    }

    /// `handle`, suffixed with `_<count>` when another profile already uses it.
    fn unique_handle(&self, handle: &str, count: usize) -> String {
        if self.state.user_profiles.iter().any(|p| p.handle == handle) {
            format!("{}_{}", handle, count)
        } else {
            handle.to_string()
        }
    }

    /// Opens a native image dialog and stores the chosen file as the profile's avatar.
    pub(crate) fn lab_pick_avatar(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        cx.spawn_in(window, async move |entity, cx| {
            let file = rfd::AsyncFileDialog::new()
                .set_title(PICK_AVATAR_TITLE)
                .add_filter(IMAGE_FILTER_NAME, &IMAGE_EXTENSIONS)
                .pick_file()
                .await;
            let Some(file_handle) = file else { return };
            let path = file_handle.path().to_string_lossy().to_string();
            let _ = entity.update_in(cx, |this, window, cx| {
                this.apply_picked_avatar(&profile_id, path, window, cx);
            });
        })
        .detach();
    }

    fn apply_picked_avatar(
        &mut self,
        profile_id: &ProfileId,
        path: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !Path::new(&path).exists() {
            return;
        }
        let Some(mut prof) = self.find_profile(profile_id) else {
            return;
        };
        prof.avatar_path = Some(path.clone());
        let _ = self.state.update_profile(prof);
        // Keep the path input in step with the new avatar; otherwise
        // the next "Save Changes" writes the stale path back.
        if self.lab.selected_profile_id.as_ref() == Some(profile_id) {
            self.set_editor_field(EditorTextField::AvatarPath, &path, window, cx);
        }
        cx.notify();
    }
}

/// The curated identity for the `count`-th member, or `fallback` past the end of the list.
fn curated(list: &[CuratedIdentity], offset: usize, fallback: CuratedIdentity, count: usize) -> CuratedIdentity {
    list.get(count.saturating_sub(offset)).copied().unwrap_or(fallback)
}
