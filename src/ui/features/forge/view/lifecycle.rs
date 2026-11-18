//! Opening the forge, resuming an autosave, and what happens after the host saved the profile.

use super::state::{ForgeContext, ForgeOpen, ForgeScreen, GuideState, Notice, StartState};
use super::{ForgeEvent, ForgeView, SuggestUi};
use crate::domain::portrait::{AvatarMode, FormLayer, FormSlot, PortraitSlot};
use crate::domain::Profile;
use crate::state::forge::{AgentDraft, AvatarField, DraftEdit, ForgeKind};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::AutosavePolicy;
use gpui_kit::*;

impl ForgeView {
    pub fn new(open: ForgeOpen, context: ForgeContext, theme: ThemeTokens, cx: &mut Context<Self>) -> Self {
        let kind = match &open {
            ForgeOpen::New { kind, .. } => *kind,
            ForgeOpen::Edit(profile) => ForgeKind::of(profile),
        };
        let (draft, screen) = match open {
            ForgeOpen::New { autosave: Some(saved), .. } => (AgentDraft::new(kind, context.network_id.clone()), ForgeScreen::Resume(saved)),
            ForgeOpen::New { autosave: None, .. } => (AgentDraft::new(kind, context.network_id.clone()), ForgeScreen::Start),
            ForgeOpen::Edit(profile) => (AgentDraft::edit_existing(*profile), ForgeScreen::Steps),
        };
        let guide = GuideState { hidden: context.guide_hidden, fresh_step: true, thinking: false };
        let mut view = Self {
            theme,
            draft,
            context,
            screen,
            issues: Vec::new(),
            autosave: AutosavePolicy::default(),
            guide,
            start: StartState::new(kind),
            intro_post: true,
            notice: None,
            portrait_slot: PortraitSlot::Face,
            form_slot: FormSlot::Layer(FormLayer::Body),
            hear: super::HearStatus::Idle,
            grimoire_open: false,
            engine: super::EngineUi::default(),
            strip_preview: None,
            suggest: SuggestUi::default(),
            focus: cx.focus_handle(),
        };
        view.revalidate();
        view
    }

    /// Replaces the theme after an appearance change.
    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }

    /// Continues the stored draft where it was left.
    pub fn resume(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let ForgeScreen::Resume(saved) = std::mem::replace(&mut self.screen, ForgeScreen::Steps) {
            self.draft = *saved;
            self.enter_steps(window, cx);
        }
    }

    /// Drops the stored draft and shows the class picker.
    pub fn start_fresh(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(ForgeEvent::ClearAutosave);
        self.screen = ForgeScreen::Start;
        cx.notify();
    }

    pub(crate) fn enter_steps(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.screen = ForgeScreen::Steps;
        self.guide.fresh_step = true;
        self.revalidate();
        self.focus.focus(window, cx);
        cx.notify();
    }

    /// The host stored `profile`; the draft now matches it.
    pub fn committed(&mut self, bring_to_life: bool, cx: &mut Context<Self>) {
        self.draft.commit();
        self.context.existing.retain(|p| &p.id != self.draft.id());
        self.context.existing.push(self.draft.profile().clone());
        self.autosave.mark_saved();
        self.revalidate();
        if bring_to_life {
            self.screen = ForgeScreen::Celebrate;
        }
        cx.notify();
    }

    /// "Forge another" after a celebration: a fresh start screen of the same kind.
    pub fn forge_another(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_kind(self.draft.kind(), window, cx);
        self.screen = ForgeScreen::Start;
    }

    /// The host imported a photo for the draft.
    pub fn apply_avatar(&mut self, mode: AvatarMode, cx: &mut Context<Self>) {
        self.apply_edit(DraftEdit::Avatar(AvatarField::Avatar, Some(mode)), cx);
    }

    /// Shows a transient message (e.g. a failed photo import).
    pub fn set_notice(&mut self, notice: Notice, cx: &mut Context<Self>) {
        self.notice = Some(notice);
        cx.notify();
    }

    /// The finished, normalized profile as it would be stored.
    pub fn finished_profile(&self) -> Profile {
        self.draft.normalized()
    }
}
