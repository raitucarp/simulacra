//! The options of the chosen form part: composed thumbnails per variant, or color swatches.

use super::portrait::swatch;
use crate::domain::portrait::{AvatarMode, FormColor, FormLayer, FormSlot, FormSpec};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{ActionCallback, CardGrid, GridCell, OptionCardSpec};
use crate::ui::illustration::PortraitView;

const PART_ID_PREFIX: &str = "forge-form-part";
const SWATCH_GRID_ID: &str = "forge-form-swatches";
const PART_GRID_ID: &str = "forge-form-parts";
/// Color swatches per row.
const SWATCH_COLUMNS: usize = 8;
/// Part thumbnails per row.
const PART_COLUMNS: usize = 4;

/// Stable grid-id suffix of a slot (its position in the editor order).
fn slot_ordinal(slot: FormSlot) -> usize {
    FormSlot::ALL.iter().position(|s| *s == slot).unwrap_or_default()
}

impl ForgeView {
    pub(super) fn form_options(&self, spec: &FormSpec, handle: &ForgeHandle) -> CardGrid {
        match self.form_slot {
            FormSlot::Layer(layer) => self.form_part_grid(spec, layer, handle),
            FormSlot::Color(color) => self.form_swatch_grid(spec, color, handle),
        }
    }

    /// A callback choosing option `index` of the current slot.
    fn form_pick(&self, spec: &FormSpec, index: usize, handle: &ForgeHandle) -> ActionCallback {
        let next = spec.clone().with_slot_index(self.form_slot, index);
        handle.act(move |this, _, cx| this.set_form(next.clone(), cx))
    }

    fn form_part_grid(&self, spec: &FormSpec, layer: FormLayer, handle: &ForgeHandle) -> CardGrid {
        let theme = &self.theme;
        let slot = self.form_slot;
        let chosen = spec.slot_index(slot);
        let width = theme.forge_compact_card_width;
        let cells = spec.slot_keys(slot).into_iter().enumerate().map(|(at, key)| {
            let art = PortraitView::new(AvatarMode::Form(spec.clone().with_slot_index(slot, at)), width, theme);
            let mut card_spec = OptionCardSpec::new((PART_ID_PREFIX, at), art, tr(&layer.variant_label_key(key)), width);
            card_spec.selected = at == chosen;
            GridCell::card(card_spec, theme, self.form_pick(spec, at, handle))
        });
        CardGrid::new((PART_GRID_ID, slot_ordinal(slot)), cells.collect(), PART_COLUMNS, theme.spacing_xs)
    }

    fn form_swatch_grid(&self, spec: &FormSpec, color: FormColor, handle: &ForgeHandle) -> CardGrid {
        let theme = &self.theme;
        let chosen = spec.swatch_index(color);
        let cells = spec.swatches(color).iter().enumerate().map(|(at, sw)| {
            let (hex, swatch_theme) = (sw.hex.clone(), theme.clone());
            let build = move |focused, click| swatch(at, &hex, (at == chosen, focused), &swatch_theme, click);
            GridCell::new(at == chosen, build, self.form_pick(spec, at, handle))
        });
        CardGrid::new((SWATCH_GRID_ID, slot_ordinal(self.form_slot)), cells.collect(), SWATCH_COLUMNS, theme.spacing_sm)
    }
}
