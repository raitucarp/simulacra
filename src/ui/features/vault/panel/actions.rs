//! Navigation and browse controls of the vault panel.

use super::VaultPanel;
use crate::domain::document::citation::CitationStyle;
use crate::domain::document::facets::Facet;
use crate::domain::document::query::{CatalogueQuery, SortKey};
use crate::domain::document::record::{Reading, PROGRESS_COMPLETE};
use crate::domain::document::SubjectId;
use crate::ui::features::vault::screen::{BrowseLayout, Navigation, Screen};
use crate::ui::features::vault::VaultEvent;
use gpui_kit::{ClipboardItem, Context, Window};

/// Reading progress is stored in steps of this many percent while scrolling.
const PROGRESS_STEP: u8 = 5;

impl VaultPanel {
    pub(in crate::ui::features::vault) fn go(&mut self, screen: Screen, cx: &mut Context<Self>) {
        if let Screen::Reader(id) = &screen {
            self.reader_scroll.set_offset(Default::default());
            self.reader_progress = self.data.document(id).map_or(0, |d| d.record.reading.progress);
        }
        self.nav = std::mem::take(&mut self.nav).go(screen);
        self.drawer_open = false;
        self.copied_style = None;
        cx.notify();
    }

    pub(in crate::ui::features::vault) fn back(&mut self, cx: &mut Context<Self>) {
        self.nav = std::mem::take(&mut self.nav).back();
        self.copied_style = None;
        cx.notify();
    }

    /// Opens the entry of document `id` (from links elsewhere in the app).
    pub fn open_entry(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.data.document(id).is_some() {
            self.go(Screen::Entry(id.to_string()), cx);
        }
    }

    /// Back to browsing, without history.
    pub fn show_browse(&mut self, cx: &mut Context<Self>) {
        self.nav = Navigation::home();
        cx.notify();
    }

    /// Documents the current screen shows.
    pub(in crate::ui::features::vault) fn open_documents(&self) -> Vec<String> {
        match &self.nav.current {
            Screen::Entry(id) | Screen::Reader(id) | Screen::Editor(Some(id)) => vec![id.clone()],
            Screen::Browse | Screen::SubjectIndex | Screen::Editor(None) => Vec::new(),
        }
    }

    pub(in crate::ui::features::vault) fn set_layout(&mut self, layout: BrowseLayout, cx: &mut Context<Self>) {
        self.layout = layout;
        cx.notify();
    }

    pub(in crate::ui::features::vault) fn toggle_facet_value(&mut self, facet: Facet, key: &str, cx: &mut Context<Self>) {
        let selection = self.query.selection.clone().toggled(facet, key);
        self.set_query(CatalogueQuery { selection, ..self.query.clone() }, cx);
    }

    /// Browses everything filed under one facet value (a chip on an entry or the index).
    pub(in crate::ui::features::vault) fn browse_only(&mut self, facet: Facet, key: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.search_input.update(cx, |input, cx| input.set_value("", window, cx));
        let mut query = CatalogueQuery::new(self.data.network_id.clone());
        query.sort = self.query.sort;
        query.descending = self.query.descending;
        query.selection = query.selection.only(facet, key);
        self.set_query(query, cx);
        self.go(Screen::Browse, cx);
    }

    pub(in crate::ui::features::vault) fn clear_filters(&mut self, cx: &mut Context<Self>) {
        self.set_query(CatalogueQuery { selection: Default::default(), ..self.query.clone() }, cx);
    }

    pub(in crate::ui::features::vault) fn sort_by(&mut self, key: SortKey, cx: &mut Context<Self>) {
        let query = self.query.clone().sorted_by(key);
        self.set_query(query, cx);
    }

    pub(in crate::ui::features::vault) fn toggle_subject(&mut self, id: &SubjectId, cx: &mut Context<Self>) {
        if !self.expanded_subjects.remove(id) {
            self.expanded_subjects.insert(id.clone());
        }
        cx.notify();
    }

    pub(in crate::ui::features::vault) fn toggle_facet_group(&mut self, facet: Facet, cx: &mut Context<Self>) {
        if !self.collapsed_facets.remove(&facet) {
            self.collapsed_facets.insert(facet);
        }
        cx.notify();
    }

    pub(in crate::ui::features::vault) fn set_drawer(&mut self, open: bool, cx: &mut Context<Self>) {
        self.drawer_open = open;
        cx.notify();
    }

    pub(in crate::ui::features::vault) fn copy_citation(&mut self, style: CitationStyle, text: String, cx: &mut Context<Self>) {
        cx.write_to_clipboard(ClipboardItem::new_string(text));
        self.copied_style = Some(style);
        cx.notify();
    }

    /// Reading progress from the reader's scroll position, stored in steps (never backwards).
    pub(in crate::ui::features::vault) fn track_reading(&mut self, cx: &mut Context<Self>) {
        let Screen::Reader(id) = &self.nav.current else { return };
        let (offset, max) = (self.reader_scroll.offset().y.as_f32(), self.reader_scroll.max_offset().y.as_f32());
        let fraction = if max <= 0.0 { 1.0 } else { (-offset / max).clamp(0.0, 1.0) };
        let progress = (fraction * f32::from(PROGRESS_COMPLETE)).round() as u8;
        let reached_end = progress == PROGRESS_COMPLETE && self.reader_progress < PROGRESS_COMPLETE;
        if progress < self.reader_progress.saturating_add(PROGRESS_STEP) && !reached_end {
            return;
        }
        self.reader_progress = progress;
        let current = self.data.document(id).map(|d| d.record.reading).unwrap_or_default();
        let reading = current.advanced_to(progress);
        if reading != current {
            cx.emit(VaultEvent::SetReading(id.clone(), reading));
        }
        cx.notify();
    }

    /// Marks document `id` read (or unread again).
    pub(in crate::ui::features::vault) fn mark_read(&mut self, id: String, read: bool, cx: &mut Context<Self>) {
        let reading = Reading::at(if read { PROGRESS_COMPLETE } else { 0 });
        self.reader_progress = reading.progress;
        cx.emit(VaultEvent::SetReading(id, reading));
    }
}
