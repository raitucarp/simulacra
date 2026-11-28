//! Window chrome handlers: title bar toggles, menus, language and the right column.

use super::super::view::AppView;
use crate::i18n::Language;
use crate::state::NavigationTab;
use crate::ui::features::feed::DEFAULT_FEED_WINDOW_SIZE;
use crate::ui::features::sidebar_right::RightSidebarTab;
use crate::ui::responsive::LayoutClass;
use jiff::civil::Date;
use gpui_kit::*;

impl AppView {
    /// Switches the active application language and updates all interface tokens dynamically.
    pub fn switch_language(&mut self, lang: Language, cx: &mut Context<Self>) {
        self.chrome.current_language = lang;
        crate::i18n::set_locale(lang);
        self.state.set_language(lang);
        cx.notify();
    }

    /// Shows or hides the right panel: its drawer in the compact layout, else its column on
    /// the current place (remembered per place).
    pub(crate) fn toggle_right_sidebar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if LayoutClass::of_window(window, &self.theme).is_compact() {
            self.chrome.is_right_drawer_open = !self.chrome.is_right_drawer_open;
        } else {
            let shown = self.is_right_panel_shown(window.viewport_size().width);
            let tab = self.state.active_tab;
            self.chrome.right_panel_choices.retain(|(place, _)| *place != tab);
            self.chrome.right_panel_choices.push((tab, !shown));
        }
        cx.notify();
    }

    pub(crate) fn close_right_drawer(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.is_right_drawer_open = false;
        cx.notify();
    }

    /// Toggles the settings modal, closing the network menu. An opened modal takes the
    /// keyboard focus (its rail), so Escape closes it at once.
    pub(crate) fn toggle_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.revert_settings_draft(window, cx);
        let chrome = &mut self.chrome;
        chrome.is_settings_open = !chrome.is_settings_open;
        chrome.workspace_menu.close();
        if chrome.is_settings_open {
            window.focus(&self.settings_ui.rail_focus, cx);
        }
        cx.notify();
    }

    /// Opens the network menu (focused, so the arrow keys move through it), or closes it.
    pub(crate) fn toggle_workspace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.chrome.is_settings_open = false;
        if self.chrome.workspace_menu.is_open() {
            self.chrome.workspace_menu.close();
        } else {
            let spec = self.network_menu(cx);
            self.chrome.workspace_menu.open(spec, window, cx, |this, _, cx| {
                this.chrome.workspace_menu.close();
                cx.notify();
            });
        }
        cx.notify();
    }

    /// The network menu's spec, its picks sent back to this view.
    fn network_menu(&self, cx: &mut Context<Self>) -> crate::ui::menus::MenuSpec {
        let view = cx.entity().downgrade();
        let (select_view, create_view) = (view.clone(), view);
        let on_select: crate::ui::features::popovers::workspace::SelectNetworkCallback = std::rc::Rc::new(move |id, window, cx| {
            let _ = select_view.update(cx, |this, cx| this.select_network(id, window, cx));
        });
        let on_create: crate::ui::features::popovers::workspace::CreateNetworkCallback = std::rc::Rc::new(move |window, cx| {
            let _ = create_view.update(cx, |this, cx| this.open_create_network(window, cx));
        });
        let active = self.state.current_network().map(|network| network.id.clone());
        crate::ui::features::popovers::network_menu_spec(&self.state.networks, active.as_ref(), self.theme.workspace_popover_width, &on_select, &on_create)
    }

    /// Applies the language picked in the language menu.
    pub(crate) fn select_language(&mut self, lang: Language, _: &mut Window, cx: &mut Context<Self>) {
        self.switch_language(lang, cx);
    }

    pub(crate) fn dismiss_workspace(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.workspace_menu.close();
        cx.notify();
    }

    pub(crate) fn select_right_sidebar_tab(&mut self, tab: RightSidebarTab, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.right_sidebar_tab = tab;
        cx.notify();
    }

    /// Narrows the feed to a cadence heatmap day (showing the timeline), or clears the
    /// filter when the same day is clicked again.
    pub(crate) fn toggle_calendar_day(&mut self, day: Date, _: &mut Window, cx: &mut Context<Self>) {
        if self.state.toggle_feed_day_filter(day).is_some() {
            self.state.active_tab = NavigationTab::Timeline;
        }
        self.feed.feed_visible_count = DEFAULT_FEED_WINDOW_SIZE;
        cx.notify();
    }

    /// Removes the feed's day filter.
    pub(crate) fn clear_day_filter(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.state.clear_feed_day_filter();
        self.feed.feed_visible_count = DEFAULT_FEED_WINDOW_SIZE;
        cx.notify();
    }

    /// Searches the feed for a clicked trending topic.
    pub(crate) fn search_topic(&mut self, tag: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.feed.search_input.update(cx, |input, cx| input.set_value(tag, window, cx));
        cx.notify();
    }
}
