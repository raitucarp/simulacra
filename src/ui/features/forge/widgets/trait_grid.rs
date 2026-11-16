//! [`trait_option_grid`]: an [`IllustratedOptionGrid`] for a trait field chosen at runtime,
//! dispatched to the field's concrete option type through [`TraitVisitor`].

use super::option_grid::IllustratedOptionGrid;
use super::ValueCallback;
use crate::domain::agent_config::AgentConfig;
use crate::domain::TraitOption;
use crate::state::forge::{TraitField, TraitValue, TraitVisitor};
use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Cards per row of a [`TraitGridLook::Small`] grid: three option-card widths fill the
/// options panel, leaving labels room for whole words.
pub const SMALL_COLUMNS: usize = 3;

/// How a trait grid draws its cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraitGridLook {
    /// Option cards with art, label and description, three per row.
    Illustrated,
    /// Cards with art and label only, [`SMALL_COLUMNS`] per row.
    Small,
}

/// Builds the grid once the concrete option type is known.
pub struct TraitPick {
    id: SharedString,
    theme: ThemeTokens,
    look: TraitGridLook,
    on_pick: ValueCallback<TraitValue>,
}

impl TraitVisitor for TraitPick {
    type Output = AnyElement;

    fn visit<T: TraitOption>(self, current: T, wrap: fn(T) -> TraitValue) -> AnyElement {
        let on_pick = self.on_pick;
        let grid = IllustratedOptionGrid::new(self.id, Some(current), &self.theme, move |option: T, window, cx| {
            on_pick(wrap(option), window, cx)
        });
        match self.look {
            TraitGridLook::Illustrated => grid.into_any_element(),
            TraitGridLook::Small => {
                grid.columns(SMALL_COLUMNS).compact().into_any_element()
            }
        }
    }
}

/// Illustrated cards for every option of `field`, ringing its value on `cfg`; picking one
/// calls `on_pick` with the new [`TraitValue`]. `id` keys the grid's keyboard cursor.
pub fn trait_option_grid(
    id: SharedString,
    field: TraitField,
    cfg: &AgentConfig,
    look: TraitGridLook,
    theme: &ThemeTokens,
    on_pick: ValueCallback<TraitValue>,
) -> AnyElement {
    field.read(cfg).visit(TraitPick { id, theme: theme.clone(), look, on_pick })
}
