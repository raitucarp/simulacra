//! Embodiment (agents and personas): flat ontology. A gallery of the eight kinds of being,
//! then the chosen kind's forms; picking either gives the body its natural parameters (one
//! undo step), which the body and stance cards below then let you adjust. People keep every
//! other page exactly as before, so a human only sees the kind gallery and a hint.

#[cfg(test)]
mod tests;

use super::super::super::common::{card, heading, labeled, muted};
use super::trait_block::chosen_line;
use crate::domain::traits::{BodyForm, BodyKind, Embodiment};
use crate::state::forge::{avatar_edits, body_edits, body_for_form, body_for_kind, roll_being, BeingScope, TraitField};
use crate::ui::features::forge::logic::time_seed;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::IllustratedOptionGrid;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;

const KIND_TITLE_KEY: &str = "forge.ui.embodiment.kind_title";
const FORM_TITLE_KEY: &str = "forge.ui.embodiment.form_title";
const HUMAN_HINT_KEY: &str = "forge.ui.embodiment.human_hint";
const BODY_TITLE_KEY: &str = "forge.ui.embodiment.body_title";
const STANCE_TITLE_KEY: &str = "forge.ui.embodiment.stance_title";
const ROLL_KEY: &str = "forge.ui.embodiment.roll";
const ROLL_HINT_KEY: &str = "forge.ui.embodiment.roll_hint";
const NATURAL_KEY: &str = "forge.ui.embodiment.natural";
const KIND_GRID_ID: &str = "forge-embodiment-kinds";
const FORM_GRID_ID: &str = "forge-embodiment-forms";
const ROLL_ID: &str = "forge-embodiment-roll";
const NATURAL_ID: &str = "forge-embodiment-natural";

/// What the body is: material, size, time scale, senses and motion.
pub(super) const BODY: [TraitField; 5] =
    [TraitField::Material, TraitField::Scale, TraitField::Temporality, TraitField::Perception, TraitField::Mobility];
/// How it stands towards people.
pub(super) const STANCE: [TraitField; 3] = [TraitField::Anthropomorphism, TraitField::RelationToHumans, TraitField::Withdrawal];

impl ForgeView {
    pub(in super::super) fn embodiment_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let body = self.draft.profile().config.embodiment;
        let page = div().flex().flex_col().gap(theme.spacing_lg).child(self.kind_card(body, handle));
        if body.is_human() {
            return page.into_any_element();
        }
        page.child(self.form_card(body, handle))
            .child(self.fields_card(BODY_TITLE_KEY, &BODY, handle))
            .child(self.fields_card(STANCE_TITLE_KEY, &STANCE, handle))
            .into_any_element()
    }

    /// The kind gallery, the dice and (for people) the hint that anything can post here.
    fn kind_card(&self, body: Embodiment, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let pick = handle.with(move |this: &mut ForgeView, kind: BodyKind, _, cx| {
            let current = this.draft.profile().config.embodiment;
            this.become_body(body_for_kind(current, kind), cx);
        });
        let grid = IllustratedOptionGrid::new(KIND_GRID_ID, Some(body.body_kind), theme, move |kind, w, cx| pick(kind, w, cx));
        let roll = handle.act(|this, _, cx| {
            let before = this.draft.profile().config.embodiment;
            if roll_being(&mut this.draft, time_seed(), BeingScope::AnyBeing) != before {
                this.after_change(cx);
            }
        });
        let dice = Button::new(ROLL_ID).outline().small().icon(LucideIcon::Dices).label(tr(ROLL_KEY)).on_click(move |_, w, cx| roll(w, cx));
        card(theme)
            .child(heading(tr(KIND_TITLE_KEY), theme))
            .child(labeled(TraitField::BodyKind.path(), grid, theme))
            .child(div().flex().items_center().gap(theme.spacing_sm).child(dice).child(muted(tr(ROLL_HINT_KEY), theme)))
            .children(body.is_human().then(|| muted(tr(HUMAN_HINT_KEY), theme)))
    }

    /// The forms of the chosen kind, what the chosen one is, and "reset to its nature".
    fn form_card(&self, body: Embodiment, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let pick = handle.with(move |this: &mut ForgeView, form: BodyForm, _, cx| {
            let current = this.draft.profile().config.embodiment;
            this.become_body(body_for_form(current, form), cx);
        });
        let grid = IllustratedOptionGrid::new(FORM_GRID_ID, Some(body.form), theme, move |form, w, cx| pick(form, w, cx))
            .options(body.body_kind.forms());
        let reset = handle.with(move |this: &mut ForgeView, (), _, cx| this.apply_edits(&body_edits(Embodiment::natural(body.form)), cx));
        let natural = Button::new(NATURAL_ID).ghost().small().icon(LucideIcon::Undo2).label(tr(NATURAL_KEY)).on_click(move |_, w, cx| reset((), w, cx));
        card(theme)
            .child(heading(tr(FORM_TITLE_KEY), theme))
            .child(labeled(TraitField::Form.path(), grid, theme))
            .children(chosen_line(TraitField::Form.read(&self.draft.profile().config), theme))
            .children(self.issue_lines(TraitField::Form.path()))
            .child(div().child(natural))
    }

    /// Turns the draft into `body` with a matching generated avatar, as one undo step.
    fn become_body(&mut self, body: Embodiment, cx: &mut Context<Self>) {
        let mut edits = body_edits(body);
        edits.extend(avatar_edits(body, self.draft.profile().config.avatar.as_ref(), time_seed()));
        self.apply_edits(&edits, cx);
    }

    /// A titled card of small trait grids.
    fn fields_card(&self, title_key: &str, fields: &[TraitField], handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let blocks: Vec<Div> = fields.iter().map(|&field| self.trait_block(field, handle)).collect();
        card(theme).gap(theme.spacing_lg).child(heading(tr(title_key), theme)).children(blocks)
    }
}
