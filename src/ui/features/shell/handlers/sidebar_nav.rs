//! Left sidebar handlers: its width (Ctrl+B and the window breakpoint), folding sections,
//! "Show all", pins and opening pinned or recent items.

use super::super::view::AppView;
use crate::domain::post::{SourceKind, SourceRef};
use crate::domain::ProfileId;
use crate::state::NavigationTab;
use crate::ui::features::sidebar_left::{toggled_preference, NavRef, SidebarMode, SidebarSection};
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;

impl AppView {
    /// Whether the left sidebar renders as the icon rail.
    pub(crate) fn is_left_rail(&self) -> bool {
        self.sidebar.mode(&self.theme) == SidebarMode::Rail
    }

    /// Follows window resizes, so the sidebar folds outside the wide layout class.
    pub(in crate::ui::features::shell) fn observe_window_width(window: &mut Window, cx: &mut Context<Self>) -> Subscription {
        cx.observe_window_bounds(window, |this, window, cx| {
            let width = window.viewport_size().width;
            if width != this.sidebar.window_width {
                this.sidebar.window_width = width;
                cx.notify();
            }
        })
    }

    pub(crate) fn toggle_left_sidebar(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let layout = LayoutClass::for_width(self.sidebar.window_width, &self.theme);
        let prefs = self.sidebar.prefs.with_preference(toggled_preference(layout, self.sidebar.prefs.preference));
        self.sidebar.update_prefs(prefs, self.state.db.as_ref());
        cx.notify();
    }

    pub(crate) fn toggle_sidebar_section(&mut self, section: SidebarSection, _: &mut Window, cx: &mut Context<Self>) {
        let prefs = self.sidebar.prefs.with_section_toggled(section);
        self.sidebar.update_prefs(prefs, self.state.db.as_ref());
        cx.notify();
    }

    pub(crate) fn toggle_sidebar_show_all(&mut self, section: SidebarSection, _: &mut Window, cx: &mut Context<Self>) {
        self.sidebar = self.sidebar.with_show_all_toggled(section);
        cx.notify();
    }

    pub(crate) fn toggle_pin(&mut self, item: NavRef, _: &mut Window, cx: &mut Context<Self>) {
        let prefs = self.sidebar.prefs.with_pin_toggled(&item);
        self.sidebar.update_prefs(prefs, self.state.db.as_ref());
        cx.notify();
    }

    /// Pins or unpins the thread open in the inspector.
    pub(crate) fn pin_open_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(post_id) = self.feed.selected_thread_post_id.clone() {
            self.toggle_pin(NavRef::Thread(post_id), window, cx);
        }
    }

    /// Opens a pinned, recent, source or member item.
    pub(crate) fn open_nav_item(&mut self, item: NavRef, window: &mut Window, cx: &mut Context<Self>) {
        match item {
            NavRef::Thread(post_id) => self.reveal_post(post_id, window, cx),
            NavRef::Document(id) => {
                self.sidebar.note_recent(NavRef::Document(id), self.state.db.as_ref());
                self.select_nav(NavigationTab::Knowledge, window, cx);
            }
            NavRef::Profile(id) => self.open_profile(id, window, cx),
        }
    }

    /// Opens an agent or persona in its lab and remembers it as recent.
    pub(crate) fn open_profile(&mut self, id: ProfileId, window: &mut Window, cx: &mut Context<Self>) {
        self.note_recent(NavRef::Profile(id.clone()));
        self.open_source(SourceRef::new(SourceKind::Profile, id.to_string(), String::new()), window, cx);
    }

    /// Remembers `item` in the sidebar's Recent section.
    pub(crate) fn note_recent(&mut self, item: NavRef) {
        self.sidebar.note_recent(item, self.state.db.as_ref());
    }
}
