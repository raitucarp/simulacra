//! Where the right panel column shows. Its graph, cadence and tags act on the feed, so by
//! default it belongs to the timeline only, and only on a window wide enough for four
//! columns; on any place the user can open or close it, and that choice is remembered there.

use super::super::view::AppView;
use crate::state::NavigationTab;
use gpui_kit::Pixels;

impl AppView {
    /// Whether the right panel column shows on the current place in a window `width` wide.
    pub(in crate::ui::features::shell) fn is_right_panel_shown(&self, width: Pixels) -> bool {
        let tab = self.state.active_tab;
        let choice = self.chrome.right_panel_choices.iter().find(|(place, _)| *place == tab).map(|(_, shown)| *shown);
        choice.unwrap_or(tab == NavigationTab::Timeline && width >= self.theme.right_panel_auto_min_width)
    }
}
