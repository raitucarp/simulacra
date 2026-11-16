//! Content panes for each settings tab.

mod ai;
mod ai_fields;
mod appearance;
mod common;
mod general;
mod vault;

pub(super) use ai::render_ai_pane;
pub(super) use appearance::render_appearance_pane;
pub(super) use general::render_general_pane;
pub(super) use vault::render_vault_pane;
