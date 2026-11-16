//! Thread drafts: creating, adding, removing and splitting post editors, plus validation.

use super::super::consts::MODAL_COMPOSER_ROWS;
use super::super::modal::ComposerModalMode;
use super::{ComposerModalPanel, DraftSlot};
use crate::domain::{first_thread_problem, split_into_thread, ThreadProblem, MAX_POST_CHARS};
use gpui_kit::component::input::{InputEvent, TextareaState};
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalPanel {
    /// A new empty editor that re-renders the panel on edits and submits on Ctrl/Cmd+Enter.
    pub(super) fn new_draft_slot(&self, window: &mut Window, cx: &mut Context<Self>) -> DraftSlot {
        let (min_rows, max_rows) = MODAL_COMPOSER_ROWS;
        let placeholder = if self.drafts.is_empty() {
            t!("composer.placeholder")
        } else {
            t!("composer.thread_placeholder")
        }
        .to_string();
        let input = cx.new(|cx| {
            TextareaState::new(window, cx).placeholder(placeholder).auto_grow(min_rows, max_rows)
        });
        let subscription = cx.subscribe_in(&input, window, Self::on_draft_event);
        DraftSlot { input, _subscription: subscription }
    }

    fn on_draft_event(
        &mut self,
        _: &Entity<TextareaState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change => {
                self.error = None;
                cx.notify();
            }
            InputEvent::PressEnter { secondary: true, .. } => self.submit(window, cx),
            _ => {}
        }
    }

    /// Appends an empty post to the thread and focuses it.
    pub(super) fn add_draft(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let slot = self.new_draft_slot(window, cx);
        self.drafts.push(slot);
        self.focus_draft(self.drafts.len() - 1, window, cx);
        cx.notify();
    }

    /// Removes a post from the thread (the first post always stays).
    pub(super) fn remove_draft(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index == 0 || index >= self.drafts.len() {
            return;
        }
        self.drafts.remove(index);
        self.focus_draft(index - 1, window, cx);
        cx.notify();
    }

    /// Replaces an over-long post with limit-sized posts split on sentence/word boundaries.
    pub(super) fn split_draft(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(input) = self.drafts.get(index).map(|slot| slot.input.clone()) else {
            return;
        };
        let text = input.read(cx).value().to_string();
        let chunks = split_into_thread(&text, MAX_POST_CHARS);
        let Some((first, rest)) = chunks.split_first().filter(|(_, rest)| !rest.is_empty()) else {
            return;
        };
        input.update(cx, |state, cx| state.set_value(first.clone(), window, cx));
        for (offset, chunk) in rest.iter().enumerate() {
            let slot = self.new_draft_slot(window, cx);
            slot.input.update(cx, |state, cx| state.set_value(chunk.clone(), window, cx));
            self.drafts.insert(index + 1 + offset, slot);
        }
        self.focus_draft(index + rest.len(), window, cx);
        cx.notify();
    }

    pub(super) fn focus_draft(&self, index: usize, window: &mut Window, cx: &mut App) {
        if let Some(slot) = self.drafts.get(index) {
            slot.input.update(cx, |state, cx| state.focus(window, cx));
        }
    }

    /// Raw text of every post, in thread order.
    pub(super) fn draft_texts(&self, cx: &App) -> Vec<String> {
        self.drafts.iter().map(|slot| slot.input.read(cx).value().to_string()).collect()
    }

    /// Whether a link or an image is attached (they belong to the first post).
    fn has_media(&self, cx: &App) -> bool {
        !self.link_input.read(cx).value().trim().is_empty() || self.image_attachment(cx).is_some()
    }

    /// The first post that blocks publishing (quotes always need commentary text).
    pub(super) fn submit_problem(&self, cx: &App) -> Option<ThreadProblem> {
        let is_quote = matches!(self.mode, ComposerModalMode::Quote(_));
        first_thread_problem(&self.draft_texts(cx), self.has_media(cx) && !is_quote)
    }
}
