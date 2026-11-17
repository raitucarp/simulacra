//! The generated-portrait editor: a chip per part, the chosen part's options as small
//! composed portraits (or color swatches for skin and hair), a 🎲 per part and for the whole
//! face.

use super::super::super::common::{card, heading};
use crate::domain::portrait::{slot_option_keys, AvatarMode, HairColor, PortraitSlot, PortraitSpec, SkinTone};
use crate::state::forge::{AvatarField, DraftEdit};
use crate::theme::color::parse_hex;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::{randomized_slot, slot_index, time_seed, with_slot_index};
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{focus_ring_shadow, ActionCallback, CardGrid, GridCell, OptionCardSpec};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::PortraitView;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const PARTS_KEY: &str = "forge.ui.appearance.parts";
const RANDOMIZE_KEY: &str = "portrait.randomize";
const RANDOMIZE_PART_KEY: &str = "portrait.randomize_part";
const ARG_PART: &str = "part";
const SLOT_ID_PREFIX: &str = "forge-slot";
const PART_ID_PREFIX: &str = "forge-part";
const SWATCH_ID_PREFIX: &str = "forge-swatch";
const RANDOMIZE_ID: &str = "forge-randomize-all";
const RANDOMIZE_PART_ID: &str = "forge-randomize-part";
const SWATCH_GRID_ID: &str = "forge-swatches";
const PART_GRID_ID: &str = "forge-parts";
/// Color swatches per row.
const SWATCH_COLUMNS: usize = 8;
/// Part thumbnails per row.
const PART_COLUMNS: usize = 4;

/// The colors a swatch slot offers, as `#rrggbb`.
fn swatch_hexes(slot: PortraitSlot) -> Vec<&'static str> {
    match slot {
        PortraitSlot::Skin => SkinTone::ALL.iter().map(|tone| tone.hex()).collect(),
        _ => HairColor::ALL.iter().map(|color| color.hex()).collect(),
    }
}

/// A round color swatch (shared by the portrait and form editors).
pub(super) fn swatch(id: usize, hex: &str, (selected, focused): (bool, bool), theme: &ThemeTokens, on_click: ActionCallback) -> Stateful<Div> {
    let fill = parse_hex(hex).unwrap_or(theme.text_muted);
    let ring = if focused { theme.focus_ring } else if selected { theme.accent_brand } else { theme.border };
    div()
        .id((SWATCH_ID_PREFIX, id))
        .size(theme.forge_swatch_size)
        .rounded(theme.radius_full)
        .bg(fill)
        .cursor_pointer()
        .border(if selected || focused { theme.border_width_threadline } else { theme.border_width_hairline })
        .border_color(ring)
        .when(focused, |dot| dot.shadow(focus_ring_shadow(theme)))
        .on_click(move |_, w, cx| on_click(w, cx))
}

impl ForgeView {
    pub(super) fn portrait_editor(&self, spec: PortraitSpec, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let slot = self.portrait_slot;
        let randomize = handle.act(|this, _, cx| this.set_portrait(PortraitSpec::random(time_seed()), cx));
        let reroll = handle.act(move |this, _, cx| this.set_portrait(randomized_slot(spec, slot, time_seed()), cx));
        let part_name = tr(&slot.label_key());
        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .child(heading(tr(PARTS_KEY), theme))
            .child(Button::new(RANDOMIZE_ID).outline().small().icon(LucideIcon::Dices).label(tr(RANDOMIZE_KEY)).on_click(move |_, w, cx| randomize(w, cx)));
        let reroll_button = Button::new(RANDOMIZE_PART_ID)
            .ghost()
            .small()
            .icon(LucideIcon::Dices)
            .label(tr_with(RANDOMIZE_PART_KEY, &[(ARG_PART, part_name)]))
            .on_click(move |_, w, cx| reroll(w, cx));
        card(theme).child(header).child(self.slot_chips(handle)).child(reroll_button).child(self.slot_options(spec, handle))
    }

    fn slot_chips(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let chips = PortraitSlot::ALL.iter().enumerate().map(|(at, &slot)| {
            let pick = handle.act(move |this, _, cx| {
                this.portrait_slot = slot;
                cx.notify();
            });
            let chip = Button::new((SLOT_ID_PREFIX, at)).xsmall().label(tr(&slot.label_key())).on_click(move |_, w, cx| pick(w, cx));
            if slot == self.portrait_slot { chip.primary() } else { chip.outline() }
        });
        div().flex().flex_wrap().gap(theme.spacing_xs).children(chips)
    }

    fn slot_options(&self, spec: PortraitSpec, handle: &ForgeHandle) -> CardGrid {
        let theme = &self.theme;
        let slot = self.portrait_slot;
        let chosen = slot_index(&spec, slot);
        let pick = |index: usize| -> ActionCallback { handle.act(move |this, _, cx| this.set_portrait(with_slot_index(spec, slot, index), cx)) };
        if slot.layer().is_none() {
            let cells = swatch_hexes(slot).into_iter().enumerate().map(|(at, hex)| {
                let swatch_theme = theme.clone();
                GridCell::new(at == chosen, move |focused, click| swatch(at, hex, (at == chosen, focused), &swatch_theme, click), pick(at))
            });
            return CardGrid::new((SWATCH_GRID_ID, slot as usize), cells.collect(), SWATCH_COLUMNS, theme.spacing_sm);
        }
        let width = theme.forge_compact_card_width;
        let keys = slot_option_keys(slot);
        let cells = keys.iter().enumerate().map(|(at, key)| {
            let art = PortraitView::new(AvatarMode::Pcg(with_slot_index(spec, slot, at)), width, theme);
            let mut card_spec = OptionCardSpec::new((PART_ID_PREFIX, at), art, tr(&slot.option_label_key(key)), width);
            card_spec.selected = at == chosen;
            GridCell::card(card_spec, theme, pick(at))
        });
        CardGrid::new((PART_GRID_ID, slot as usize), cells.collect(), PART_COLUMNS, theme.spacing_xs)
    }

    /// Replaces the generated portrait (one undo step).
    pub(super) fn set_portrait(&mut self, spec: PortraitSpec, cx: &mut Context<Self>) {
        self.apply_edit(DraftEdit::Avatar(AvatarField::Avatar, Some(AvatarMode::Pcg(spec))), cx);
    }
}
