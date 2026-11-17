//! The non-human body editor: which being (kind gallery), then a chip per part, the chosen
//! part's options as small composed forms (or swatches for the two colors), a dice per part and
//! for the whole body. The kind gallery is temporary until the Embodiment step drives it.

use super::super::super::common::{card, heading, muted};
use crate::domain::portrait::{AvatarMode, FormKind, FormLayer, FormSlot, FormSpec};
use crate::state::forge::{AvatarField, DraftEdit};
use crate::ui::features::forge::logic::time_seed;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{ActionCallback, CardGrid, GridCell, OptionCardSpec};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::PortraitView;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;

const PARTS_KEY: &str = "forge.ui.appearance.parts";
const KIND_KEY: &str = "forms.kind";
const KIND_HINT_KEY: &str = "forms.kind_hint";
const RANDOMIZE_KEY: &str = "portrait.randomize";
const RANDOMIZE_PART_KEY: &str = "portrait.randomize_part";
const ARG_PART: &str = "part";
const KIND_ID_PREFIX: &str = "forge-form-kind";
const KIND_GRID_ID: &str = "forge-form-kinds";
const SLOT_ID_PREFIX: &str = "forge-form-slot";
const RANDOMIZE_ID: &str = "forge-form-randomize-all";
const RANDOMIZE_PART_ID: &str = "forge-form-randomize-part";
/// Kind cards per row until the grid's width is first measured.
const KIND_COLUMNS: usize = 4;

impl ForgeView {
    pub(super) fn form_editor(&self, spec: &FormSpec, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let (kind, slot) = (spec.kind, self.form_slot);
        let randomize = handle.act(move |this, _, cx| this.set_form(FormSpec::random_any(kind, time_seed()), cx));
        let current = spec.clone();
        let reroll = handle.act(move |this, _, cx| this.set_form(current.rerolled(slot, time_seed()), cx));
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
            .label(tr_with(RANDOMIZE_PART_KEY, &[(ARG_PART, tr(&slot.label_key()))]))
            .on_click(move |_, w, cx| reroll(w, cx));
        let parts = card(theme).child(header).child(self.form_slot_chips(handle)).child(reroll_button).child(self.form_options(spec, handle));
        div().flex().flex_col().gap(theme.section_gap).child(self.kind_picker(kind, handle)).child(parts)
    }

    /// "Which being": one card per kind showing its first form.
    fn kind_picker(&self, current: FormKind, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        // Preset-sized cards keep one-word kind names ("Abstraction") on one line.
        let width = theme.forge_preset_card_width;
        let cells = FormKind::ALL.iter().enumerate().map(|(at, &kind)| {
            let first = FormSpec::new(kind, "").catalog().default_variant(FormLayer::Body);
            let art = PortraitView::new(AvatarMode::Form(FormSpec::new(kind, first)), width, theme);
            let mut card_spec = OptionCardSpec::new((KIND_ID_PREFIX, at), art, tr(&kind.label_key()), width);
            card_spec.selected = kind == current;
            let pick: ActionCallback = handle.act(move |this, _, cx| {
                if this.current_form().is_some_and(|spec| spec.kind != kind) {
                    this.set_form(FormSpec::random_any(kind, time_seed()), cx);
                }
            });
            GridCell::card(card_spec, theme, pick)
        });
        card(theme)
            .child(heading(tr(KIND_KEY), theme))
            .child(muted(tr(KIND_HINT_KEY), theme))
            .child(CardGrid::new(KIND_GRID_ID, cells.collect(), KIND_COLUMNS, theme.spacing_xs).auto_columns(width))
    }

    fn form_slot_chips(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let chips = FormSlot::ALL.iter().enumerate().map(|(at, &slot)| {
            let pick = handle.act(move |this, _, cx| {
                this.form_slot = slot;
                cx.notify();
            });
            let chip = Button::new((SLOT_ID_PREFIX, at)).xsmall().label(tr(&slot.label_key())).on_click(move |_, w, cx| pick(w, cx));
            if slot == self.form_slot { chip.primary() } else { chip.outline() }
        });
        div().flex().flex_wrap().gap(theme.spacing_xs).children(chips)
    }

    /// The draft's form, when its avatar is one.
    fn current_form(&self) -> Option<FormSpec> {
        match &self.draft.profile().config.avatar {
            Some(AvatarMode::Form(spec)) => Some(spec.clone()),
            _ => None,
        }
    }

    /// Replaces the form (one undo step).
    pub(super) fn set_form(&mut self, spec: FormSpec, cx: &mut Context<Self>) {
        self.apply_edit(DraftEdit::Avatar(AvatarField::Avatar, Some(AvatarMode::Form(spec))), cx);
    }
}
