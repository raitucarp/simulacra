//! What a finished answer can become (sb C2): clipboard text, a reply under the thought (split
//! into a thread at the post length limit) or a vault document that links back to the thought.

use super::super::view::AppView;
use crate::domain::document::record::OriginKind;
use crate::domain::document::NewDocument;
use crate::domain::preferences::ToastKind;
use crate::domain::split_into_thread;
use crate::state::semantic::hits::shortened;
use crate::state::{ThreadAnchor, ThreadDraft};
use crate::ui::features::post_actions::kind_title;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;

/// Characters of the thought in a saved document's title.
const TITLE_SOURCE_CHARS: usize = 48;
/// Wikilink back to the thought at the end of a saved document.
const LINK_OPEN: &str = "[[";
const LINK_CLOSE: &str = "]]";
const PARAGRAPH_BREAK: &str = "\n\n";

impl AppView {
    /// The answer as plain text, with the sheet's post id and the action title.
    fn finished_answer(&self) -> Option<(crate::domain::PostId, String, String)> {
        let sheet = self.post_actions.sheet.as_ref()?;
        let output = sheet.output()?;
        Some((sheet.post_id.clone(), output.plain_text(), kind_title(sheet.request.kind)))
    }

    pub(super) fn copy_action_answer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((_, text, _)) = self.finished_answer() else { return };
        cx.write_to_clipboard(ClipboardItem::new_string(text));
        self.toast(ToastKind::Confirmation, Notification::success(t!("post_actions.copied").to_string()), window, cx);
    }

    /// Posts the answer as the active persona's reply to the thought.
    pub(super) fn reply_with_action_answer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((post_id, text, _)) = self.finished_answer() else { return };
        let limit = self.state.preferences().writing.post_length_limit as usize;
        let drafts: Vec<ThreadDraft> = split_into_thread(&text, limit).into_iter().map(ThreadDraft::text).collect();
        let anchor = ThreadAnchor { reply_to: Some(post_id), quote_of: None };
        match self.state.submit_thread(&drafts, &anchor) {
            Ok(ids) => {
                self.close_action_sheet(window, cx);
                if let Some(last) = ids.last().cloned() {
                    self.reveal_post(last, window, cx);
                }
                self.toast(ToastKind::Confirmation, Notification::success(t!("post_actions.replied").to_string()), window, cx);
            }
            Err(err) => window.push_notification(Notification::error(t!("post_actions.reply_failed", reason = err.to_string()).to_string()), cx),
        }
    }

    /// Saves the answer as a vault document titled after the action and the thought.
    pub(super) fn save_action_answer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((post_id, text, action)) = self.finished_answer() else { return };
        let source = self.post_actions.sheet.as_ref().map(|sheet| shortened(&sheet.source, TITLE_SOURCE_CHARS)).unwrap_or_default();
        let title = t!("post_actions.saved_title", action = action, source = source).to_string();
        let content = format!("{text}{PARAGRAPH_BREAK}{LINK_OPEN}{post_id}{LINK_CLOSE}");
        let persona = self.state.current_profile().map(|p| p.name.clone()).unwrap_or_default();
        let new = NewDocument::new(self.state.active_network_id.clone(), title.clone(), content).from_origin(OriginKind::Typed, "").added_by(persona);
        match self.state.add_new_document(new) {
            Ok(_) => {
                self.push_vault_data(cx);
                self.toast(ToastKind::Confirmation, Notification::success(t!("post_actions.saved", title = title).to_string()), window, cx);
            }
            Err(_) => window.push_notification(Notification::error(t!("post_actions.save_failed").to_string()), cx),
        }
    }
}
