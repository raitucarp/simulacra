//! Free compose state (T2): the first editor holds the whole text; each line becomes a post
//! when the thread is posted. Turning it on joins the thread's posts into lines; turning it
//! off splits the lines back into one editor per post.

use super::super::free_compose::{FreeComposeView, SplitLine};
use super::super::modal::ComposerModalMode;
use super::ComposerModalPanel;
use crate::domain::compose::{compose_lines, read_shorthand, split_line};
use crate::domain::MAX_POST_CHARS;
use std::rc::Rc;
use gpui_kit::*;

impl ComposerModalPanel {
    /// Switches free compose on or off (never while editing a post).
    pub(super) fn toggle_free(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(self.mode, ComposerModalMode::Edit(_)) {
            return;
        }
        let first = self.drafts.first().map(|slot| slot.input.read(cx).value().to_string()).unwrap_or_default();
        if self.free.on {
            self.free.on = false;
            let posts: Vec<String> = compose_lines(&first).into_iter().map(|line| line.text).collect();
            self.replace_draft(0, if posts.is_empty() { vec![String::new()] } else { posts }, window, cx);
            return;
        }
        let joined = self.drafts.iter().map(|slot| slot.input.read(cx).value().trim().to_string()).filter(|text| !text.is_empty()).collect::<Vec<_>>().join("\n");
        self.drafts.truncate(1);
        self.free.on = true;
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

    /// The draft each post of the free text replies to (T4); empty unless composing freely.
    pub(super) fn free_parents(&self, cx: &App) -> Vec<Option<usize>> {
        let text = self.drafts.first().filter(|_| self.free.on).map(|slot| slot.input.read(cx).value().to_string()).unwrap_or_default();
        compose_lines(&text).into_iter().map(|line| line.parent).collect()
    }

    /// The preview of the split while composing freely.
    pub(super) fn free_view(&self, cx: &mut Context<Self>) -> Option<FreeComposeView> {
        let text = self.drafts.first().map(|slot| slot.input.read(cx).value().to_string())?;
        let entity = cx.entity();
        let on_split: SplitLine = Rc::new(move |line, window, cx| entity.update(cx, |this, cx| this.split_free_line(line, window, cx)));
        let lines = if self.free.shorthand { read_shorthand(compose_lines(&text)) } else { compose_lines(&text) }; // T5
        self.free.on.then(|| FreeComposeView { lines, on_split })
    }

    /// Cuts the free compose line `line` into lines that each fit a post (T3).
    pub(super) fn split_free_line(&mut self, line: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(input) = self.drafts.first().map(|slot| slot.input.clone()) else { return };
        let text = split_line(input.read(cx).value().as_ref(), line, MAX_POST_CHARS);
        input.update(cx, |state, cx| state.set_value(text, window, cx));
        cx.notify();
    }
}

/// Whether the composer composes freely, and whether a line's leading markers become its
/// post's symbols (T5: symbol shorthand, from Settings › Writing).
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct FreeCompose {
    pub on: bool,
    pub shorthand: bool,
}

impl ComposerModalPanel {
    /// Follows the symbol shorthand setting in the preview of the split (T5).
    pub fn set_shorthand(&mut self, on: bool, cx: &mut Context<Self>) {
        if self.free.shorthand != on {
            self.free.shorthand = on;
            cx.notify();
        }
    }
}
