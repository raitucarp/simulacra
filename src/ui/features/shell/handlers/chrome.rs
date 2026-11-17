//! Window chrome handlers: title bar toggles, menus, language and the right column.

use super::super::view::AppView;
use crate::i18n::Language;
use crate::state::NavigationTab;
use crate::ui::features::feed::DEFAULT_FEED_WINDOW_SIZE;
use crate::ui::features::sidebar_right::RightSidebarTab;
use crate::ui::responsive::LayoutClass;
use chrono::NaiveDate;
use gpui_kit::*;

impl AppView {
    /// Switches the active application language and updates all interface tokens dynamically.
    pub fn switch_language(&mut self, lang: Language, cx: &mut Context<Self>) {
        self.chrome.current_language = lang;
        crate::i18n::set_locale(lang);
        self.state.set_language(lang);
        cx.notify();
    }

    pub(crate) fn toggle_left_sidebar(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.is_left_sidebar_collapsed = !self.chrome.is_left_sidebar_collapsed;
        cx.notify();
    }

    /// Shows or hides the right panel: its drawer in the compact layout, else its column.
    pub(crate) fn toggle_right_sidebar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let chrome = &mut self.chrome;
        if LayoutClass::of_window(window, &self.theme).is_compact() {
            chrome.is_right_drawer_open = !chrome.is_right_drawer_open;
        } else {
            chrome.is_right_sidebar_open = !chrome.is_right_sidebar_open;
        }
        cx.notify();
    }

    pub(crate) fn close_right_drawer(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.is_right_drawer_open = false;
        cx.notify();
    }

    /// Toggles the language menu, closing the other menus.
    pub(crate) fn toggle_lang_menu(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let chrome = &mut self.chrome;
        chrome.is_lang_menu_open = !chrome.is_lang_menu_open;
        chrome.is_settings_open = false;
        chrome.is_workspace_open = false;
        cx.notify();
    }

    /// Toggles the settings modal, closing the other menus.
    pub(crate) fn toggle_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.revert_settings_draft(window, cx);
        let chrome = &mut self.chrome;
        chrome.is_settings_open = !chrome.is_settings_open;
        chrome.is_lang_menu_open = false;
        chrome.is_workspace_open = false;
        cx.notify();
    }

    /// Toggles the workspace switcher, closing the other menus.
    pub(crate) fn toggle_workspace(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let chrome = &mut self.chrome;
        chrome.is_workspace_open = !chrome.is_workspace_open;
        chrome.is_lang_menu_open = false;
        chrome.is_settings_open = false;
        cx.notify();
    }

    /// Applies the language picked in the language menu and closes it.
    pub(crate) fn select_language(&mut self, lang: Language, _: &mut Window, cx: &mut Context<Self>) {
        self.switch_language(lang, cx);
        self.chrome.is_lang_menu_open = false;
    }

    pub(crate) fn dismiss_lang_menu(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.is_lang_menu_open = false;
        cx.notify();
    }

    pub(crate) fn dismiss_workspace(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.is_workspace_open = false;
        cx.notify();
    }

    pub(crate) fn select_right_sidebar_tab(&mut self, tab: RightSidebarTab, _: &mut Window, cx: &mut Context<Self>) {
        self.chrome.right_sidebar_tab = tab;
        cx.notify();
    }

    /// Narrows the feed to a cadence heatmap day (showing the timeline), or clears the
    /// filter when the same day is clicked again.
    pub(crate) fn toggle_calendar_day(&mut self, day: NaiveDate, _: &mut Window, cx: &mut Context<Self>) {
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
