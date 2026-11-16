//! The Enneagram picker: the wheel of core types, the two wings valid for the chosen core as
//! illustrated cards, and the instinctual stacking as illustrated cards.

use super::enneagram::{core, variants, with_core};
use super::wheel::enneagram_wheel;
use crate::domain::{EnneagramType, InstinctualVariant};
use crate::state::forge::{DraftEdit, TraitField, TraitValue};
use crate::ui::features::forge::screens::common::labeled;
use crate::ui::features::forge::text::{tr, trait_description, trait_label};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{CardGrid, GridCell, IllustratedOptionGrid, OptionCardSpec};
use crate::ui::illustration::Illustration;
use gpui_kit::*;

const WHEEL_HINT_KEY: &str = "forge.ui.birthsign.wheel_hint";
const WING_KEY: &str = "forge.ui.birthsign.wing";
const WING_ID_PREFIX: &str = "forge-enneagram-wing";
const WING_GRID_ID: &str = "forge-enneagram-wings";
const INSTINCT_GRID_ID: &str = "forge-instinct";
/// Instinct cards per row.
const INSTINCT_COLUMNS: usize = 3;

fn caption(text: String, theme: &crate::theme::ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).text_color(theme.text_muted).line_height(relative(theme.line_height_body)).child(text)
}

impl ForgeView {
    /// The wheel above the wing cards of the chosen core.
    pub(super) fn enneagram_picker(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let current = self.draft.profile().config.enneagram;
        let pick_core = handle.with(|this, point: u8, _, cx| {
            let moved = with_core(this.draft.profile().config.enneagram, point);
            this.apply_edit(DraftEdit::Trait(TraitValue::Enneagram(moved)), cx);
        });
        let wheel = div().flex().justify_center().child(enneagram_wheel(core(current), theme, pick_core));
        let body = div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(caption(tr(WHEEL_HINT_KEY), theme))
            .child(wheel)
            .child(caption(tr(WING_KEY), theme))
            .child(self.wing_cards(current, handle));
        labeled(TraitField::Enneagram.path(), body, theme)
    }

    /// One illustrated card per valid wing of `current`'s core.
    fn wing_cards(&self, current: EnneagramType, handle: &ForgeHandle) -> CardGrid {
        let theme = &self.theme;
        let width = theme.forge_option_card_width;
        let wings = variants(core(current));
        let columns = wings.len();
        let cells = wings.into_iter().map(|variant| {
            let art = Illustration::trait_card(variant).width(width);
            let mut spec = OptionCardSpec::new((WING_ID_PREFIX, variant as usize), art, trait_label(variant), width);
            spec.description = trait_description(variant);
            spec.selected = variant == current;
            let pick = handle.edit(|ty: EnneagramType| DraftEdit::Trait(TraitValue::Enneagram(ty)));
            GridCell::card(spec, theme, std::rc::Rc::new(move |w, cx| pick(variant, w, cx)))
        });
        CardGrid::new(WING_GRID_ID, cells.collect(), columns, theme.spacing_sm).centered()
    }

    /// Self-preservation / social / intimate stacking as illustrated cards.
    pub(super) fn instinct_picker(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let current = self.draft.profile().config.instinctual_variant;
        let edit = handle.edit(|v: InstinctualVariant| DraftEdit::Trait(TraitValue::InstinctualVariant(v)));
        let grid = IllustratedOptionGrid::new(INSTINCT_GRID_ID, Some(current), theme, move |v, w, cx| edit(v, w, cx)).columns(INSTINCT_COLUMNS);
        labeled(TraitField::InstinctualVariant.path(), grid, theme)
    }
}
