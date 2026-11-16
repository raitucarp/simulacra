//! Opening the forge, the start screen's ways into the creator, and what happens after the
//! host saved the profile.

use super::state::{ForgeContext, ForgeOpen, ForgeScreen, GuideState, Notice, StartState};
use super::{ForgeEvent, ForgeView};
use crate::domain::portrait::{AvatarMode, PortraitSlot};
use crate::domain::{ArchetypeClass, Profile};
use crate::state::forge::{apply_class, roll_into, AgentDraft, AvatarField, DraftEdit, ForgeKind};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::{time_seed, AutosavePolicy};
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
            class: None,
            intro_post: true,
            notice: None,
            portrait_slot: PortraitSlot::Face,
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

    /// Switches the start screen between a new agent and a new persona.
    pub fn set_kind(&mut self, kind: ForgeKind, _: &mut Window, cx: &mut Context<Self>) {
        self.start = StartState { kind, ..StartState::new(kind) };
        self.draft = AgentDraft::new(kind, self.context.network_id.clone());
        self.class = None;
        self.revalidate();
        cx.notify();
    }

    /// "Begin as <class>": applies the class and opens the creator.
    pub fn begin_as(&mut self, class: ArchetypeClass, window: &mut Window, cx: &mut Context<Self>) {
        apply_class(&mut self.draft, class);
        self.class = Some(class);
        self.enter_steps(window, cx);
    }

    /// "Start from scratch".
    pub fn begin_blank(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.class = None;
        self.enter_steps(window, cx);
    }

    /// Rolls (or re-rolls) a random thinker and shows the result on the start screen.
    pub fn roll(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let class = roll_into(&mut self.draft, time_seed(), &self.context.existing);
        self.class = Some(class);
        self.start.rolled = Some(class);
        self.start.describing = false;
        self.revalidate();
        cx.notify();
    }

    /// Keeps the rolled thinker and opens the creator.
    pub fn keep_roll(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.start.rolled = None;
        self.enter_steps(window, cx);
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
