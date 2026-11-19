//! Editing one belief: its wording in a field and its credence stepped by 0.05, saved as a
//! revision by the user (or cancelled).

use super::super::events::BeliefsEvent;
use super::super::{BeliefEdit, BeliefsPanel};
use super::text::{credence_text, tr};
use crate::domain::beliefs::{BeliefAction, BeliefId};
use crate::ui::features::arena::kit::{control_row, ghost_button, line_field, primary_button};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

/// Percent points one step moves the credence.
const STEP_POINTS: i32 = 5;
const LESS_ID: &str = "belief-edit-less";
const MORE_ID: &str = "belief-edit-more";
const SAVE_ID: &str = "belief-edit-save";
const CANCEL_ID: &str = "belief-edit-cancel";

impl BeliefsPanel {
    /// Opens the editor on `edit`, its field holding `proposition`.
    pub(crate) fn start_edit(&mut self, edit: BeliefEdit, proposition: String, window: &mut Window, cx: &mut Context<Self>) {
        self.wording.update(cx, |field, cx| field.set_value(proposition, window, cx));
        self.editing = Some(edit);
        cx.notify();
    }

    pub(crate) fn toggle_history(&mut self, id: BeliefId, cx: &mut Context<Self>) {
        self.expanded = if self.expanded.as_ref() == Some(&id) { None } else { Some(id) };
        cx.notify();
    }

    fn step(&mut self, delta: i32, cx: &mut Context<Self>) {
        if let Some(edit) = self.editing.as_mut() {
            edit.credence = edit.credence.nudged(delta);
        }
        cx.notify();
    }

    fn save_edit(&mut self, cx: &mut Context<Self>) {
        let Some(edit) = self.editing.take() else { return };
        let proposition = self.wording.read(cx).value().to_string();
        cx.emit(BeliefsEvent::Act(BeliefAction::Edit { id: edit.id, proposition, credence: edit.credence }));
        cx.notify();
    }

    pub(super) fn render_edit(&self, edit: &BeliefEdit, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        // Kit buttons, like Save and Cancel beside them: they claim their press, so the first
        // click after the editor opens registers (a plain div stepper here missed it).
        let step = |id: &'static str, icon: LucideIcon, delta: i32| {
            let entity = cx.entity().downgrade();
            Button::new(id).ghost().xsmall().icon(Icon::new(icon)).on_click(move |_, _, cx| {
                let _ = entity.update(cx, |panel, cx| panel.step(delta, cx));
            })
        };
        let (save_entity, cancel_entity) = (cx.entity().downgrade(), cx.entity().downgrade());
        let value = div().min_w(theme.key_cap_min_width).text_center().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).child(credence_text(edit.credence));
        let save = primary_button(SAVE_ID, LucideIcon::Check, tr("beliefs.save")).on_click(move |_, _, cx| {
            let _ = save_entity.update(cx, |panel, cx| panel.save_edit(cx));
        });
        let cancel = ghost_button(CANCEL_ID, LucideIcon::X, tr("beliefs.cancel")).on_click(move |_, _, cx| {
            let _ = cancel_entity.update(cx, |panel, cx| {
                panel.editing = None;
                cx.notify();
            });
        });
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(line_field(&self.wording, theme))
            .child(control_row(theme).child(step(LESS_ID, LucideIcon::Minus, -STEP_POINTS)).child(value).child(step(MORE_ID, LucideIcon::Plus, STEP_POINTS)))
            .child(control_row(theme).child(save).child(cancel))
    }
}
