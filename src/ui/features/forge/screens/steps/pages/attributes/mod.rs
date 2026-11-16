//! Attributes (Skyrim-style): spend points on the Big Five and HEXACO honesty, optionally
//! within the "balanced build" budget; the stage radar follows every change.

mod budget;
mod panel;

#[cfg(test)]
mod tests;

use crate::state::forge::{DraftEdit, ScoreField};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use gpui_kit::*;
use panel::PersonalityPanel;

impl ForgeView {
    pub(in crate::ui::features::forge::screens::steps) fn attributes_page(&self, handle: &ForgeHandle) -> AnyElement {
        let on_score = handle.edit(|(field, value): (ScoreField, u8)| DraftEdit::Score(field, value));
        PersonalityPanel { cfg: self.draft.profile().config.clone(), theme: self.theme.clone(), on_score }.into_any_element()
    }
}
