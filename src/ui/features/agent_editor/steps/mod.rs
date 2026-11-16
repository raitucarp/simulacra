//! Tier 3: focused sub-tab bodies for each vertical category.

mod identity;
mod intelligence;
mod prompt;
mod psychology;
mod schedule;
mod trait_grid;
mod voice;

use super::callbacks::{PickAvatarCallback, SaveTextInputsCallback, UpdateProfileCallback};
use super::config_tab::AgentConfigTab;
use super::inputs::AgentEditorInputs;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::components::save_footer;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Everything a sub-tab body needs to render and to publish profile edits.
pub(crate) struct StepCtx<'a> {
    pub profile: &'a Profile,
    pub inputs: &'a AgentEditorInputs<'a>,
    pub theme: &'a ThemeTokens,
    pub network_name: &'a str,
    pub grounding_doc_titles: &'a [String],
    pub on_update: Rc<UpdateProfileCallback>,
    pub on_save_text: Rc<SaveTextInputsCallback>,
    pub on_pick_avatar: Rc<PickAvatarCallback>,
}

impl StepCtx<'_> {
    /// Callback that clones the profile, applies `apply(profile, value)` and publishes it.
    pub fn edit<T: 'static>(
        &self,
        apply: impl Fn(&mut Profile, T) + 'static,
    ) -> impl Fn(T, &mut Window, &mut App) + 'static {
        let base = self.profile.clone();
        let on_update = self.on_update.clone();
        move |value, w, cx| {
            let mut next = base.clone();
            apply(&mut next, value);
            on_update(next, w, cx);
        }
    }

    /// Value-less variant of [`StepCtx::edit`] (toggles, clear buttons).
    pub fn action(
        &self,
        apply: impl Fn(&mut Profile) + 'static,
    ) -> impl Fn(&mut Window, &mut App) + 'static {
        let edit = self.edit(move |p, ()| apply(p));
        move |w, cx| edit((), w, cx)
    }

    /// Right-aligned "Save changes" footer that commits the bound text inputs.
    pub fn save_footer(&self, id: &'static str) -> Div {
        let on_save = self.on_save_text.clone();
        save_footer(id, t!("lab.save_changes").to_string(), self.theme, move |w, cx| on_save(w, cx))
    }
}

/// Dispatches to the body of `sub_tab` within the vertical category `tab`.
pub(crate) fn render_step(tab: AgentConfigTab, sub_tab: usize, ctx: &StepCtx<'_>) -> AnyElement {
    match tab {
        AgentConfigTab::IdentityAndBio => identity::render(sub_tab, ctx),
        AgentConfigTab::PsychologyAndMbti => psychology::render(sub_tab, ctx),
        AgentConfigTab::IntelligenceAndMind => intelligence::render(sub_tab, ctx),
        AgentConfigTab::VoiceDictionGrammar => voice::render(sub_tab, ctx),
        AgentConfigTab::ScheduleAndMentions => schedule::render(sub_tab, ctx),
        AgentConfigTab::PromptAndAssembly => prompt::render(sub_tab, ctx),
    }
}
