//! The three resizable columns: left navigation, center content, right intelligence panel.
//! While the Agent Forge is open the right column is dropped and the forge fills the center,
//! under its own layout id so the normal column sizes are kept for when it closes.

use super::super::consts::{
    LAYOUT_ID_COLLAPSED, LAYOUT_ID_EXPANDED, LAYOUT_ID_FORGE_COLLAPSED, LAYOUT_ID_FORGE_EXPANDED,
    LEFT_SIDEBAR_COLLAPSED_MAX, LEFT_SIDEBAR_COLLAPSED_MIN, LEFT_SIDEBAR_COLLAPSED_WIDTH, LEFT_SIDEBAR_EXPANDED_MAX,
    LEFT_SIDEBAR_EXPANDED_MIN, LEFT_SIDEBAR_EXPANDED_WIDTH, RIGHT_SIDEBAR_MAX, RIGHT_SIDEBAR_MIN, RIGHT_SIDEBAR_WIDTH,
};
use super::super::panels::ShellSection;
use super::super::view::AppView;
use gpui_kit::component::resizable::{h_resizable, resizable_panel};
use gpui_kit::*;

impl AppView {
    /// Layout id, width and resize range of the left sidebar.
    fn left_column(&self) -> (&'static str, f32, f32, f32) {
        let forge = self.forge.is_some();
        if self.chrome.is_left_sidebar_collapsed {
            let id = if forge { LAYOUT_ID_FORGE_COLLAPSED } else { LAYOUT_ID_COLLAPSED };
            (id, LEFT_SIDEBAR_COLLAPSED_WIDTH, LEFT_SIDEBAR_COLLAPSED_MIN, LEFT_SIDEBAR_COLLAPSED_MAX)
        } else {
            let id = if forge { LAYOUT_ID_FORGE_EXPANDED } else { LAYOUT_ID_EXPANDED };
            (id, LEFT_SIDEBAR_EXPANDED_WIDTH, LEFT_SIDEBAR_EXPANDED_MIN, LEFT_SIDEBAR_EXPANDED_MAX)
        }
    }

    /// Resizable layout of the left sidebar, the center panel and (without the forge) the
    /// right sidebar.
    pub(super) fn render_columns(&self) -> impl IntoElement {
        let (layout_id, width, min, max) = self.left_column();
        let columns = h_resizable(layout_id)
            // Panel 1: Left Navigation Sidebar
            .child(
                resizable_panel()
                    .size(px(width))
                    .size_range(px(min)..px(max))
                    .child(self.panels.cached(ShellSection::LeftSidebar)),
            )
            // Panel 2: Center: feed, roster, Mind Graph or the Agent Forge
            .child(resizable_panel().child(self.panels.cached(ShellSection::Center)));
        if self.forge.is_some() {
            return columns;
        }
        // Panel 3: Unified Intelligence & Telemetry Sidebar
        columns.child(
            resizable_panel()
                .size(px(RIGHT_SIDEBAR_WIDTH))
                .size_range(px(RIGHT_SIDEBAR_MIN)..px(RIGHT_SIDEBAR_MAX))
                .visible(self.chrome.is_right_sidebar_open)
                .child(self.panels.cached(ShellSection::RightSidebar)),
        )
    }
}
