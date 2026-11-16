//! Shared builders of the Voice, Daily Life, Relationships and Grimoire pages: a trait field
//! as a captioned grid of small illustrated cards with a "what the chosen option does" line,
//! and any control with its field's "?" explainer beside it.

use super::super::super::common::labeled;
use crate::domain::trait_option::TRAIT_DESCRIPTION_SUFFIX;
use crate::domain::TraitOption;
use crate::state::forge::{DraftEdit, TraitField, TraitValue, TraitVisitor};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{tr, tr_opt};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{ExplainerButton, IllustratedOptionGrid, ValueCallback};
use gpui_kit::*;

/// Element id prefix of these grids (`forge-pick-<path>`).
const GRID_ID_PREFIX: &str = "forge-pick";
/// Small cards per row.
const SMALL_COLUMNS: usize = 4;

/// Builds a small-card grid once the field's concrete option type is known.
struct SmallGrid {
    id: SharedString,
    theme: ThemeTokens,
    on_pick: ValueCallback<TraitValue>,
}

impl TraitVisitor for SmallGrid {
    type Output = AnyElement;

    fn visit<T: TraitOption>(self, current: T, wrap: fn(T) -> TraitValue) -> AnyElement {
        let on_pick = self.on_pick;
        IllustratedOptionGrid::new(self.id, Some(current), &self.theme, move |option: T, window, cx| on_pick(wrap(option), window, cx))
            .columns(SMALL_COLUMNS)
            .card_width(self.theme.forge_compact_card_width)
            .compact()
            .into_any_element()
    }
}

/// A tinted line naming the chosen option and how it changes their posts.
pub(super) fn chosen_line(value: TraitValue, theme: &ThemeTokens) -> Option<Div> {
    let description = tr_opt(&format!("{}.{TRAIT_DESCRIPTION_SUFFIX}", value.label_key()))?;
    Some(
        div()
            .p(theme.spacing_sm)
            .rounded(theme.radius_md)
            .bg(theme.bg_callout)
            .border_l(theme.border_width_threadline)
            .border_color(theme.accent_brand)
            .text_size(theme.font_size_xs)
            .line_height(relative(theme.line_height_body))
            .child(div().font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr(&value.label_key())))
            .child(div().text_color(theme.text_secondary).child(description)),
    )
}

/// `control` with the "?" explainer of field `path` at its top right.
pub(super) fn explained(path: &str, control: impl IntoElement, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_start()
        .gap(theme.spacing_xs)
        .child(div().flex_1().min_w_0().child(control))
        .children(ExplainerButton::for_field(path, theme))
}

impl ForgeView {
    /// The illustrated grid of `field` (small cards), its caption and the chosen-option line.
    pub(super) fn trait_block(&self, field: TraitField, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let value = field.read(&self.draft.profile().config);
        let id = SharedString::from(format!("{GRID_ID_PREFIX}-{}", field.path()));
        let grid = value.visit(SmallGrid { id, theme: theme.clone(), on_pick: handle.edit(DraftEdit::Trait) });
        labeled(field.path(), grid, theme).children(chosen_line(value, theme))
    }
}

#[cfg(test)]
mod tests;
