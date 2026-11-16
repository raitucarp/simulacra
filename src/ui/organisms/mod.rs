//! Organism-level layout panels.

pub mod agent_lab_view;
pub mod app_title_bar;
pub mod composer_modal;
pub mod knowledge_vault_view;
pub mod left_sidebar;
pub use crate::ui::features::feed as main_feed;
pub mod mind_graph_view;
pub mod right_sidebar;
pub mod secondary_sidebar;
pub mod settings_modal;
pub mod unified_right_sidebar;

pub use agent_lab_view::{
    render_agent_lab_view, render_agent_workspace_view, AgentConfigTab, AgentEditorInputs,
    AgentWorkspaceParams, LabMode,
};
pub use app_title_bar::AppTitleBar;
pub use composer_modal::{ComposerModalMode, ComposerModalView};
pub use knowledge_vault_view::render_knowledge_vault_view;
pub use left_sidebar::{render_left_sidebar, LeftSidebarParams, NewThreadCallback};
pub use main_feed::{
    compute_stream_threadline_style, render_main_feed, FeedParams, DEFAULT_FEED_WINDOW_SIZE,
    FEED_WINDOW_INCREMENT,
};
pub use mind_graph_view::render_mind_graph_view;
pub use right_sidebar::render_right_sidebar;
pub use secondary_sidebar::render_secondary_sidebar;
pub use settings_modal::{SettingsModal, SettingsTab};
pub use unified_right_sidebar::{render_unified_right_sidebar, RightSidebarTab, UnifiedRightSidebarParams};
