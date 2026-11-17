//! Birthsign: the MBTI type (sixteen cards in four temperament families) with its
//! Assertive / Turbulent identity, then the Enneagram (wheel, valid wings, instincts) and the
//! free-text tritype.

mod enneagram;
mod enneagram_view;
mod groups;
mod mbti;
mod wheel;

#[cfg(test)]
mod tests;

use crate::domain::IdentityVariant;
use crate::state::forge::{DraftEdit, TextField, TraitField, TraitValue};
use crate::ui::features::forge::screens::common::{card, heading, labeled};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::IllustratedOptionGrid;
use gpui_kit::*;

const TYPE_KEY: &str = "forge.ui.birthsign.type";
const ENNEAGRAM_KEY: &str = "forge.ui.birthsign.enneagram";
const IDENTITY_GRID_ID: &str = "forge-identity-variant";
/// Identity cards per row.
const IDENTITY_COLUMNS: usize = 2;

impl ForgeView {
    pub(in crate::ui::features::forge::screens::steps) fn birthsign_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        div()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(card(theme).child(heading(tr(TYPE_KEY), theme)).child(self.mbti_picker(handle)).child(self.identity_picker(handle)))
            .child(
                card(theme)
                    .child(heading(tr(ENNEAGRAM_KEY), theme))
                    .child(self.enneagram_picker(handle))
                    .child(self.instinct_picker(handle))
                    .child(self.text_row(TextField::Tritype, handle)),
            )
            .into_any_element()
    }

    fn identity_picker(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let current = self.draft.profile().config.identity_variant;
        let edit = handle.edit(|v: IdentityVariant| DraftEdit::Trait(TraitValue::IdentityVariant(v)));
        let grid = IllustratedOptionGrid::new(IDENTITY_GRID_ID, Some(current), theme, move |v, w, cx| edit(v, w, cx)).columns(IDENTITY_COLUMNS);
        labeled(TraitField::IdentityVariant.path(), grid, theme)
    }
}
