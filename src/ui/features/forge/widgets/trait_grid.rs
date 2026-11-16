//! [`trait_option_grid`]: an [`IllustratedOptionGrid`] for a trait field chosen at runtime,
//! dispatched to the field's concrete option type through [`TraitVisitor`].

use super::option_grid::IllustratedOptionGrid;
use super::ValueCallback;
use crate::domain::agent_config::AgentConfig;
use crate::domain::TraitOption;
use crate::state::forge::{TraitField, TraitValue, TraitVisitor};
use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Element id prefix of runtime trait grids (`forge-trait-<path>`).
const GRID_ID_PREFIX: &str = "forge-trait";

/// Builds the grid once the concrete option type is known.
pub struct TraitPick {
    id: SharedString,
    theme: ThemeTokens,
    compact: bool,
    on_pick: ValueCallback<TraitValue>,
}

impl TraitVisitor for TraitPick {
    type Output = AnyElement;

    fn visit<T: TraitOption>(self, current: T, wrap: fn(T) -> TraitValue) -> AnyElement {
        let on_pick = self.on_pick;
        let grid = IllustratedOptionGrid::new(self.id, Some(current), &self.theme, move |option: T, window, cx| {
            on_pick(wrap(option), window, cx)
        });
        if self.compact {
            grid.compact().into_any_element()
        } else {
            grid.into_any_element()
        }
    }
}

/// Illustrated cards for every option of `field`, ringing its value on `cfg`; picking one
/// calls `on_pick` with the new [`TraitValue`]. `compact` hides descriptions.
pub fn trait_option_grid(
    field: TraitField,
    cfg: &AgentConfig,
    compact: bool,
    theme: &ThemeTokens,
    on_pick: ValueCallback<TraitValue>,
) -> AnyElement {
    let id = SharedString::from(format!("{GRID_ID_PREFIX}-{}", field.path()));
    field.read(cfg).visit(TraitPick { id, theme: theme.clone(), compact, on_pick })
}
