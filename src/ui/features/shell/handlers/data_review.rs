//! The monthly migration review in the shell (S16): the sheet walks the user's open tasks,
//! oldest first, and each choice moves the task (a patch version, like the post menu's
//! moves). When it is finished the tally can go out as a summary post. Once a month, while
//! open tasks wait, a quiet notice says the review is due.

use super::super::view::AppView;
use crate::domain::symbols::{needs_review, review_due, MigrationReview, ReviewChoice};
use crate::domain::PostId;
use crate::storage::SettingKey;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tally_text, tr, DataEvent, ReviewView};
use gpui_kit::*;
use jiff::Timestamp;

impl AppView {
    pub(in crate::ui::features::shell) fn on_review_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            DataEvent::OpenReview => self.data.review = Some(MigrationReview::new(self.open_tasks())),
            DataEvent::ReviewDecide(choice) => self.decide_review(choice, window, cx),
            DataEvent::ReviewSummary => self.post_review_summary(window, cx),
            _ => self.data.review = None,
        }
    }

    /// The user's open tasks, oldest first.
    fn open_tasks(&self) -> Vec<PostId> {
        let mut tasks: Vec<&crate::domain::Post> = self.state.posts().iter().filter(|post| needs_review(&post.extras.symbols) && self.state.author_of(post).is_human_persona()).collect();
        tasks.sort_by_key(|post| post.created_at);
        tasks.into_iter().map(|post| post.id.clone()).collect()
    }

    fn decide_review(&mut self, choice: ReviewChoice, window: &mut Window, cx: &mut Context<Self>) {
        let Some(task) = self.data.review.as_ref().and_then(|review| review.current().cloned()) else { return };
        if let Some(to) = choice.state() {
            match self.state.set_entry_state(&task, to, Timestamp::now()) {
                Ok(_) => *self.data.revision_counts.entry(task.as_str().to_string()).or_default() += 1,
                Err(err) => self.notify_data(publish_error_message(&err), window, cx),
            }
        }
        if let Some(review) = self.data.review.as_mut() {
            review.decide(choice);
            if review.is_done() {
                self.mark_reviewed();
            }
        }
    }

    fn post_review_summary(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(review) = self.data.review.take() else { return };
        let text = tally_text("symbols.review.summary_post", ReviewChoice::ALL.map(|choice| review.count(choice)));
        match self.state.submit_post(&text) {
            Ok(id) => self.reveal_post(id, window, cx),
            Err(err) => self.notify_data(publish_error_message(&err), window, cx),
        }
    }

    /// What the sheet shows of the review under way.
    pub(in crate::ui::features::shell) fn review_view(&self) -> Option<ReviewView> {
        let review = self.data.review.as_ref()?;
        let current = review.current().and_then(|id| self.state.get_post(id)).map(|post| post.content.clone());
        let tally = ReviewChoice::ALL.map(|choice| review.count(choice));
        Some(ReviewView { current, position: review.decided.len() + 1, total: review.queue.len(), tally })
    }

    fn mark_reviewed(&self) {
        let today = crate::time::local_now().date().to_string();
        if let Some(db) = self.state.db.as_ref() {
            let _ = db.set_setting(SettingKey::MigrationReviewed, &today);
        }
    }

    /// Says the monthly review is due while open tasks wait and none was done this month.
    pub(in crate::ui::features::shell) fn nudge_migration_review(&self, window: &mut Window, cx: &mut Context<Self>) {
        let last = self.state.db.as_ref().and_then(|db| db.get_setting(SettingKey::MigrationReviewed).ok().flatten()).and_then(|raw| raw.parse::<jiff::civil::Date>().ok());
        if review_due(last, crate::time::local_now().date()) && !self.open_tasks().is_empty() {
            self.notify_data(tr("symbols.review.due"), window, cx);
        }
    }
}
