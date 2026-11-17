//! Hosting the Agent Forge: opening it for a new or existing profile, and carrying out what
//! it asks for (autosave, saving the profile, photo import, the guide switch, closing).

use super::view::AppView;
use crate::domain::portrait::AvatarMode;
use crate::domain::ProfileId;
use crate::state::forge::ForgeKind;
use crate::state::NavigationTab;
use crate::ui::features::forge::view::{ForgeContext, ForgeEvent, ForgeOpen, ForgeView, Notice};
use crate::ui::features::roster::LabMode;
use crate::ui::features::settings::SettingsTab;
use crate::ui::illustration::forget_stored_photo;
use gpui_kit::*;
use rust_i18n::t;
use std::path::Path;

/// The open forge and its event subscription.
pub struct ForgeSession {
    pub view: Entity<ForgeView>,
    _subscription: Subscription,
}

impl AppView {
    fn forge_context(&self) -> ForgeContext {
        ForgeContext {
            existing: self.state.user_profiles.clone(),
            network_id: self.state.active_network_id.clone(),
            llm: self.state.llm_settings.clone(),
            guide_hidden: self.state.forge_guide_hidden(),
            prompt: self.prompt_context(&self.state.active_network_id),
            credentials: self.state.credentials.clone(),
        }
    }

    fn open_forge(&mut self, open: ForgeOpen, window: &mut Window, cx: &mut Context<Self>) {
        let (context, theme) = (self.forge_context(), self.theme.clone());
        let view = cx.new(|cx| ForgeView::new(open, context, theme, cx));
        let subscription = cx.subscribe_in(&view, window, Self::on_forge_event);
        view.read(cx).focus.clone().focus(window, cx);
        self.forge = Some(ForgeSession { view, _subscription: subscription });
        cx.notify();
    }

    /// "New agent / persona": the class picker, or the resume prompt over an autosave.
    pub(crate) fn open_forge_new(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let kind = match self.current_lab_mode() {
            LabMode::NetworkAgents => ForgeKind::Agent,
            LabMode::PersonaLab => ForgeKind::Persona,
        };
        let autosave = self.state.load_forge_autosave().map(Box::new);
        self.open_forge(ForgeOpen::New { kind, autosave }, window, cx);
    }

    /// "Edit in Forge": opens `profile_id` at Origin.
    pub(crate) fn open_forge_edit(&mut self, profile_id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(profile) = self.find_profile(&profile_id) {
            self.open_forge(ForgeOpen::Edit(Box::new(profile)), window, cx);
        }
    }

    /// Asks an open forge to close (it autosaves unsaved work first).
    pub(crate) fn close_forge(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(view) = self.forge.as_ref().map(|session| session.view.clone()) {
            view.update(cx, |forge, cx| forge.close(window, cx));
        }
    }

    fn on_forge_event(&mut self, forge: &Entity<ForgeView>, event: &ForgeEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            ForgeEvent::Close => self.forge = None,
            ForgeEvent::Autosave(draft) => self.state.save_forge_autosave(draft),
            ForgeEvent::ClearAutosave => self.state.clear_forge_autosave(),
            ForgeEvent::GuideHidden(hidden) => self.state.set_forge_guide_hidden(*hidden),
            ForgeEvent::Commit { profile, bring_to_life, intro_post } => {
                self.commit_forged(forge, (**profile).clone(), (*bring_to_life, *intro_post), window, cx)
            }
            ForgeEvent::ImportPhoto(path) => self.import_forge_photo(forge, path, cx),
            ForgeEvent::OpenAiSettings => self.open_ai_settings(window, cx),
        }
        cx.notify();
    }

    /// Stores the finished profile, selects it in the roster and runs its first turn if asked.
    fn commit_forged(
        &mut self,
        forge: &Entity<ForgeView>,
        profile: crate::domain::Profile,
        (bring_to_life, intro_post): (bool, bool),
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let (id, is_agent) = (profile.id.clone(), profile.is_agent());
        if let Err(err) = self.state.update_profile(profile) {
            self.report_storage_failure(&err, window, cx);
            return;
        }
        self.state.clear_forge_autosave();
        self.state.active_tab = if is_agent { NavigationTab::AgentLab } else { NavigationTab::PersonaLab };
        self.lab.selected_profile_id = Some(id.clone());
        forge.update(cx, |forge, cx| forge.committed(bring_to_life, cx));
        if intro_post && is_agent {
            self.trigger_simulation_for_agent(Some(id), window, cx);
        }
    }

    /// Copies a picked photo into the vault and hands the avatar to the forge.
    fn import_forge_photo(&mut self, forge: &Entity<ForgeView>, path: &Path, cx: &mut Context<Self>) {
        let draft_id = forge.read(cx).draft.id().clone();
        match self.state.import_avatar_photo(&draft_id, path) {
            Ok(mode) => {
                if let AvatarMode::Photo { path: stored, .. } = &mode {
                    forget_stored_photo(stored).into_iter().for_each(|image| image.remove_asset(cx));
                }
                forge.update(cx, |forge, cx| forge.apply_avatar(mode, cx));
            }
            Err(err) => {
                let text = t!("portrait.import_failed", error = err.to_string()).to_string();
                forge.update(cx, |forge, cx| forge.set_notice(Notice::error(text), cx));
            }
        }
    }

    /// Keeps an open forge on the current theme and the stored credentials.
    pub(super) fn sync_forge_theme(&self, cx: &mut Context<Self>) {
        if let Some(session) = &self.forge {
            let (theme, credentials) = (self.theme.clone(), self.state.credentials.clone());
            session.view.update(cx, |forge, cx| {
                forge.set_theme(&theme, cx);
                forge.set_credentials(&credentials, cx);
            });
        }
    }

    /// Opens the settings modal on its AI tab (credentials), keeping the forge underneath.
    fn open_ai_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.revert_settings_draft(window, cx);
        self.chrome.is_settings_open = true;
        self.chrome.active_settings_tab = SettingsTab::AiIntelligence;
    }
}

