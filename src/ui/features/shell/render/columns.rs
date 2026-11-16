//! The three resizable columns: left navigation, center content, right intelligence panel.
//! While the Agent Forge is open the right column is dropped and the forge fills the center,
//! under its own layout id so the normal column sizes are kept for when it closes.

use super::super::consts::{
    LAYOUT_ID_COLLAPSED, LAYOUT_ID_EXPANDED, LAYOUT_ID_FORGE_COLLAPSED, LAYOUT_ID_FORGE_EXPANDED,
};
use super::super::panels::ShellSection;
use super::super::view::AppView;
use gpui_kit::component::resizable::{h_resizable, resizable_panel};
use gpui_kit::*;

impl AppView {
    /// Layout id, width and resize range of the left sidebar.
    fn left_column(&self) -> (&'static str, Pixels, Pixels, Pixels) {
        let forge = self.forge.is_some();
        let t = &self.theme;
        if self.chrome.is_left_sidebar_collapsed {
            let id = if forge { LAYOUT_ID_FORGE_COLLAPSED } else { LAYOUT_ID_COLLAPSED };
            (id, t.left_sidebar_collapsed_width, t.left_sidebar_collapsed_min, t.left_sidebar_collapsed_max)
        } else {
            let id = if forge { LAYOUT_ID_FORGE_EXPANDED } else { LAYOUT_ID_EXPANDED };
            (id, t.left_sidebar_expanded_width, t.left_sidebar_expanded_min, t.left_sidebar_expanded_max)
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
                    .size(width)
                    .size_range(min..max)
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
                .size(self.theme.right_sidebar_width)
                .size_range(self.theme.right_sidebar_min..self.theme.right_sidebar_max)
                .visible(self.chrome.is_right_sidebar_open)
                .child(self.panels.cached(ShellSection::RightSidebar)),
        )
    }
}
