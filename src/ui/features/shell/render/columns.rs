//! The three resizable columns: left navigation, center content, right intelligence panel.
//! While the Agent Forge is open the right column is dropped and the forge fills the center,
//! under its own layout id so the normal column sizes are kept for when it closes. Column
//! sizes persist per layout id (see `persisted_split`).

use super::super::consts::{
    LAYOUT_ID_COLLAPSED, LAYOUT_ID_EXPANDED, LAYOUT_ID_FORGE_COLLAPSED, LAYOUT_ID_FORGE_EXPANDED,
};
use super::super::panels::ShellSection;
use super::super::view::AppView;
use crate::ui::components::{persisted_h_resizable, SplitPane};
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
        let t = &self.theme;
        let columns = persisted_h_resizable(layout_id, t)
            // Panel 1: Left Navigation Sidebar
            .pane(SplitPane::sized(width, min..max).child(self.panels.cached(ShellSection::LeftSidebar)))
            // Panel 2: Center: feed, roster, Mind Graph or the Agent Forge
            .pane(SplitPane::flex(t.split_flex_min).child(self.panels.cached(ShellSection::Center)));
        if self.forge.is_some() {
            return columns;
        }
        // Panel 3: Unified Intelligence & Telemetry Sidebar
        columns.pane(
            SplitPane::sized(t.right_sidebar_width, t.right_sidebar_min..t.right_sidebar_max)
                .visible(self.chrome.is_right_sidebar_open)
                .child(self.panels.cached(ShellSection::RightSidebar)),
        )
    }
}
