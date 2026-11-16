//! Left navigation sidebar and unified right sidebar wiring.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::sidebar_left::{render_left_sidebar, LeftSidebarParams};
use crate::ui::features::sidebar_right::{render_unified_right_sidebar, UnifiedRightSidebarParams};
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// Navigation, workspace switcher, members and persona switcher.
    pub(super) fn left_sidebar(&self, bind: &Binder) -> impl IntoElement {
        render_left_sidebar(LeftSidebarParams {
            state: &self.state,
            theme: &self.theme,
            is_workspace_open: self.chrome.is_workspace_open,
            is_collapsed: self.chrome.is_left_sidebar_collapsed,
            on_toggle_workspace: bind.boxed_act(Self::toggle_workspace),
            on_select_nav: bind.boxed(Self::select_nav),
            on_new_thread: bind.boxed_act(Self::new_thread),
            on_select_agent: Some(bind.boxed(Self::select_sidebar_agent)),
            on_switch_persona: Some(bind.boxed_act(Self::switch_persona)),
        })
    }

    /// Mind & Cadence / Vault & Telemetry panel.
    pub(super) fn right_sidebar(&self, bind: &Binder) -> impl IntoElement {
        let topic_entity = bind.entity().clone();
        render_unified_right_sidebar(UnifiedRightSidebarParams {
            active_tab: self.chrome.right_sidebar_tab,
            on_select_tab: bind.boxed(Self::select_right_sidebar_tab),
            state: &self.state,
            posts: self.state.posts(),
            documents: self.state.documents(),
            selected_day: self.chrome.selected_calendar_day,
            on_click_day: Rc::new(bind.with(Self::toggle_calendar_day)),
            search_input: &self.feed.search_input,
            on_click_topic: Box::new(move |tag, window, cx| {
                topic_entity.update(cx, |this, cx| this.search_topic(tag, window, cx));
            }),
            theme: &self.theme,
        })
    }
}
