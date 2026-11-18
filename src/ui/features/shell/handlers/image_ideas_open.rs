//! Image ideas (sb D3): the three ways in (the Forge's appearance step, the composer and the
//! palette command) and the dispatch of every sheet event.

use super::super::view::AppView;
use super::image_ideas::IdeasOpen;
use crate::domain::image_prompt::{subject_from_config, subject_from_post, ImageTarget};
use crate::domain::Profile;
use crate::state::semantic::hits::shortened;
use crate::ui::features::image_prompt::{IdeasDestination, IdeasEvent};
use crate::ui::features::settings::SettingsTab;
use gpui_kit::*;

/// Characters of the post shown in the sheet's subtitle.
const SOURCE_CHARS: usize = 60;

fn avatar_open(profile: &Profile, destination: IdeasDestination) -> IdeasOpen {
    IdeasOpen {
        target: ImageTarget::Avatar,
        destination,
        about: profile.name.clone(),
        subject: subject_from_config(&profile.config),
        own_engine: profile.config.engine.image.clone(),
    }
}

impl AppView {
    /// "Image ideas" in the Forge: an avatar for the draft being forged.
    pub(in crate::ui::features::shell) fn open_image_ideas_for_forge(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(profile) = self.forge.as_ref().map(|session| session.view.read(cx).draft.profile().clone()) else { return };
        self.open_image_ideas(avatar_open(&profile, IdeasDestination::Forge), window, cx);
    }

    /// "Image ideas" in the composer: a picture for the post being written.
    pub(in crate::ui::features::shell) fn open_image_ideas_for_draft(&mut self, text: String, window: &mut Window, cx: &mut Context<Self>) {
        let own_engine = self.state.current_profile().map(|p| p.config.engine.image.clone()).unwrap_or_default();
        let open = IdeasOpen {
            target: ImageTarget::Post,
            destination: IdeasDestination::Composer,
            about: shortened(&text, SOURCE_CHARS),
            subject: subject_from_post(&text),
            own_engine,
        };
        self.open_image_ideas(open, window, cx);
    }

    /// The palette's "Image ideas": an avatar for the agent or persona selected in a roster,
    /// else for the persona you post as.
    pub(crate) fn open_image_ideas_command(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.forge.is_some() {
            return self.open_image_ideas_for_forge(window, cx);
        }
        let selected = self.is_lab_tab().then(|| self.lab.selected_profile_id.clone()).flatten().and_then(|id| self.find_profile(&id));
        let Some(profile) = selected.or_else(|| self.state.current_profile().cloned()) else { return };
        self.open_image_ideas(avatar_open(&profile, IdeasDestination::Profile(profile.id.clone())), window, cx);
    }

    pub(crate) fn on_image_ideas_event(&mut self, event: IdeasEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            IdeasEvent::Pick(pick) => self.pick_image_idea(pick, window, cx),
            IdeasEvent::Surprise => self.surprise_image_idea(window, cx),
            IdeasEvent::WriteForMe => self.write_image_ideas(window, cx),
            IdeasEvent::UseCandidate(index) => self.use_image_candidate(index, window, cx),
            IdeasEvent::Generate => self.generate_image_idea(window, cx),
            IdeasEvent::Cancel => self.cancel_image_ideas(cx),
            IdeasEvent::UseResult(index) => self.use_image_result(index, window, cx),
            IdeasEvent::OpenSettings => {
                self.close_image_ideas(window, cx);
                self.open_settings_tab(SettingsTab::Images, window, cx);
            }
            IdeasEvent::Close => self.close_image_ideas(window, cx),
        }
    }
}
