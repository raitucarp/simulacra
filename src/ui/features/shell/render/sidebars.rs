//! Left navigation sidebar and unified right sidebar wiring.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::sidebar_left::{render_left_sidebar, LeftSidebarParams, SidebarCallbacks};
use crate::ui::features::sidebar_right::{render_unified_right_sidebar, UnifiedRightSidebarParams};
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// Network switcher, New menu, places, pinned / recent / sources / members and the footer.
    pub(in crate::ui::features::shell) fn left_sidebar(&self, bind: &Binder) -> impl IntoElement {
        render_left_sidebar(LeftSidebarParams {
            state: &self.state,
            theme: &self.theme,
            sidebar: &self.sidebar,
            is_workspace_open: self.chrome.is_workspace_open,
            shell_focus: self.palette.shell_focus.clone(),
            callbacks: SidebarCallbacks {
                on_select_nav: Rc::new(bind.with(Self::select_nav)),
                on_command: Rc::new(bind.with(Self::run_command)),
                on_open: Rc::new(bind.with(Self::open_nav_item)),
                on_toggle_pin: Rc::new(bind.with(Self::toggle_pin)),
                on_toggle_section: Rc::new(bind.with(Self::toggle_sidebar_section)),
                on_toggle_show_all: Rc::new(bind.with(Self::toggle_sidebar_show_all)),
            },
        })
    }

    /// Mind & Cadence / Vault & Telemetry panel.
    pub(in crate::ui::features::shell) fn right_sidebar(&self, bind: &Binder) -> impl IntoElement {
        let topic_entity = bind.entity().clone();
        let heatmap = self.state.current_activity_heatmap();
        render_unified_right_sidebar(UnifiedRightSidebarParams {
            active_tab: self.chrome.right_sidebar_tab,
            on_select_tab: bind.boxed(Self::select_right_sidebar_tab),
            state: &self.state,
            posts: self.state.posts(),
            heatmap: &heatmap,
            selected_day: self.state.feed_day_filter,
            on_click_day: Rc::new(bind.with(Self::toggle_calendar_day)),
            graph: self.graph.cache.current(),
            on_open_graph: Rc::new(bind.act(Self::open_mind_graph)),
            search_input: &self.feed.search_input,
            on_click_topic: Box::new(move |tag, window, cx| {
                topic_entity.update(cx, |this, cx| this.search_topic(tag, window, cx));
            }),
            on_import_document: bind.boxed_act(Self::pick_import_files),
            theme: &self.theme,
        })
    }
}
