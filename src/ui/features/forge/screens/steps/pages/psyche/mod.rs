//! Disposition (how they bond, what they value, their emotional weather) and Mind (how they
//! reason, plus the intelligence quotients as skills): illustrated option grids per trait.

mod disposition;
mod ladder;
mod mind;
mod quotients;

#[cfg(test)]
mod tests;

use crate::state::forge::{DraftEdit, TraitField};
use crate::ui::features::forge::screens::common::{card, heading, labeled};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::trait_option_grid;
use gpui_kit::*;

impl ForgeView {
    /// The illustrated cards (with descriptions) of one trait field, under its caption.
    fn trait_section(&self, field: TraitField, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let grid = trait_option_grid(field, &self.draft.profile().config, false, theme, handle.edit(DraftEdit::Trait));
        labeled(field.path(), grid, theme)
    }

    /// A titled card holding the option grids of `fields`.
    fn trait_card(&self, title_key: &str, fields: &[TraitField], handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let sections: Vec<Div> = fields.iter().map(|&field| self.trait_section(field, handle)).collect();
        card(theme).gap(theme.spacing_lg).child(heading(tr(title_key), theme)).children(sections)
    }
}
