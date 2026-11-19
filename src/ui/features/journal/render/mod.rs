//! The Journal page: a header (art, title, privacy note) with its tabs above the current tab,
//! in a centred column; focus mode replaces all of it with the timer alone.

mod affect_grid;
mod associations;
mod capsule_open;
mod capsules;
mod checkin;
mod common;
pub(in crate::ui::features::journal) mod confirm; // sb P16: deletes ask first
mod editor;
mod empty;
mod entries;
mod focus;
mod focus_log;
mod focus_mode;
mod header;
mod heat;
mod mood;
mod mood_charts;
mod today;

use super::panel::JournalPanel;
use super::tab::JournalTab;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;

const ROOT_ID: &str = "journal-root";
const SCROLL_ID: &str = "journal-scroll";

impl Render for JournalPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.clone();
        let class = LayoutClass::of_window(window, &theme);
        let root = div().id(ROOT_ID).size_full().track_focus(&self.focus).bg(theme.bg_app).font_family(theme.font_family_ui.clone());
        if self.focus_run.focus_mode {
            return root.child(self.render_focus_mode(class, cx));
        }
        let column = div()
            .w_full()
            .max_w(theme.journal_page_max_width)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.render_header(class, cx))
            .child(self.render_tab(class, cx));
        let inset = if class.is_compact() { theme.panel_padding_compact } else { theme.panel_padding };
        // Top-aligned: a stretched column would be clipped to the viewport and never scroll.
        root.child(div().id(SCROLL_ID).size_full().overflow_y_scroll().flex().justify_center().items_start().px(inset).py(theme.panel_padding).child(column))
    }
}

impl JournalPanel {
    fn render_tab(&self, class: LayoutClass, cx: &mut Context<Self>) -> AnyElement {
        match self.tab {
            JournalTab::Today => self.render_today(class, cx).into_any_element(),
            JournalTab::Journal => self.render_journal(class, cx).into_any_element(),
            JournalTab::Mood => self.render_mood(class, cx).into_any_element(),
            JournalTab::Focus => self.render_focus(class, cx).into_any_element(),
            JournalTab::Capsules => self.render_capsules(cx).into_any_element(),
        }
    }
}
