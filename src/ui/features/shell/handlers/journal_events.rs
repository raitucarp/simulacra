//! The Journal page's events (sb E3): records are replaced or removed in the book and saved
//! to the vault; "Turn into a post" opens the composer with the text (the user picks the
//! persona as usual); a distraction opens quick capture; focus mode hides the other columns;
//! links open their post, document or the Review page.

use super::super::view::AppView;
use crate::domain::journal::focus::FocusLink;
use crate::domain::journal::records::{removed, upserted};
use crate::domain::journal::{FocusSession, JournalEntry, MoodCheckin, RecordId, TimeCapsule};
use crate::domain::PostId;
use crate::state::NavigationTab;
use crate::storage::db::StoredRecord;
use crate::ui::features::composer::ComposerModalMode;
use crate::ui::features::journal::{JournalEvent, JournalPanel};
use gpui_kit::*;

impl AppView {
    pub(in crate::ui::features::shell) fn handle_journal_event(&mut self, _: &Entity<JournalPanel>, event: &JournalEvent, window: &mut Window, cx: &mut Context<Self>) {
        let book = &mut self.journal.book;
        match event.clone() {
            JournalEvent::SaveEntry(entry) => book.entries = upserted(&book.entries, store(self.state.db.as_ref(), entry)),
            JournalEvent::DeleteEntry(id) => book.entries = removed(&book.entries, &forget::<JournalEntry>(self.state.db.as_ref(), id)),
            JournalEvent::SaveCheckin(checkin) => book.checkins = upserted(&book.checkins, store(self.state.db.as_ref(), checkin)),
            JournalEvent::DeleteCheckin(id) => book.checkins = removed(&book.checkins, &forget::<MoodCheckin>(self.state.db.as_ref(), id)),
            JournalEvent::SaveSession(session) => book.sessions = upserted(&book.sessions, store::<FocusSession>(self.state.db.as_ref(), session)),
            JournalEvent::SaveCapsule(capsule) => self.save_journal_capsule(capsule),
            JournalEvent::DeleteCapsule(id) => book.capsules = removed(&book.capsules, &forget::<TimeCapsule>(self.state.db.as_ref(), id)),
            JournalEvent::TurnIntoPost(text) => return self.open_composer_with(text, window, cx),
            JournalEvent::CaptureDistraction => return self.open_quick_capture(window, cx),
            JournalEvent::FocusMode(on) => self.journal.focus_mode = on,
            JournalEvent::OpenWikilink(target) => return self.open_wikilink(target, window, cx),
            JournalEvent::OpenLink(link) => return self.open_focus_link(link, window, cx),
        }
        self.journal_changed(cx);
    }

    /// Replaces or adds `capsule` and saves it.
    pub(in crate::ui::features::shell) fn save_journal_capsule(&mut self, capsule: TimeCapsule) {
        let capsule = store(self.state.db.as_ref(), capsule);
        self.journal.book.capsules = upserted(&self.journal.book.capsules, capsule);
    }

    /// Opens the composer on a new thread holding `text`.
    fn open_composer_with(&mut self, text: String, window: &mut Window, cx: &mut Context<Self>) {
        self.composer_modal.update(cx, |panel, cx| panel.open_with_text(ComposerModalMode::NewThread, text, window, cx));
        cx.notify();
    }

    fn open_focus_link(&mut self, link: FocusLink, window: &mut Window, cx: &mut Context<Self>) {
        match link {
            FocusLink::Post(id) => self.reveal_post(PostId::from(id), window, cx),
            FocusLink::Document(id) => self.open_vault_document(&id, window, cx),
            FocusLink::Review => self.select_nav(NavigationTab::Review, window, cx),
            FocusLink::Goal(_) => {}
        }
    }
}

/// Queues saving `record` and hands it back.
pub(super) fn store<T: StoredRecord + Clone>(vault: Option<&crate::storage::Vault>, record: T) -> T {
    if let Some(vault) = vault {
        if let Err(err) = vault.save_journal_record(record.clone()) {
            eprintln!("simulacra: journal record not saved: {err}");
        }
    }
    record
}

/// Queues deleting the record `id` of kind `T` and hands the id back.
fn forget<T: StoredRecord>(vault: Option<&crate::storage::Vault>, id: RecordId) -> RecordId {
    if let Some(vault) = vault {
        if let Err(err) = vault.delete_journal_record::<T>(&id) {
            eprintln!("simulacra: journal record not deleted: {err}");
        }
    }
    id
}
