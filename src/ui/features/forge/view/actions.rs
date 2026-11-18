//! Editing, undo / redo, navigation, autosave and saving the draft.

use super::keys::{Redo, Undo};
use super::{ForgeEvent, ForgeView};
use crate::state::forge::{validate_with, DraftEdit, ForgeKind, ForgeStep, ToggleField};
use crate::ui::features::forge::logic::save_bar_state;
use crate::ui::features::forge::widgets::close_explainers;
use gpui_kit::*;

impl ForgeView {
    /// Re-checks the draft and re-derives which visited steps are complete.
    pub(crate) fn revalidate(&mut self) {
        self.issues = validate_with(&self.draft, &self.context.existing, &self.context.credentials);
        self.draft.sync_completion(&self.issues);
    }

    /// Applies one edit as an undo step.
    pub fn apply_edit(&mut self, edit: DraftEdit, cx: &mut Context<Self>) {
        if self.draft.edit(edit) {
            self.after_change(cx);
        }
    }

    /// Applies several edits as one undo step.
    pub fn apply_edits(&mut self, edits: &[DraftEdit], cx: &mut Context<Self>) {
        if self.draft.edit_all(edits) {
            self.after_change(cx);
        }
    }

    /// Bookkeeping after any change to the working copy.
    pub(crate) fn after_change(&mut self, cx: &mut Context<Self>) {
        if self.notice.as_ref().is_some_and(|notice| notice.is_stale_for(self.draft.profile())) {
            self.notice = None;
        }
        self.revalidate();
        self.guide.fresh_step = false;
        if self.autosave.record_edit() {
            self.autosave_now(cx);
        }
        cx.notify();
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        if self.draft.undo() {
            self.after_change(cx);
        }
    }

    pub fn redo(&mut self, cx: &mut Context<Self>) {
        if self.draft.redo() {
            self.after_change(cx);
        }
    }

    pub(crate) fn on_undo(&mut self, _: &Undo, _: &mut Window, cx: &mut Context<Self>) {
        self.undo(cx);
    }

    pub(crate) fn on_redo(&mut self, _: &Redo, _: &mut Window, cx: &mut Context<Self>) {
        self.redo(cx);
    }

    /// Opens `step`, autosaving pending edits first.
    pub fn go_to(&mut self, step: ForgeStep, window: &mut Window, cx: &mut Context<Self>) {
        if step == self.draft.step() || !self.draft.go_to(step) {
            return;
        }
        self.entered_step(window, cx);
    }

    /// "Next": marks the step complete and opens the following one.
    pub fn next(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.draft.advance() {
            self.entered_step(window, cx);
        }
    }

    pub fn back(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.draft.back() {
            self.entered_step(window, cx);
        }
    }

    fn entered_step(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.autosave.on_step_change() {
            self.autosave_now(cx);
        }
        self.guide.fresh_step = true;
        self.notice = None;
        self.suggest.reset();
        self.guide.thinking = false;
        close_explainers(cx);
        self.revalidate();
        self.focus.focus(window, cx);
        cx.notify();
    }

    /// Hands the draft to the host for the settings table.
    pub(crate) fn autosave_now(&mut self, cx: &mut Context<Self>) {
        self.autosave.mark_saved();
        cx.emit(ForgeEvent::Autosave(Box::new(self.draft.clone())));
    }

    /// Throws away every unsaved change.
    pub fn discard(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.draft.discard();
        self.autosave.mark_saved();
        cx.emit(ForgeEvent::ClearAutosave);
        self.revalidate();
        cx.notify();
    }

    /// Saves the draft without leaving the forge (ignored while errors block saving).
    pub fn save(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if save_bar_state(true, &self.issues).can_save() {
            self.emit_commit(false, cx);
        }
    }

    /// "Bring to life": saves, switches agents on and celebrates.
    pub fn bring_to_life(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if !save_bar_state(true, &self.issues).can_save() {
            return;
        }
        if self.draft.kind() == ForgeKind::Agent {
            self.draft.edit(DraftEdit::Toggle(ToggleField::IsActive, true));
        }
        self.emit_commit(true, cx);
    }

    fn emit_commit(&mut self, bring_to_life: bool, cx: &mut Context<Self>) {
        let profile = Box::new(self.draft.normalized());
        let intro_post = bring_to_life && self.intro_post;
        cx.emit(ForgeEvent::Commit { profile, bring_to_life, intro_post });
    }

    /// Hides or shows Mnemo (persisted by the host).
    pub fn set_guide_hidden(&mut self, hidden: bool, _: &mut Window, cx: &mut Context<Self>) {
        self.guide.hidden = hidden;
        cx.emit(ForgeEvent::GuideHidden(hidden));
        cx.notify();
    }
}
