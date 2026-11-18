//! Image ideas (sb D3): taking one of the AI's ideas into the prompt, and putting a drawn
//! picture where it was asked for (the forged draft's avatar, a stored profile's avatar, or
//! the composer draft's attachments).

use super::super::view::AppView;
use crate::domain::portrait::AvatarMode;
use crate::domain::ProfileId;
use crate::ui::features::image_prompt::{CandidatesStatus, IdeasDestination};
use crate::ui::illustration::forget_stored_photo;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::path::PathBuf;

impl AppView {
    /// Puts the AI's idea at `index` into the prompt editor.
    pub(in crate::ui::features::shell) fn use_image_candidate(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let idea = match self.image_ideas.sheet.as_ref().map(|sheet| &sheet.candidates) {
            Some(CandidatesStatus::Ready(ideas)) => ideas.get(index).cloned(),
            _ => None,
        };
        if let Some(idea) = idea {
            self.image_ideas.prompt.update(cx, |editor, cx| editor.set_value(idea, window, cx));
            cx.notify();
        }
    }

    /// Uses the drawn picture at `index`, then closes the sheet.
    pub(in crate::ui::features::shell) fn use_image_result(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.image_ideas.sheet.as_ref() else { return };
        let Some(path) = sheet.results.get(index).map(PathBuf::from) else { return };
        match sheet.destination.clone() {
            IdeasDestination::Forge => {
                if let Some(forge) = self.forge.as_ref().map(|session| session.view.clone()) {
                    self.import_forge_photo(&forge, &path, cx);
                }
            }
            IdeasDestination::Profile(id) => self.set_profile_photo(&id, &path, window, cx),
            IdeasDestination::Composer => self.composer_modal.update(cx, |panel, cx| panel.attach_paths(vec![path], window, cx)),
        }
        self.close_image_ideas(window, cx);
    }

    /// Imports `path` as the photo avatar of the stored profile `id`.
    fn set_profile_photo(&mut self, id: &ProfileId, path: &std::path::Path, window: &mut Window, cx: &mut Context<Self>) {
        let Some(mut profile) = self.find_profile(id) else { return };
        match self.state.import_avatar_photo(id, path) {
            Ok(mode) => {
                if let AvatarMode::Photo { path: stored, .. } = &mode {
                    forget_stored_photo(stored).into_iter().for_each(|image| image.remove_asset(cx));
                }
                profile.config.avatar = Some(mode);
                if let Err(err) = self.state.update_profile(profile) {
                    self.report_storage_failure(&err, window, cx);
                }
            }
            Err(err) => {
                let text = t!("portrait.import_failed", error = err.to_string()).to_string();
                window.push_notification(Notification::error(text), cx);
            }
        }
    }
}
