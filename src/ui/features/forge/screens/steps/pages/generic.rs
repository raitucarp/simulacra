//! The interim page of the steps that do not have a bespoke illustrated page yet: every
//! field of the step rendered with the generic forge widgets, so the creator is complete
//! end to end. Steps with their own page replace this one in `options::step_page`.

use super::super::super::common::{card, labeled, muted};
use crate::state::forge::{
    fields_for_step, DraftEdit, ForgeStep, HourField, ScoreField, TextField, ToggleField, TraitField,
};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{trait_option_grid, HourRangePicker, ScoreSlider, ToggleCard};
use gpui_kit::*;

const PLACEHOLDER_KEY: &str = "forge.ui.placeholder";

impl ForgeView {
    pub(in super::super) fn generic_page(&self, step: ForgeStep, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let controls: Vec<AnyElement> = fields_for_step(step).into_iter().filter_map(|path| self.field_control(path, handle)).collect();
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_lg)
            .child(muted(tr(PLACEHOLDER_KEY), theme))
            .child(card(theme).gap(theme.spacing_lg).children(controls))
            .into_any_element()
    }

    /// The generic control for the field with serde name `path`, or `None` for fields that
    /// have no stand-alone control (hour ends share one picker; avatar fields live on
    /// Appearance).
    pub(super) fn field_control(&self, path: &str, handle: &ForgeHandle) -> Option<AnyElement> {
        let theme = &self.theme;
        let profile = self.draft.profile();
        if let Some(field) = TextField::ALL.iter().copied().find(|f| f.path() == path) {
            return Some(self.text_row(field, handle).into_any_element());
        }
        if let Some(field) = ScoreField::ALL.iter().copied().find(|f| f.path() == path) {
            let edit = handle.edit(move |value: u8| DraftEdit::Score(field, value));
            return Some(ScoreSlider::new(field, &profile.config, theme, move |v, w, cx| edit(v, w, cx)).into_any_element());
        }
        if path == HourField::ActiveHoursStart.path() {
            let edit = handle.edit(|(field, hour): (HourField, u8)| DraftEdit::Hour(field, hour));
            let picker = HourRangePicker::new(&profile.config, theme, move |change, w, cx| edit(change, w, cx));
            return Some(labeled(path, picker, theme).children(self.issue_lines(path)).into_any_element());
        }
        if let Some(field) = ToggleField::ALL.iter().copied().find(|f| f.path() == path) {
            let edit = handle.edit(move |on: bool| DraftEdit::Toggle(field, on));
            return Some(ToggleCard::new(field, *field.get(profile), theme, move |on, w, cx| edit(on, w, cx)).into_any_element());
        }
        if let Some(field) = TraitField::from_path(path) {
            let grid = trait_option_grid(field, &profile.config, true, theme, handle.edit(DraftEdit::Trait));
            return Some(labeled(path, grid, theme).into_any_element());
        }
        None
    }
}
