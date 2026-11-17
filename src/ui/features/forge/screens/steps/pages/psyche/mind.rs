//! Mind: cognitive style, epistemic stance and tempo as illustrated grids, then the five
//! intelligence quotients as skills.

use crate::state::forge::TraitField;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use gpui_kit::*;

const REASONING_KEY: &str = "forge.ui.mind.reasoning";

/// Trait grids of the reasoning card, in page order.
pub(super) const REASONING: [TraitField; 3] = [TraitField::IntelligenceStyle, TraitField::EpistemicStance, TraitField::CognitivePace];

impl ForgeView {
    pub(in crate::ui::features::forge::screens::steps) fn mind_page(&self, handle: &ForgeHandle) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(self.theme.section_gap)
            .child(self.trait_card(REASONING_KEY, &REASONING, handle))
            .child(self.quotient_skills(handle))
            .into_any_element()
    }
}
