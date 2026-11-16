//! The start screen's ways into the creator: kind switch, class, blank sheet and the dice.

use super::state::StartState;
use super::ForgeView;
use crate::domain::portrait::{suggested_outfit, AvatarMode, PortraitSpec};
use crate::domain::ArchetypeClass;
use crate::state::forge::{apply_class, roll_into, AgentDraft, AvatarField, DraftEdit, ForgeKind};
use crate::ui::features::forge::logic::time_seed;
use gpui_kit::*;

impl ForgeView {
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
        self.ensure_face();
        self.enter_steps(window, cx);
    }

    /// "Start from scratch".
    pub fn begin_blank(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.class = None;
        self.ensure_face();
        self.enter_steps(window, cx);
    }

    /// Gives a faceless draft a random generated portrait dressed for its occupation.
    pub(crate) fn ensure_face(&mut self) {
        let cfg = &self.draft.profile().config;
        if cfg.avatar.is_some() {
            return;
        }
        let spec = PortraitSpec::random(time_seed());
        let dressed = PortraitSpec { outfit: suggested_outfit(&cfg.occupation).unwrap_or(spec.outfit), ..spec };
        self.draft.edit(DraftEdit::Avatar(AvatarField::Avatar, Some(AvatarMode::Pcg(dressed))));
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
}
