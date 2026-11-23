//! Data ownership in the shell (sb F2): the one router of [`DataEvent`]s, the controls every
//! post card gets, the palette and settings commands, and the overlays.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::domain::data::saved::ItemRef;
use crate::ui::features::command_palette::CommandId;
use crate::ui::features::data::{closes_on_escape, render_export_sheet, render_history_sheet, render_restore_sheet, render_scheduled_sheet, render_snapshot_sheet, DataAction, DataEvent, DataSource};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use std::collections::HashMap;
use std::rc::Rc;

impl AppView {
    /// Routes one data event to its area.
    pub(crate) fn on_data_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        use DataEvent as E;
        let was_open = self.data_sheet_open();
        match event {
            E::EditPost(_) | E::SetEntryState(..) | E::ToggleTodo(..) | E::SetArchived(..) | E::SetAutoArchive(..) | E::SetAutoDelete(..) | E::ToggleBookmark(_) | E::AddToCollection(..) | E::RemoveFromCollection(..) | E::NewCollectionFor(_) => self.on_post_data_event(event, window, cx),
            E::OpenPostHistory(_) | E::OpenDocumentHistory(_) | E::SelectVersion(_) | E::CompareWith(_) | E::RestoreVersion(_) | E::EditFromVersion(_) | E::SwitchToVersion(_) | E::CloseHistory => self.on_history_event(event, window, cx),
            E::OpenExport | E::SetScope(_) | E::PickCollection(_) | E::PickPersona(_) | E::LastDays(_) | E::ToggleOriginals | E::ToggleJournal | E::RunExport | E::CloseExport => self.on_export_event(event, window, cx),
            E::OpenSnapshot | E::CreateSnapshot | E::CloseSnapshot => self.on_snapshot_event(event, window, cx),
            E::OpenRestore | E::PickSnapshotFile | E::VerifySnapshot | E::ConfirmRestore | E::CloseRestore => self.on_restore_event(event, window, cx),
            E::OpenReview | E::ReviewDecide(_) | E::ReviewSummary | E::CloseReview => self.on_review_event(event, window, cx), // S16
            E::OpenLegend | E::EditMeaning(_) | E::SaveMeaning | E::CancelMeaning | E::CloseLegend => self.on_legend_event(event, window, cx), // S9
            E::ReplyInMargin(_) | E::PickPassage(_) | E::SubmitMargin | E::CloseMargin => self.on_margin_event(event, window, cx), // N2
            E::SetGarden(id, maturity) => self.set_garden(&id, maturity, window, cx), // N3
            E::ShowDepth(..) | E::RenderDepth(..) => self.on_depth_event(event, window, cx), // N5
            E::Branch(event) => self.on_branch_event(event, window, cx), // N6
            E::Quote(event) => self.on_quote_event(event, window, cx), // N7
            E::SetAudience(id, audience) => self.set_audience(&id, audience, window, cx), // N8
            E::Question(event) => self.on_question_event(event, window, cx), // N9
            E::Series(event) => self.on_series_event(event, window, cx), // N10
            E::Voice(event) => self.on_voice_event(event, window, cx), E::Ink(event) => self.on_ink_event(event, window, cx), E::Correction(event) => self.on_correction_event(event, window, cx), E::Check(event) => self.on_check_event(event, window, cx), // N11–N14
            E::OpenScheduled | E::EditScheduled(_) | E::PublishScheduledNow(_) | E::CancelScheduled(_) | E::CloseScheduled => self.on_scheduled_event(event, window, cx),
            _ => self.on_saved_event(event, window, cx),
        }
        self.hand_keyboard_to_sheet(was_open, window, cx);
        cx.notify();
    }

    fn data_sheet_open(&self) -> bool {
        self.data.history.is_some() || self.data.export.is_some() || self.data.snapshot.is_some() || self.data.restore.is_some() || self.data.scheduled_open || self.data.legend_open || self.data.review.is_some() || self.data.margin.is_some() || self.data.ideas.any_open()
    }

    /// A sheet that just opened takes the keyboard (unless one of its fields already did), so
    /// Escape reaches it; a sheet that just closed gives it back to the shell.
    fn hand_keyboard_to_sheet(&mut self, was_open: bool, window: &mut Window, cx: &mut Context<Self>) {
        match (was_open, self.data_sheet_open()) {
            (false, true) if !self.data.inputs.any_focused(window, cx) => window.focus(&self.data.focus, cx),
            (true, false) => window.focus(&self.palette.shell_focus, cx),
            _ => {}
        }
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
            history_open: self.data.history.is_some(),
            depth: Rc::new(self.data.depth.clone()), // N5
            ideas: Rc::new(self.idea_source()), // N6–N10
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
            CommandId::ScheduledPosts => DataEvent::OpenScheduled,
            CommandId::SymbolKey => DataEvent::OpenLegend, // S9
            CommandId::MigrationReview => DataEvent::OpenReview, // S16
            CommandId::NewVoiceNote => DataEvent::Voice(crate::ui::features::data::VoiceEvent::Open), // N11
            CommandId::NewDrawing => DataEvent::Ink(crate::ui::features::data::InkEvent::Open), // N12
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
        let (on_event, theme) = (self.data_action(bind), &self.theme);
        let (sheet, close) = if let Some(sheet) = &self.data.history {
            (render_history_sheet(sheet, on_event.clone(), theme), DataEvent::CloseHistory)
        } else if let Some(dialog) = &self.data.margin {
            (crate::ui::features::data::render_margin_sheet(dialog, on_event.clone(), theme), DataEvent::CloseMargin) // N2
        } else if let Some(found) = self.idea_overlay(on_event.clone()) {
            found // N6–N10
        } else if let Some(dialog) = &self.data.export {
            (render_export_sheet(dialog, on_event.clone(), theme), DataEvent::CloseExport)
        } else if let Some(dialog) = &self.data.snapshot {
            (render_snapshot_sheet(dialog, on_event.clone(), theme), DataEvent::CloseSnapshot)
        } else if let Some(review) = self.review_view() {
            (crate::ui::features::data::render_review_sheet(&review, on_event.clone(), theme), DataEvent::CloseReview)
        } else if self.data.legend_open {
            (crate::ui::features::data::render_legend_sheet(&self.legend_view(), on_event.clone(), theme), DataEvent::CloseLegend)
        } else if self.data.scheduled_open {
            (render_scheduled_sheet(&self.scheduled_rows(), on_event.clone(), theme), DataEvent::CloseScheduled)
        } else {
            (render_restore_sheet(self.data.restore.as_ref()?, on_event.clone(), theme), DataEvent::CloseRestore)
        };
        Some(closes_on_escape(sheet, &self.data.focus, close, on_event).into_any_element())
    }
}
