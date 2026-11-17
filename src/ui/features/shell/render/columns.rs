//! The three resizable columns: left navigation, center content, right intelligence panel.
//! While the Agent Forge is open, or in the compact layout class (where the right panel is a
//! drawer), the right column is dropped; the forge and the medium class use their own layout
//! ids so each arrangement keeps its own column sizes. Sizes persist per layout id (see
//! `persisted_split`).

use super::super::consts::{
    LAYOUT_ID_COLLAPSED, LAYOUT_ID_EXPANDED, LAYOUT_ID_FORGE_COLLAPSED, LAYOUT_ID_FORGE_EXPANDED,
    LAYOUT_ID_MEDIUM_COLLAPSED, LAYOUT_ID_MEDIUM_EXPANDED,
};
use super::super::panels::ShellSection;
use super::super::view::AppView;
use crate::ui::components::{persisted_h_resizable, SplitPane};
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;

/// Layout id of the columns for (forge open, medium class, left sidebar collapsed).
fn layout_id(forge: bool, medium: bool, collapsed: bool) -> &'static str {
    match (forge, medium, collapsed) {
        (true, _, true) => LAYOUT_ID_FORGE_COLLAPSED,
        (true, _, false) => LAYOUT_ID_FORGE_EXPANDED,
        (false, true, true) => LAYOUT_ID_MEDIUM_COLLAPSED,
        (false, true, false) => LAYOUT_ID_MEDIUM_EXPANDED,
        (false, false, true) => LAYOUT_ID_COLLAPSED,
        (false, false, false) => LAYOUT_ID_EXPANDED,
    }
}

impl AppView {
    /// Layout id, width and resize range of the left sidebar.
    fn left_column(&self, class: LayoutClass) -> (&'static str, Pixels, Pixels, Pixels) {
        let collapsed = self.chrome.is_left_sidebar_collapsed;
        let id = layout_id(self.forge.is_some(), class == LayoutClass::Medium, collapsed);
        let t = &self.theme;
        if collapsed {
            (id, t.left_sidebar_collapsed_width, t.left_sidebar_collapsed_min, t.left_sidebar_collapsed_max)
        } else {
            (id, t.left_sidebar_expanded_width, t.left_sidebar_expanded_min, t.left_sidebar_expanded_max)
        }
    }

    /// Resizable layout of the left sidebar, the center panel and (without the forge, outside
    /// the compact class) the right sidebar.
    pub(super) fn render_columns(&self, class: LayoutClass) -> impl IntoElement {
        let (layout_id, width, min, max) = self.left_column(class);
        let t = &self.theme;
        let columns = persisted_h_resizable(layout_id, t)
            // Panel 1: Left Navigation Sidebar
            .pane(SplitPane::sized(width, min..max).child(self.panels.cached(ShellSection::LeftSidebar)))
            // Panel 2: Center: feed, roster, Mind Graph or the Agent Forge
            .pane(SplitPane::flex(t.split_flex_min).child(self.panels.cached(ShellSection::Center)));
        if self.forge.is_some() || class.is_compact() {
            return columns;
        }
        // Panel 3: Unified Intelligence & Telemetry Sidebar
        let (right, right_min, right_max) = class.right_sidebar_size(t);
        columns.pane(
            SplitPane::sized(right, right_min..right_max)
                .visible(self.chrome.is_right_sidebar_open)
                .child(self.panels.cached(ShellSection::RightSidebar)),
        )
    }
}
