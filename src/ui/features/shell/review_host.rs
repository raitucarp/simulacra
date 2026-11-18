//! Review wiring of the shell (sb C1): the deck (loaded once from the vault), the Review page
//! and the card maker, keeping the page's data in step with the deck, the network, the review
//! day and Settings › Learning, and the daily review reminder.

use super::view::AppView;
use crate::domain::preferences::LearningPrefs;
use crate::domain::review::day::review_day;
use crate::domain::review::{DailyLimits, Deck, Policy};
use crate::domain::NetworkId;
use crate::state::AppState;
use crate::ui::features::review::{FlashcardMaker, ReviewData, ReviewPanel};
use chrono::{Local, NaiveDate, Timelike};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;

/// What the page's data was last built from.
type Stamp = (u64, NetworkId, NaiveDate, LearningPrefs);

/// The deck, the Review page and the card maker.
pub struct ReviewHost {
    pub panel: Entity<ReviewPanel>,
    pub maker: Entity<FlashcardMaker>,
    pub deck: Deck,
    /// Cards in today's queue (the sidebar count and the reminder).
    pub due: usize,
    /// Bumped on every change of the deck.
    pub(super) version: u64,
    stamp: Option<Stamp>,
    /// The review day the reminder was last shown on.
    reminded_on: Option<NaiveDate>,
}

/// Today's review day in local time.
pub fn review_today() -> NaiveDate {
    review_day(&Local::now())
}

/// The limits and the scheduling policy of Settings › Learning.
pub fn learning_policy(prefs: &LearningPrefs) -> (DailyLimits, Policy) {
    (DailyLimits { reviews: prefs.daily_review_limit, new_cards: prefs.new_cards_per_day }, Policy::with_retention_percent(prefs.desired_retention))
}

impl ReviewHost {
    /// Loads the deck and creates the page and the maker, returning their subscriptions.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Vec<Subscription>) {
        let deck = state.db.as_ref().and_then(|db| db.load_review_deck().map_err(|err| eprintln!("simulacra: review deck not loaded: {err}")).ok()).unwrap_or_default();
        let (limits, policy) = learning_policy(&state.preferences().learning);
        let data = ReviewData::empty(state.active_network_id.clone(), review_today(), limits, policy);
        let theme = state.theme_tokens.clone();
        let panel = cx.new(|cx| ReviewPanel::new(data, theme.clone(), window, cx));
        let maker = cx.new(|cx| FlashcardMaker::new(theme, window, cx));
        let subscriptions = vec![cx.subscribe_in(&panel, window, AppView::handle_review_event), cx.subscribe_in(&maker, window, AppView::handle_maker_event)];
        (Self { panel, maker, deck, due: 0, version: 0, stamp: None, reminded_on: None }, subscriptions)
    }
}

impl AppView {
    /// Render hook: fresh data when the deck, network, day or settings changed, the theme,
    /// and the reminder.
    pub(super) fn sync_review(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let today = review_today();
        let stamp = (self.review.version, self.state.active_network_id.clone(), today, self.state.preferences().learning.clone());
        if self.review.stamp.as_ref() != Some(&stamp) {
            self.review.stamp = Some(stamp);
            let data = self.review_data(today);
            self.review.due = data.plan().queued();
            self.review.panel.update(cx, |panel, cx| panel.set_data(data, cx));
        }
        let theme = self.theme.clone();
        self.review.panel.update(cx, |panel, cx| panel.set_theme(&theme, cx));
        self.review.maker.update(cx, |maker, cx| maker.set_theme(&theme, cx));
        self.remind_review(today, window, cx);
    }

    /// Marks the deck changed so the page is rebuilt on the next frame.
    pub(super) fn review_changed(&mut self, cx: &mut Context<Self>) {
        self.review.version += 1;
        cx.notify();
    }

    /// Once a day, at or after the reminder hour, says how many cards wait (Settings › Learning).
    fn remind_review(&mut self, today: NaiveDate, window: &mut Window, cx: &mut Context<Self>) {
        let prefs = &self.state.preferences().learning;
        if !prefs.review_reminder || self.review.reminded_on == Some(today) || Local::now().hour() < prefs.reminder_hour {
            return;
        }
        self.review.reminded_on = Some(today);
        let waiting = self.review.due;
        if waiting > 0 {
            let text = crate::i18n::t_count("review.reminder", waiting);
            window.push_notification(Notification::info(text), cx);
        }
    }
}
