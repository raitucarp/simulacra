//! Post and thread handlers: inline composer, post interactions and the feed window.

use super::super::view::AppView;
use crate::domain::PostId;
use crate::ui::features::sidebar_left::NavRef;
use crate::ui::features::composer::{publish_error_message, ComposerModalMode};
use crate::ui::features::feed::FEED_WINDOW_INCREMENT;
use gpui_kit::*;

impl AppView {
    /// Publishes the inline composer's thought and brings it into view; an `Err` carries the
    /// message the composer shows under its editor.
    pub fn submit_post_content(
        &mut self,
        content: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<(), String> {
        let post_id = self.state.submit_post(content).map_err(|err| publish_error_message(&err))?;
        self.reveal_post(post_id, window, cx);
        Ok(())
    }

    /// Appends helper syntax such as `[[` or `#` to the composer and focuses it.
    pub fn insert_composer_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.feed.composer_input.update(cx, |state, cx| {
            let mut current = state.value().to_string();
            current.push_str(text);
            state.set_value(&current, window, cx);
            state.focus(window, cx);
        });
        self.state.draft_text = self.feed.composer_input.read(cx).value().to_string();
        cx.notify();
    }

    /// Focuses `post_id` (the original of an instant repost) in the thread inspector, collapsing
    /// the replies of a newly focused post.
    pub(crate) fn select_post(&mut self, post_id: PostId, _: &mut Window, cx: &mut Context<Self>) {
        let post_id = self.state.action_target(&post_id);
        if self.feed.selected_thread_post_id.as_ref() != Some(&post_id) {
            self.feed.inspector_expanded = false;
        }
        self.note_recent(NavRef::Thread(post_id.clone()));
        self.feed.selected_thread_post_id = Some(post_id);
        cx.notify();
    }

    pub(crate) fn like_post(&mut self, post_id: PostId, _: &mut Window, cx: &mut Context<Self>) {
        self.state.toggle_like(&post_id);
        cx.notify();
    }

    pub(crate) fn repost_post(&mut self, post_id: PostId, _: &mut Window, cx: &mut Context<Self>) {
        let _ = self.state.repost(&post_id);
        cx.notify();
    }

    /// Opens the composer modal replying to `post_id` (the original of an instant repost).
    pub(crate) fn reply_to_post(&mut self, post_id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(target) = self.state.get_post(&self.state.action_target(&post_id)).map(|p| self.state.authored(p)) {
            self.open_composer_modal(ComposerModalMode::Reply(Box::new(target)), window, cx);
        }
    }

    /// Opens the composer modal quoting `post_id` (the original of an instant repost).
    pub(crate) fn quote_post(&mut self, post_id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(target) = self.state.get_post(&self.state.action_target(&post_id)).map(|p| self.state.authored(p)) {
            self.open_composer_modal(ComposerModalMode::Quote(Box::new(target)), window, cx);
        }
    }

    /// Extends the visible timeline window.
    pub(crate) fn load_more_posts(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.feed.feed_visible_count += FEED_WINDOW_INCREMENT;
        cx.notify();
    }

    /// Cycles the active user persona from the feed.
    pub(crate) fn cycle_profile(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let _ = self.state.cycle_active_persona();
        cx.notify();
    }
}
