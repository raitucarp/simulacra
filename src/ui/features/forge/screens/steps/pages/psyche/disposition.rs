//! Disposition: attachment and locus of control (how they bond and act), moral foundation and
//! core values (what matters), temperament, humor and current mood (emotional weather).

use crate::state::forge::{TextField, TraitField};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use gpui_kit::*;

const BONDS_KEY: &str = "forge.ui.disposition.bonds";
const VALUES_KEY: &str = "forge.ui.disposition.values";
const WEATHER_KEY: &str = "forge.ui.disposition.weather";

/// Trait grids of each Disposition card, in page order.
pub(super) const BONDS: [TraitField; 2] = [TraitField::AttachmentStyle, TraitField::LocusOfControl];
pub(super) const VALUES: [TraitField; 1] = [TraitField::MoralFoundation];
pub(super) const WEATHER: [TraitField; 2] = [TraitField::Temperament, TraitField::HumorStyle];
/// Free-text fields of the values and weather cards.
pub(super) const VALUES_TEXT: TextField = TextField::CoreValues;
pub(super) const WEATHER_TEXT: TextField = TextField::CurrentMood;

impl ForgeView {
    pub(in crate::ui::features::forge::screens::steps) fn disposition_page(&self, handle: &ForgeHandle) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(self.theme.section_gap)
            .child(self.trait_card(BONDS_KEY, &BONDS, handle))
            .child(self.trait_card(VALUES_KEY, &VALUES, handle).child(self.text_row(VALUES_TEXT, handle)))
            .child(self.trait_card(WEATHER_KEY, &WEATHER, handle).child(self.text_row(WEATHER_TEXT, handle)))
            .into_any_element()
    }
}
