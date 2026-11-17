//! Rhetoric (agents and personas): how they persuade (mode, plus any number of rhetorical
//! devices), the stylistic school and sentence structure, the semiotic mode and post format,
//! and the lexical fields they borrow vocabulary from. Multi-selects toggle card by card.

#[cfg(test)]
mod tests;

use super::super::super::common::{card, heading, muted};
use crate::state::forge::{TraitField, TraitSet};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{TraitGridLook, SMALL_COLUMNS};
use gpui_kit::*;

const PERSUASION_KEY: &str = "forge.ui.rhetoric.persuasion";
const DEVICES_HINT_KEY: &str = "forge.ui.rhetoric.devices_hint";
const STYLE_KEY: &str = "forge.ui.rhetoric.style";
const SIGNS_KEY: &str = "forge.ui.rhetoric.signs";
const WORDS_KEY: &str = "forge.ui.rhetoric.words";
const FIELDS_HINT_KEY: &str = "forge.ui.rhetoric.fields_hint";

/// Single-choice grids of the style and the signs cards, in page order.
pub(super) const STYLE: [TraitField; 2] = [TraitField::StylisticSchool, TraitField::SentenceStructure];
pub(super) const SIGNS: [TraitField; 2] = [TraitField::SemioticMode, TraitField::PostFormat];

impl ForgeView {
    pub(in super::super) fn rhetoric_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let cards = |title_key: &str, fields: &[TraitField]| {
            let blocks: Vec<Div> = fields.iter().map(|&field| self.trait_block(field, handle)).collect();
            card(theme).gap(theme.spacing_lg).child(heading(tr(title_key), theme)).children(blocks)
        };
        let look = (SMALL_COLUMNS, theme.forge_option_card_width);
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_lg)
            .child(
                card(theme)
                    .gap(theme.spacing_lg)
                    .child(heading(tr(PERSUASION_KEY), theme))
                    .child(self.trait_grid(TraitField::RhetoricalMode, TraitGridLook::Illustrated, handle))
                    .child(muted(tr(DEVICES_HINT_KEY), theme))
                    .child(self.trait_set_block(TraitSet::RhetoricalDevices, look, handle)),
            )
            .child(cards(STYLE_KEY, &STYLE))
            .child(cards(SIGNS_KEY, &SIGNS))
            .child(
                card(theme)
                    .child(heading(tr(WORDS_KEY), theme))
                    .child(muted(tr(FIELDS_HINT_KEY), theme))
                    .child(self.trait_set_block(TraitSet::LexicalFields, look, handle)),
            )
            .into_any_element()
    }
}
