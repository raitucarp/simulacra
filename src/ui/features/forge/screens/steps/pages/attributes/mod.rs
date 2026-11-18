//! Attributes (Skyrim-style): spend points on the Big Five and HEXACO honesty, optionally
//! within the "balanced build" budget; the stage radar follows every change.

mod budget;
mod panel;

#[cfg(test)]
mod tests;

use crate::state::forge::attributes::BUDGET_FIELDS;
use crate::state::forge::{DraftEdit, ScoreField};
use crate::ui::features::forge::text::field_label;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::state::forge::suggest::Param;
use gpui_kit::*;
use panel::PersonalityPanel;

impl ForgeView {
    pub(in crate::ui::features::forge::screens::steps) fn attributes_page(&self, handle: &ForgeHandle) -> AnyElement {
        let on_score = handle.edit(|(field, value): (ScoreField, u8)| DraftEdit::Score(field, value));
        let hints = BUDGET_FIELDS.iter().filter_map(|&field| Some((field, self.slider_hint(Param::Score(field), &field_label(field.path()), handle)?))).collect();
        PersonalityPanel { cfg: self.draft.profile().config.clone(), theme: self.theme.clone(), on_score, hints }.into_any_element()
    }
}
