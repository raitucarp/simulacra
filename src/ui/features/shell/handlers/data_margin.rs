//! N2: the margin sheet in the shell: opened from a post's menu with its passages, a passage
//! picked, the reply sent as a margin reply to exactly that passage. A locked post never
//! opens it (its sealed text has no passages to offer).

use super::super::view::AppView;
use crate::domain::post::anchor::{passages, span_text};
use crate::domain::post::{readable_line, HASHTAG_SIGIL};
use crate::domain::PostId;
use crate::ui::features::data::{tr, DataEvent, MarginDialog};
use gpui_kit::*;

/// A passage as the reader sees it, markup and post links left out; `None` when nothing but
/// tags (or links) is left, as nobody answers a line of hashtags.
fn readable_passage(raw: &str) -> Option<String> {
    let text = readable_line(raw);
    text.split_whitespace().any(|word| !word.starts_with(HASHTAG_SIGIL)).then_some(text)
}

impl AppView {
    /// Routes one margin sheet event.
    pub(in crate::ui::features::shell) fn on_margin_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            DataEvent::ReplyInMargin(id) => self.open_margin(id, window, cx),
            DataEvent::PickPassage(at) => {
                if let Some(dialog) = self.data.margin.as_mut() {
                    (dialog.picked, dialog.problem) = (Some(at), None);
                }
            }
            DataEvent::SubmitMargin => self.submit_margin(window, cx),
            _ => self.close_margin(window, cx),
        }
    }

    fn open_margin(&mut self, id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(&id).filter(|post| !post.is_locked()) else { return };
        let found: Vec<_> = passages(&post.content).into_iter().filter_map(|range| readable_passage(&span_text(&post.content, range.clone())).map(|text| (range, text))).collect();
        if found.is_empty() {
            return;
        }
        let input = self.data.inputs.margin_reply.clone();
        input.update(cx, |field, cx| field.set_value("", window, cx));
        self.data.margin = Some(MarginDialog::new(id, found, input.clone()));
        input.update(cx, |field, cx| field.focus(window, cx));
    }

    fn close_margin(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.data.inputs.margin_reply.update(cx, |field, cx| field.set_value("", window, cx));
        self.data.margin = None;
    }

    /// Sends the reply to the picked passage, or says why it cannot go yet.
    fn submit_margin(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.margin.as_ref() else { return };
        let (post, text) = (dialog.post_id.clone(), dialog.reply.read(cx).value().trim().to_string());
        let outcome = match dialog.picked_range() {
            None => Err(tr("post.margin.pick_first")),
            Some(_) if text.is_empty() => Err(tr("post.margin.write_first")),
            Some(range) => self.state.submit_margin_reply(&post, range, &text).map_err(|_| tr("post.margin.failed")),
        };
        match outcome {
            Ok(_) => {
                self.close_margin(window, cx);
                self.feed.selected_thread_post_id = Some(post);
                self.notify_data(tr("post.margin.sent"), window, cx);
            }
            Err(problem) => {
                if let Some(dialog) = self.data.margin.as_mut() {
                    dialog.problem = Some(problem);
                }
            }
        }
    }
}
