//! Data ownership in the shell (sb F2): the one router of [`DataEvent`]s, the controls every
//! post card gets, the palette and settings commands, and the overlays.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::domain::data::saved::ItemRef;
use crate::ui::features::command_palette::CommandId;
use crate::ui::features::data::{render_export_sheet, render_history_sheet, render_restore_sheet, render_snapshot_sheet, DataAction, DataEvent, DataSource};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use std::collections::HashMap;
use std::rc::Rc;

impl AppView {
    /// Routes one data event to its area.
    pub(crate) fn on_data_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        use DataEvent as E;
        match event {
            E::EditPost(_) | E::ToggleBookmark(_) | E::AddToCollection(..) | E::RemoveFromCollection(..) | E::NewCollectionFor(_) => self.on_post_data_event(event, window, cx),
            E::OpenPostHistory(_) | E::OpenDocumentHistory(_) | E::SelectVersion(_) | E::RestoreVersion(_) | E::CloseHistory => self.on_history_event(event, window, cx),
            E::OpenExport | E::SetScope(_) | E::PickCollection(_) | E::PickPersona(_) | E::LastDays(_) | E::ToggleOriginals | E::ToggleJournal | E::RunExport | E::CloseExport => self.on_export_event(event, window, cx),
            E::OpenSnapshot | E::CreateSnapshot | E::CloseSnapshot => self.on_snapshot_event(event, window, cx),
            E::OpenRestore | E::PickSnapshotFile | E::VerifySnapshot | E::ConfirmRestore | E::CloseRestore => self.on_restore_event(event, window, cx),
            _ => self.on_saved_event(event, window, cx),
        }
        cx.notify();
    }

    /// A data callback for the views.
    pub(in crate::ui::features::shell) fn data_action(&self, bind: &Binder) -> DataAction {
        Rc::new(bind.with(Self::on_data_event))
    }

    /// What the feed needs for every card's Edit, History, Bookmark and collection items.
    pub(in crate::ui::features::shell) fn data_source(&self, bind: &Binder) -> DataSource {
        let mut memberships: HashMap<String, Vec<crate::domain::document::CollectionId>> = HashMap::new();
        for entry in &self.data.shelf.entries {
            if let ItemRef::Post(id) = &entry.item {
                memberships.entry(id.as_str().to_string()).or_default().push(entry.collection.clone());
            }
        }
        DataSource {
            revisions: Rc::new(self.data.revision_counts.clone()),
            bookmarks: Rc::new(self.data.shelf.bookmarks.iter().map(|b| b.post.as_str().to_string()).collect()),
            collections: Rc::new(self.network_collections()),
            memberships: Rc::new(memberships),
            on_event: self.data_action(bind),
        }
    }

    /// The active network's collections, by name.
    pub(in crate::ui::features::shell) fn network_collections(&self) -> Vec<(crate::domain::document::CollectionId, String)> {
        let mut collections: Vec<_> = self.state.library.collections.iter().filter(|c| c.network_id == self.state.active_network_id).map(|c| (c.id.clone(), c.name.clone())).collect();
        collections.sort_by_key(|(_, name)| name.to_lowercase());
        collections
    }

    /// Palette commands of data ownership.
    pub(crate) fn run_data_command(&mut self, id: CommandId, window: &mut Window, cx: &mut Context<Self>) {
        let event = match id {
            CommandId::OpenCollection => DataEvent::ShowSaved,
            CommandId::BookmarkPost => match self.feed.selected_thread_post_id.clone() {
                Some(post) => DataEvent::ToggleBookmark(post),
                None => return self.notify_data(crate::ui::features::data::tr("data.saved.open_a_post"), window, cx),
            },
            CommandId::ExportBundle => DataEvent::OpenExport,
            CommandId::EncryptedSnapshot => DataEvent::OpenSnapshot,
            _ => DataEvent::OpenRestore,
        };
        self.on_data_event(event, window, cx);
    }

    /// Re-renders when a live data field changes (strength meter, export preview).
    pub(in crate::ui::features::shell) fn on_data_input_change(&mut self, _: &Entity<InputState>, event: &InputEvent, _: &mut Window, cx: &mut Context<Self>) {
        if matches!(event, InputEvent::Change) {
            self.refresh_snapshot_checks(cx);
            self.refresh_export_preview(cx);
            cx.notify();
        }
    }

    /// An info toast.
    pub(in crate::ui::features::shell) fn notify_data(&self, text: String, window: &mut Window, cx: &mut Context<Self>) {
        use gpui_kit::component::WindowExt;
        window.push_notification(gpui_kit::component::notification::Notification::info(text), cx);
    }

    /// The open data sheet, if any.
    pub(in crate::ui::features::shell) fn data_overlay(&self, bind: &Binder) -> Option<AnyElement> {
        let on_event = self.data_action(bind);
        let theme = &self.theme;
        if let Some(sheet) = &self.data.history {
            return Some(render_history_sheet(sheet, on_event, theme).into_any_element());
        }
        if let Some(dialog) = &self.data.export {
            return Some(render_export_sheet(dialog, on_event, theme).into_any_element());
        }
        if let Some(dialog) = &self.data.snapshot {
            return Some(render_snapshot_sheet(dialog, on_event, theme).into_any_element());
        }
        self.data.restore.as_ref().map(|dialog| render_restore_sheet(dialog, on_event, theme).into_any_element())
    }
}
