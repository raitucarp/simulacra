//! A persona's feeling also logs a journal mood check-in (M6), when Settings › Reading &
//! Writing › "Log my feelings in the journal" is on (off by default). The check-in goes to the
//! private journal only, like one made on the Journal page.

use super::super::view::AppView;
use super::journal_events::store;
use crate::domain::journal::mood::from_feeling::checkin_from_feeling;
use crate::domain::journal::records::upserted;
use crate::domain::journal::{journal_day, RecordId};
use crate::domain::post::PostKind;
use gpui_kit::*;

impl AppView {
    /// Logs `feeling`, just posted by the current profile, as a check-in when it is a persona
    /// and the setting is on.
    pub(super) fn log_feeling_checkin(&mut self, feeling: &PostKind, cx: &mut Context<Self>) {
        let persona = self.state.current_profile().is_some_and(|profile| profile.is_human_persona());
        if !persona || !self.state.preferences().writing.feeling_checkins {
            return;
        }
        let Some(draft) = checkin_from_feeling(feeling) else { return };
        let now = crate::time::local_now();
        let Some(checkin) = draft.into_checkin(RecordId::random(), now.clone(), journal_day(&now)) else { return };
        let checkin = store(self.state.db.as_ref(), checkin);
        self.journal.book.checkins = upserted(&self.journal.book.checkins, checkin);
        self.journal_changed(cx);
    }
}
