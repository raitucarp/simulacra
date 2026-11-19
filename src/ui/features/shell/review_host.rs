//! Review wiring of the shell (sb C1, anchored on posts in sb C1b): the deck (loaded once
//! from the vault), the Review page and the learn sheet, keeping the page's data in step
//! with the deck, the posts (an edited post is reviewed as edited, a deleted one retires
//! its item), the network, the review day and Settings › Learning, and the daily reminder.

use super::view::AppView;
use crate::domain::preferences::LearningPrefs;
use crate::domain::review::day::review_day;
use crate::domain::review::item::retire_orphans;
use crate::domain::review::{Anchor, DailyLimits, Deck, Policy};
use crate::ui::features::feed::LearningPosts;
use crate::ui::features::review::status::learn_status;
use std::rc::Rc;
use crate::domain::NetworkId;
use crate::state::AppState;
use crate::ui::features::review::{LearnSheet, ReviewData, ReviewPanel};
use jiff::civil::Date;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;

/// What the page's data was last built from: deck version, network, day, settings, post
/// revision and document count.
type Stamp = (u64, NetworkId, Date, LearningPrefs, u64, usize);

/// The deck, the Review page and the learn sheet.
pub struct ReviewHost {
    pub panel: Entity<ReviewPanel>,
    pub learn: Entity<LearnSheet>,
    pub deck: Deck,
    /// Items in today's queue (the sidebar count and the reminder).
    pub due: usize,
    /// Bumped on every change of the deck.
    pub(super) version: u64,
    stamp: Option<Stamp>,
    /// The review day the reminder was last shown on.
    reminded_on: Option<Date>,
    /// Goals, tutor sessions and palaces of the Learn tab (sb E2).
    pub learning: super::learning_host::LearningHost,
}

/// Today's review day in local time.
pub fn review_today() -> Date {
    review_day(&crate::time::local_now())
}

/// The limits and the scheduling policy of Settings › Learning.
pub fn learning_policy(prefs: &LearningPrefs) -> (DailyLimits, Policy) {
    (DailyLimits { reviews: prefs.daily_review_limit, new_cards: prefs.new_cards_per_day }, Policy::with_retention_percent(prefs.desired_retention))
}

impl ReviewHost {
    /// Loads the deck and creates the page and the sheet, returning their subscriptions.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Vec<Subscription>) {
        let deck = state.db.as_ref().and_then(|db| db.load_review_deck().map_err(|err| eprintln!("simulacra: review deck not loaded: {err}")).ok()).unwrap_or_default();
        let (limits, policy) = learning_policy(&state.preferences().learning);
        let data = ReviewData::empty(state.active_network_id.clone(), review_today(), limits, policy);
        let theme = state.theme_tokens.clone();
        let panel = cx.new(|cx| ReviewPanel::new(data, theme.clone(), window, cx));
        let learn = cx.new(|cx| LearnSheet::new(theme, window, cx));
        let subscriptions = vec![cx.subscribe_in(&panel, window, AppView::handle_review_event), cx.subscribe_in(&learn, window, AppView::handle_learn_event)];
        let learning = super::learning_host::LearningHost::load(state);
        (Self { panel, learn, deck, due: 0, version: 0, stamp: None, reminded_on: None, learning }, subscriptions)
    }
}

impl AppView {
    /// Render hook: retire items whose post is gone, fresh data when anything it shows
    /// changed, the theme, and the reminder.
    pub(super) fn sync_review(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let today = review_today();
        let posts = (self.state.index.revision(), self.state.documents.len());
        let stamp = (self.review.version, self.state.active_network_id.clone(), today, self.state.preferences().learning.clone(), posts.0, posts.1);
        if self.review.stamp.as_ref() != Some(&stamp) {
            self.retire_orphaned_items();
            let (_, network, day, prefs, revision, documents) = stamp;
            self.review.stamp = Some((self.review.version, network, day, prefs, revision, documents));
            let data = self.review_data(today);
            self.review.due = data.plan().queued();
            self.review.panel.update(cx, |panel, cx| panel.set_data(data, cx));
        }
        let theme = self.theme.clone();
        self.review.panel.update(cx, |panel, cx| panel.set_theme(&theme, cx));
        self.review.learn.update(cx, |sheet, cx| sheet.set_theme(&theme, cx));
        self.remind_review(today, window, cx);
        self.remind_weekly_review(today, window, cx);
    }

    /// Marks items whose post was deleted as retired, and saves them.
    fn retire_orphaned_items(&mut self) {
        let state = &self.state;
        let retired = retire_orphans(&self.review.deck.items, &|id| state.get_post_by_id(id).is_some());
        if retired.is_empty() {
            return;
        }
        if let Some(db) = &self.state.db {
            let _ = db.save_review_items(retired.clone());
        }
        self.review.deck = retired.into_iter().fold(std::mem::take(&mut self.review.deck), Deck::with_item);
        self.review.version += 1;
    }

    /// The review state of every post being learnt in the active network (post indicators).
    pub(super) fn learning_posts(&self) -> LearningPosts {
        let today = review_today();
        let network = &self.state.active_network_id;
        let on_posts = self.review.deck.items.iter().filter(|item| &item.network_id == network && !item.retired);
        let statuses = on_posts.filter_map(|item| match item.subject.anchor() {
            Some(Anchor::Post { id }) => Some((id.clone(), learn_status(item, today))),
            _ => None,
        });
        Rc::new(statuses.collect())
    }

    /// Marks the deck changed so the page is rebuilt on the next frame.
    pub(super) fn review_changed(&mut self, cx: &mut Context<Self>) {
        self.review.version += 1;
        cx.notify();
    }

    /// Once a day, at or after the reminder hour, says how many items wait (Settings › Learning).
    fn remind_review(&mut self, today: Date, window: &mut Window, cx: &mut Context<Self>) {
        let prefs = &self.state.preferences().learning;
        if !prefs.review_reminder || self.review.reminded_on == Some(today) || (crate::time::local_now().hour() as u32) < prefs.reminder_hour {
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
