//! The MBTI picker: sixteen illustrated type cards in four temperament families, then a
//! callout describing the chosen type and its cognitive-function stack.

use super::groups::MbtiGroup;
use crate::domain::MbtiType;
use crate::state::forge::{DraftEdit, TraitValue};
use crate::ui::features::forge::screens::common::labeled;
use crate::ui::features::forge::text::{tr, trait_description, trait_label};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{CardGrid, GridCell, OptionCardSpec};
use crate::ui::illustration::Illustration;
use crate::state::forge::TraitField;
use gpui_kit::*;

const CARD_ID_PREFIX: &str = "forge-mbti";
const GRID_ID: &str = "forge-mbti-grid";
/// Type cards per row: one temperament family each.
const FAMILY_SIZE: usize = 4;

impl ForgeView {
    /// Every family with its cards, and the chosen type's callout.
    pub(super) fn mbti_picker(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let chosen = self.draft.profile().config.mbti;
        let headers = MbtiGroup::ALL.into_iter().map(|group| self.mbti_family(group).into_any_element()).collect();
        let cells = MbtiGroup::ALL.into_iter().flat_map(|group| group.members()).map(|ty| self.mbti_card(ty, ty == chosen, handle)).collect();
        let grid = CardGrid::new(GRID_ID, cells, FAMILY_SIZE, theme.spacing_sm).row_headers(headers);
        let body = div().flex().flex_col().gap(theme.spacing_md).child(grid).child(self.mbti_callout(chosen));
        labeled(TraitField::Mbti.path(), body, theme)
    }

    /// The family's name and blurb, above its row of cards.
    fn mbti_family(&self, group: MbtiGroup) -> Div {
        let theme = &self.theme;
        div()
            .flex()
            .items_baseline()
            .gap(theme.spacing_xs)
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr(&group.label_key())))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr(&group.blurb_key())))
    }

    fn mbti_card(&self, ty: MbtiType, selected: bool, handle: &ForgeHandle) -> GridCell {
        let width = self.theme.forge_compact_card_width;
        let art = Illustration::trait_card(ty).width(width);
        let mut spec = OptionCardSpec::new((CARD_ID_PREFIX, ty as usize), art, ty.code(), width);
        spec.selected = selected;
        let pick = handle.edit(|ty: MbtiType| DraftEdit::Trait(TraitValue::Mbti(ty)));
        GridCell::card(spec, &self.theme, std::rc::Rc::new(move |w, cx| pick(ty, w, cx)))
    }

    /// The chosen type's name, what it does to their posts and its function stack.
    fn mbti_callout(&self, chosen: MbtiType) -> Div {
        let theme = &self.theme;
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_2xs)
            .p(theme.spacing_md)
            .rounded(theme.radius_lg)
            .bg(theme.bg_callout)
            .border_l(theme.border_width_threadline)
            .border_color(theme.accent_brand)
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.accent_brand).child(trait_label(chosen)))
            .children(trait_description(chosen).map(|text| {
                div().text_size(theme.font_size_xs).text_color(theme.text_primary).line_height(relative(theme.line_height_body)).child(text)
            }))
            .child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(theme.text_muted).child(chosen.cognitive_functions()))
    }
}
