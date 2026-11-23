//! N7: the quote sheet in the shell: opened from a vault document with its passages, narrowed
//! by the search as it is typed, a passage picked and posted as a pinned quote; and re-pinning
//! a drifted quote from its post's menu.

use super::super::view::AppView;
use crate::domain::post::anchor::{passages, span_text};
use crate::domain::post::pinned::MAX_QUOTE_CHARS;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, QuoteDialog, QuoteEvent};
use gpui_kit::*;

impl AppView {
    /// Routes one quote event.
    pub(in crate::ui::features::shell) fn on_quote_event(&mut self, event: QuoteEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            QuoteEvent::Open(doc) => self.open_quote(&doc, window, cx),
            QuoteEvent::Pick(at) => {
                if let Some(dialog) = self.data.ideas.quote.as_mut() {
                    (dialog.picked, dialog.problem) = (Some(at), None);
                }
            }
            QuoteEvent::Submit => self.submit_quote(window, cx),
            QuoteEvent::Close => self.close_quote(window, cx),
            QuoteEvent::Repin(id) => self.repin_quote(&id, window, cx),
        }
    }

    fn open_quote(&mut self, doc_id: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(doc) = self.state.documents.iter().find(|doc| doc.id == doc_id) else { return };
        let found: Vec<_> = passages(&doc.content).into_iter().filter(|range| range.len() <= MAX_QUOTE_CHARS).map(|range| (range.clone(), span_text(&doc.content, range))).collect();
        let (find, comment) = (self.data.inputs.ideas.quote_find.clone(), self.data.inputs.ideas.quote_comment.clone());
        for input in [&find, &comment] {
            input.update(cx, |field, cx| field.set_value("", window, cx));
        }
        self.data.ideas.quote = Some(QuoteDialog { doc_id: doc.id.clone(), title: doc.title.clone(), passages: found, query: String::new(), picked: None, find: find.clone(), comment, problem: None });
        find.update(cx, |field, cx| field.focus(window, cx));
    }

    fn close_quote(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for input in [&self.data.inputs.ideas.quote_find, &self.data.inputs.ideas.quote_comment] {
            input.update(cx, |field, cx| field.set_value("", window, cx));
        }
        self.data.ideas.quote = None;
    }

    /// Keeps the sheet's search in step with its field.
    pub(in crate::ui::features::shell) fn refresh_quote_filter(&mut self, cx: &App) {
        if let Some(dialog) = self.data.ideas.quote.as_mut() {
            dialog.query = dialog.find.read(cx).value().trim().to_lowercase();
        }
    }

    fn submit_quote(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.quote.as_ref() else { return };
        let (doc, text) = (dialog.doc_id.clone(), dialog.comment.read(cx).value().trim().to_string());
        let outcome = match dialog.picked_range() {
            None => Err(tr("posts.pinned.pick_first")),
            Some(_) if text.is_empty() => Err(tr("posts.pinned.write_first")),
            Some(range) => self.state.pin_quote(&doc, range, &text).map_err(|err| publish_error_message(&err)),
        };
        match outcome {
            Ok(id) => {
                self.close_quote(window, cx);
                self.feed.selected_thread_post_id = Some(id);
                self.notify_data(tr("posts.pinned.sent"), window, cx);
            }
            Err(problem) => {
                if let Some(dialog) = self.data.ideas.quote.as_mut() {
                    dialog.problem = Some(problem);
                }
            }
        }
    }

    fn repin_quote(&mut self, id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let text = match self.state.repin_quote(id) {
            Ok(true) => tr("posts.pinned.repinned"),
            Ok(false) => return,
            Err(_) => tr("posts.pinned.repin_failed"),
        };
        self.notify_data(text, window, cx);
    }
}
