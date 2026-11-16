//! Unified Personas & Network Agents editor workspace.
//!
//! Implements a Three-Tier Progressive Disclosure Layout grounded in cognitive psychology
//! (Hick's Law, Miller's Law chunking, Gestalt proximity & figure-ground):
//! - **Tier 1 (`roster`)**: compact vertical contact list with a segmented
//!   `Network Agents` / `My Personas` switcher, avatar, name, `@handle` and MBTI badge.
//! - **Tier 2 (`rail`)**: primary configuration categories with live summary subtitles and
//!   1-click quick archetype presets.
//! - **Tier 3 (`subtab_bar` + `steps`)**: peer sub-tabs within each category, each rendering
//!   a compact single-viewport body.
//! - **100% tokenized**: every visual comes from `ThemeTokens` via `crate::ui::components`.

pub mod callbacks;
mod config_pane;
pub mod config_tab;
mod header;
mod header_actions;
pub mod inputs;
pub mod params;
mod rail;
mod roster;
mod steps;
mod subtab_bar;
mod tab_summary;
pub mod workspace;

#[cfg(test)]
mod tests;

pub use callbacks::{
    ActivatePersonaCallback, CreateProfileCallback, DeleteProfileCallback, PickAvatarCallback,
    SaveTextInputsCallback, SelectConfigTabCallback, SelectProfileCallback, SelectSubTabCallback,
    SwitchLabModeCallback, TriggerAgentSimCallback, UpdateProfileCallback,
};
pub use config_tab::{AgentConfigTab, LabMode};
pub use inputs::AgentEditorInputs;
pub use params::AgentWorkspaceParams;
pub use workspace::render_agent_workspace_view;
