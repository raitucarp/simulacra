//! Organism-level layout panels.

pub use crate::ui::features::composer::modal as composer_modal;
pub use crate::ui::features::feed as main_feed;
pub use crate::ui::features::mind_graph as mind_graph_view;
pub use crate::ui::features::settings as settings_modal;
pub use crate::ui::features::sidebar_left as left_sidebar;
pub use crate::ui::features::sidebar_right::secondary as secondary_sidebar;
pub use crate::ui::features::sidebar_right::unified as unified_right_sidebar;
pub use crate::ui::features::title_bar as app_title_bar;
pub use crate::ui::features::vault as knowledge_vault_view;

pub use app_title_bar::AppTitleBar;
pub use composer_modal::{ComposerModalMode, ComposerModalView};
pub use knowledge_vault_view::{VaultEvent, VaultPanel};
pub use left_sidebar::{render_left_sidebar, LeftSidebarParams, NewThreadCallback};
pub use main_feed::{
    compute_stream_threadline_style, render_main_feed, FeedParams, DEFAULT_FEED_WINDOW_SIZE,
    FEED_WINDOW_INCREMENT,
};
pub use mind_graph_view::{MindGraphEvent, MindGraphPanel};
pub use settings_modal::{SettingsModal, SettingsTab};
pub use unified_right_sidebar::{render_unified_right_sidebar, RightSidebarTab, UnifiedRightSidebarParams};
