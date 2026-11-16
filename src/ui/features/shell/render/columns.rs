//! The three resizable columns: left navigation, center content, right intelligence panel.

use super::super::binder::Binder;
use super::super::consts::{
    LAYOUT_ID_COLLAPSED, LAYOUT_ID_EXPANDED, LEFT_SIDEBAR_COLLAPSED_MAX, LEFT_SIDEBAR_COLLAPSED_MIN,
    LEFT_SIDEBAR_COLLAPSED_WIDTH, LEFT_SIDEBAR_EXPANDED_MAX, LEFT_SIDEBAR_EXPANDED_MIN,
    LEFT_SIDEBAR_EXPANDED_WIDTH, RIGHT_SIDEBAR_MAX, RIGHT_SIDEBAR_MIN, RIGHT_SIDEBAR_WIDTH,
};
use super::super::view::AppView;
use gpui_kit::component::resizable::{h_resizable, resizable_panel};
use gpui_kit::*;

impl AppView {
    /// Resizable layout of the left sidebar, the center panel and the right sidebar.
    pub(super) fn render_columns(&self, bind: &Binder) -> impl IntoElement {
        let collapsed = self.chrome.is_left_sidebar_collapsed;
        let (layout_id, width, min, max) = if collapsed {
            (
                LAYOUT_ID_COLLAPSED,
                LEFT_SIDEBAR_COLLAPSED_WIDTH,
                LEFT_SIDEBAR_COLLAPSED_MIN,
                LEFT_SIDEBAR_COLLAPSED_MAX,
            )
        } else {
            (
                LAYOUT_ID_EXPANDED,
                LEFT_SIDEBAR_EXPANDED_WIDTH,
                LEFT_SIDEBAR_EXPANDED_MIN,
                LEFT_SIDEBAR_EXPANDED_MAX,
            )
        };
        h_resizable(layout_id)
            // Panel 1: Left Navigation Sidebar
            .child(
                resizable_panel()
                    .size(px(width))
                    .size_range(px(min)..px(max))
                    .child(self.left_sidebar(bind)),
            )
            // Panel 2: Center Main Dual Feed OR Two-Pane Agent/Persona Lab
            .child(resizable_panel().child(self.center_panel(bind)))
            // Panel 3: Unified Intelligence & Telemetry Sidebar
            .child(
                resizable_panel()
                    .size(px(RIGHT_SIDEBAR_WIDTH))
                    .size_range(px(RIGHT_SIDEBAR_MIN)..px(RIGHT_SIDEBAR_MAX))
                    .visible(self.chrome.is_right_sidebar_open)
                    .child(self.right_sidebar(bind)),
            )
    }
}
