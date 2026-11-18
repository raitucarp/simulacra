//! Shared builders of the option pages: a trait field as a captioned illustrated grid (full
//! cards on Disposition / Mind, small cards with a "what the chosen option does" line on Voice,
//! Daily Life, Relationships and Grimoire), and a switch with its "?", "Why?" and "Suggest".

use crate::domain::trait_option::TRAIT_DESCRIPTION_SUFFIX;
use crate::state::forge::{DraftEdit, TraitField, TraitSet, TraitValue};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{set_labels, tr_count, tr_trait};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{trait_option_grid, trait_set_grid, ExplainerButton, TraitGridLook};
use crate::state::forge::suggest::Param;
use gpui_kit::*;

/// Element id prefix of the trait grids (`forge-trait-<path>`).
const GRID_ID_PREFIX: &str = "forge-trait";
/// "N selected" under a multi-select grid.
const SELECTED_KEY: &str = "forge.ui.multi.selected";
const SUMMARY_SEPARATOR: &str = " · ";

/// A tinted line naming the chosen option and how it changes their posts.
pub(super) fn chosen_line(value: TraitValue, theme: &ThemeTokens) -> Option<Div> {
    let label_key = value.label_key();
    let description = tr_trait(&format!("{label_key}.{TRAIT_DESCRIPTION_SUFFIX}"))?;
    let label = tr_trait(&label_key).unwrap_or(label_key);
    Some(
        div()
            .p(theme.spacing_sm)
            .rounded(theme.radius_md)
            .bg(theme.bg_callout)
            .border_l(theme.border_width_threadline)
            .border_color(theme.accent_brand)
            .text_size(theme.font_size_xs)
            .line_height(relative(theme.line_height_body))
            .child(div().font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(label))
            .child(div().text_color(theme.text_secondary).child(description)),
    )
}

impl ForgeView {
    /// The option grid of `field` drawn as `look`, under its caption.
    pub(in super::super) fn trait_grid(&self, field: TraitField, look: TraitGridLook, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let id = SharedString::from(format!("{GRID_ID_PREFIX}-{}", field.path()));
        let param = Param::Trait(field);
        let grid = trait_option_grid((id, field), &self.draft.profile().config, look, (theme, self.picks(param)), handle.edit(DraftEdit::Trait));
        self.hinted(param, grid, handle)
    }

    /// A switch with its "?" explainer, "Why?" and "Suggest" beside it, and any AI value below.
    pub(super) fn hinted_toggle(&self, param: Param, control: impl IntoElement, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let row = div()
            .flex()
            .items_start()
            .gap(theme.spacing_xs)
            .child(div().flex_1().min_w_0().child(control))
            .children(ExplainerButton::for_field(param.path(), theme))
            .child(self.hint_actions(param, handle));
        div().flex().flex_col().gap(theme.spacing_xs).child(row).children(self.ai_chips(param, handle))
    }

    /// The small-card grid of `field`, its caption and the chosen-option line.
    pub(super) fn trait_block(&self, field: TraitField, handle: &ForgeHandle) -> Div {
        let value = field.read(&self.draft.profile().config);
        self.trait_grid(field, TraitGridLook::Small, handle).children(chosen_line(value, &self.theme))
    }

    /// The toggleable card grid of multi-select `field` (`columns` cards of `width` per row)
    /// under its caption, with what is chosen summed up underneath.
    pub(super) fn trait_set_block(&self, field: TraitSet, (columns, width): (usize, Pixels), handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let cfg = &self.draft.profile().config;
        let id = SharedString::from(format!("{GRID_ID_PREFIX}-{}", field.path()));
        let param = Param::Set(field);
        let grid = trait_set_grid((id, field), cfg, (columns, width), (theme, self.picks(param)), handle.edit(DraftEdit::TraitSet));
        let value = field.read(cfg);
        let count = value.keys().len();
        let summary = match count {
            0 => set_labels(&value),
            _ => format!("{}{SUMMARY_SEPARATOR}{}", tr_count(SELECTED_KEY, count, &[]), set_labels(&value)),
        };
        let line = div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(summary);
        self.hinted(param, grid, handle).child(line).children(self.issue_lines(field.path()))
    }
}

#[cfg(test)]
mod tests;
