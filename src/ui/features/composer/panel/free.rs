//! Free compose state (T2): the first editor holds the whole text; each line becomes a post
//! when the thread is posted. Turning it on joins the thread's posts into lines; turning it
//! off splits the lines back into one editor per post.

use super::super::free_compose::FreeComposeView;
use super::super::modal::ComposerModalMode;
use super::ComposerModalPanel;
use crate::domain::compose::compose_lines;
use gpui_kit::*;

impl ComposerModalPanel {
    /// Switches free compose on or off (never while editing a post).
    pub(super) fn toggle_free(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.mode, ComposerModalMode::Edit(_)) {
            return;
        }
        let first = self.drafts.first().map(|slot| slot.input.read(cx).value().to_string()).unwrap_or_default();
        if self.free {
            self.free = false;
            let posts: Vec<String> = compose_lines(&first).into_iter().map(|line| line.text).collect();
            self.replace_draft(0, if posts.is_empty() { vec![String::new()] } else { posts }, window, cx);
            return;
        }
        let joined = self.drafts.iter().map(|slot| slot.input.read(cx).value().trim().to_string()).filter(|text| !text.is_empty()).collect::<Vec<_>>().join("\n");
        self.drafts.truncate(1);
        self.free = true;
        if let Some(input) = self.drafts.first().map(|slot| slot.input.clone()) {
            input.update(cx, |state, cx| state.set_value(joined, window, cx));
        }
        self.focus_draft(0, window, cx);
        cx.notify();
    }

    /// The posts the free text makes, in thread order.
    pub(super) fn free_texts(&self, cx: &App) -> Vec<String> {
        let text = self.drafts.first().map(|slot| slot.input.read(cx).value().to_string()).unwrap_or_default();
        compose_lines(&text).into_iter().map(|line| line.text).collect()
    }

    /// The preview of the split while composing freely.
    pub(super) fn free_view(&self, cx: &App) -> Option<FreeComposeView> {
        let text = self.drafts.first().map(|slot| slot.input.read(cx).value().to_string())?;
        self.free.then(|| FreeComposeView { lines: compose_lines(&text) })
    }
}
