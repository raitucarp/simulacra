//! The start screen's ways into the creator: kind switch, palette, blank sheet and the dice,
//! plus the step palettes offered on each step.

use super::state::StartState;
use super::ForgeView;
use crate::domain::portrait::{suggested_outfit, AvatarMode, PortraitSpec};
use super::state::Notice;
use crate::domain::Palette;
use crate::state::forge::{apply_palette, clear_palette, roll_into, AgentDraft, AvatarField, DraftEdit, ForgeKind};
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::logic::time_seed;
use gpui_kit::*;

const APPLIED_KEY: &str = "forge.palette.ui.applied";
const ARG_PALETTE: &str = "palette";

impl ForgeView {
    /// Switches the start screen between a new agent and a new persona.
    pub fn set_kind(&mut self, kind: ForgeKind, _: &mut Window, cx: &mut Context<Self>) {
        self.start = StartState { kind, ..StartState::new(kind) };
        self.draft = AgentDraft::new(kind, self.context.network_id.clone());
        self.revalidate();
        cx.notify();
    }

    /// "Begin as <palette>": applies the full palette and opens the creator.
    pub fn begin_as(&mut self, palette: &'static Palette, window: &mut Window, cx: &mut Context<Self>) {
        apply_palette(&mut self.draft, palette);
        self.ensure_face();
        self.enter_steps(window, cx);
    }

    /// "Start from scratch".
    pub fn begin_blank(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        clear_palette(&mut self.draft);
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

    /// Applies a step palette from the strip (one undo step) and says how to take it back.
    pub fn apply_step_palette(&mut self, palette: &'static Palette, _: &mut Window, cx: &mut Context<Self>) {
        if apply_palette(&mut self.draft, palette) {
            self.notice = Some(Notice::info(tr_with(APPLIED_KEY, &[(ARG_PALETTE, tr(&palette.label_key()))]), Vec::new()));
            self.after_change(cx);
        }
    }

    /// Shows (or, with `None`, hides) the changes of a strip palette.
    pub fn preview_step_palette(&mut self, palette: Option<&'static Palette>, cx: &mut Context<Self>) {
        if self.strip_preview.map(|p| &p.key) != palette.map(|p| &p.key) {
            self.strip_preview = palette;
            cx.notify();
        }
    }
}
