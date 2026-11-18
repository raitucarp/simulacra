//! Journal wiring of the shell (sb E3): the private records (loaded once from the vault),
//! the Journal page, and the page's data kept in step with the records, the day, the user's
//! activity (posts, review sessions) and display preferences; plus the notice when a time
//! capsule's day has come. Nothing here feeds the stream, the index, Insights or agents.

use super::view::AppView;
use crate::domain::journal::capsule::notify::{announced, to_announce};
use crate::domain::journal::{journal_day, JournalBook};
use crate::ui::features::journal::{JournalData, JournalPanel};
use chrono::{Local, NaiveDate};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;

/// What the page's data was last built from: records version, day, post revision, review
/// answers, documents, the open thought, reduced motion.
type Stamp = (u64, NaiveDate, u64, usize, usize, Option<String>, bool);

/// The records and the Journal page.
pub struct JournalHost {
    pub panel: Entity<JournalPanel>,
    pub book: JournalBook,
    /// Bumped on every change of the records.
    pub(super) version: u64,
    stamp: Option<Stamp>,
    /// Focus mode hides the feed and the side panels.
    pub focus_mode: bool,
}

/// Today's journal day in local time.
pub fn journal_today() -> NaiveDate {
    journal_day(&Local::now())
}

impl JournalHost {
    pub(super) fn new(state: &crate::state::AppState, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Subscription) {
        let book = state.db.as_ref().and_then(|db| db.load_journal_book().map_err(|err| eprintln!("simulacra: journal not loaded: {err}")).ok()).unwrap_or_default();
        let theme = state.theme_tokens.clone();
        let panel = cx.new(|cx| JournalPanel::new(JournalData::empty(journal_today()), theme, window, cx));
        let subscription = cx.subscribe_in(&panel, window, AppView::handle_journal_event);
        (Self { panel, book, version: 0, stamp: None, focus_mode: false }, subscription)
    }
}

impl AppView {
    /// Render hook: the theme, fresh data when anything it shows changed, and the notice for
    /// capsules whose day has come.
    pub(super) fn sync_journal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let theme = self.theme.clone();
        self.journal.panel.update(cx, |panel, cx| panel.set_theme(&theme, cx));
        let today = journal_today();
        self.announce_capsules(today, window, cx);
        let open = self.feed.selected_thread_post_id.as_ref().map(|id| id.as_str().to_string());
        let reduced = self.state.preferences().appearance.reduced_motion;
        let stamp = (self.journal.version, today, self.state.index.revision(), self.review.deck.log.len(), self.state.documents.len(), open, reduced);
        if self.journal.stamp.as_ref() == Some(&stamp) {
            return;
        }
        self.journal.stamp = Some(stamp);
        let data = self.journal_data(today);
        self.journal.panel.update(cx, |panel, cx| panel.set_data(data, cx));
    }

    /// One notice for the capsules whose day has come; each is told once (marked and saved).
    fn announce_capsules(&mut self, today: NaiveDate, window: &mut Window, cx: &mut Context<Self>) {
        let ready: Vec<_> = to_announce(&self.journal.book.capsules, today).into_iter().map(announced).collect();
        if ready.is_empty() {
            return;
        }
        let text = crate::i18n::t_count("journal.capsule.notice", ready.len());
        window.push_notification(Notification::info(text), cx);
        for capsule in ready {
            self.save_journal_capsule(capsule);
        }
        self.journal_changed(cx);
    }

    /// Marks the records changed so the page is rebuilt on the next frame.
    pub(super) fn journal_changed(&mut self, cx: &mut Context<Self>) {
        self.journal.version += 1;
        cx.notify();
    }

    /// The panel, alone in the window, while focus mode is on (sb E3).
    pub(super) fn journal_focus_view(&self) -> Option<AnyElement> {
        let on = self.journal.focus_mode && self.state.active_tab == crate::state::NavigationTab::Journal && self.forge.is_none();
        on.then(|| self.journal.panel.clone().into_any_element())
    }
}
